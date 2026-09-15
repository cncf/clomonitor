use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use anyhow::{Result, format_err};
use clomonitor_core::tools::{LocalTool, Tool};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Shared runner state.
#[derive(Debug)]
pub(crate) struct State {
    /// Runner configuration.
    config: RunnerConfig,
    /// Run slots (one permit per concurrent run allowed).
    slots: Arc<Semaphore>,
    /// Tools enabled, indexed by tool.
    tools: HashMap<Tool, ToolEntry>,
    /// Number of requests waiting for a run slot.
    waiting: AtomicUsize,
}

impl State {
    /// Create a new state instance.
    pub(crate) fn new(entries: Vec<ToolEntry>, config: RunnerConfig) -> Result<Self> {
        // Validate configuration
        if config.max_concurrent_runs == 0 {
            return Err(format_err!("maxConcurrentRuns must be greater than zero"));
        }

        // Setup run slots and tools index
        let slots = Arc::new(Semaphore::new(config.max_concurrent_runs));
        Ok(Self {
            config,
            slots,
            tools: entries.into_iter().map(|e| (e.local.tool, e)).collect(),
            waiting: AtomicUsize::new(0),
        })
    }

    /// Wait for a run slot.
    pub(crate) async fn acquire_slot(&self) -> OwnedSemaphorePermit {
        self.slots
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore never to be closed")
    }

    /// Runner configuration.
    pub(crate) fn config(&self) -> &RunnerConfig {
        &self.config
    }

    /// Try to enter the queue of requests waiting for a run slot.
    pub(crate) fn enqueue(self: &Arc<Self>) -> Admission {
        let mut waiting = self.waiting.load(Ordering::SeqCst);
        loop {
            // Reject the request when the queue is full
            if waiting >= self.config.max_queue {
                return Admission::QueueFull;
            }

            // Claim a queue position atomically, retrying on contention
            match self.waiting.compare_exchange(
                waiting,
                waiting + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    return Admission::Queued(QueueGuard {
                        state: self.clone(),
                    });
                }
                Err(current) => waiting = current,
            }
        }
    }

    /// Number of runs currently in flight.
    pub(crate) fn running(&self) -> usize {
        self.config
            .max_concurrent_runs
            .saturating_sub(self.slots.available_permits())
    }

    /// Get the entry of the tool provided, if enabled.
    pub(crate) fn tool(&self, tool: Tool) -> Option<&ToolEntry> {
        self.tools.get(&tool)
    }

    /// Enabled tools, sorted by id.
    pub(crate) fn tools(&self) -> Vec<Tool> {
        let mut tools: Vec<Tool> = self.tools.keys().copied().collect();
        tools.sort_by_key(|t| t.id());
        tools
    }

    /// Number of requests currently waiting for a slot.
    pub(crate) fn waiting(&self) -> usize {
        self.waiting.load(Ordering::SeqCst)
    }
}

/// Result of trying to queue a request for a run slot.
pub(crate) enum Admission {
    /// The request can wait for a slot.
    Queued(QueueGuard),
    /// The queue is full.
    QueueFull,
}

/// Guard that keeps track of a request waiting for a run slot. The queue
/// position is released when the guard is dropped.
pub(crate) struct QueueGuard {
    /// State the queue position was claimed from.
    state: Arc<State>,
}

/// Runner configuration.
#[derive(Debug, Clone)]
pub(crate) struct RunnerConfig {
    /// Maximum number of tool runs executed concurrently.
    pub max_concurrent_runs: usize,
    /// Maximum number of requests waiting for a run slot.
    pub max_queue: usize,
    /// Minimum budget a run must have left when admitted to be started.
    pub min_budget: Duration,
}

/// Tool enabled in the runner.
#[derive(Debug, Clone)]
pub(crate) struct ToolEntry {
    /// Request-wide deadline for runs of this tool (including queue time).
    pub deadline: Duration,
    /// Probed tool binary used for the runs.
    pub local: LocalTool,
}

impl Drop for QueueGuard {
    fn drop(&mut self) {
        self.state.waiting.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_guard_drop_releases_position() {
        // Setup state with room for one queued request
        let state = Arc::new(State::new(vec![], config(1)).unwrap());

        // Claim the position and check it is tracked
        let guard = state.enqueue();
        assert!(matches!(guard, Admission::Queued(_)));
        assert_eq!(state.waiting(), 1);

        // Check dropping the guard releases the position
        drop(guard);
        assert_eq!(state.waiting(), 0);
        assert!(matches!(state.enqueue(), Admission::Queued(_)));
    }

    #[test]
    fn enqueue_queue_full_rejected() {
        // Setup state with room for one queued request
        let state = Arc::new(State::new(vec![], config(1)).unwrap());

        // Fill the queue and check the next request is rejected
        let _guard = state.enqueue();
        assert!(matches!(state.enqueue(), Admission::QueueFull));
        assert_eq!(state.waiting(), 1);
    }

    #[test]
    fn new_rejects_zero_concurrency() {
        let err = State::new(
            vec![],
            RunnerConfig {
                max_concurrent_runs: 0,
                max_queue: 1,
                min_budget: Duration::from_millis(1),
            },
        )
        .expect_err("zero concurrency to be rejected");
        assert!(err.to_string().contains("maxConcurrentRuns"), "{err}");
    }

    #[tokio::test]
    async fn running_tracks_acquired_slots() {
        // Setup state with a single run slot
        let state = Arc::new(State::new(vec![], config(1)).unwrap());
        assert_eq!(state.running(), 0);

        // Check acquiring and releasing the slot is reflected
        let permit = state.acquire_slot().await;
        assert_eq!(state.running(), 1);
        drop(permit);
        assert_eq!(state.running(), 0);
    }

    // Helpers.

    /// Configuration with a single run slot and the queue size provided.
    fn config(max_queue: usize) -> RunnerConfig {
        RunnerConfig {
            max_concurrent_runs: 1,
            max_queue,
            min_budget: Duration::from_millis(1),
        }
    }
}
