use rinja::Template;

/// Template for the annual review due comment.
#[derive(Template)]
#[template(path = "annual-review-due.md")]
pub(crate) struct AnnualReviewDue {}

/// Template for the annual review due reminder comment.
#[derive(Template)]
#[template(path = "annual-review-due-reminder.md")]
pub(crate) struct AnnualReviewDueReminder {}
