use serde::{Deserialize, Serialize};

/// Check result information.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckResult<T = ()> {
    pub passed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T> Default for CheckResult<T> {
    fn default() -> Self {
        Self {
            passed: false,
            url: None,
            value: None,
        }
    }
}

impl<T> From<bool> for CheckResult<T> {
    fn from(passed: bool) -> Self {
        Self {
            passed,
            ..Default::default()
        }
    }
}

impl<T> From<Option<T>> for CheckResult<T> {
    fn from(value: Option<T>) -> Self {
        Self {
            passed: value.is_some(),
            value,
            ..Default::default()
        }
    }
}

impl<T> From<(bool, Option<T>)> for CheckResult<T> {
    fn from((passed, value): (bool, Option<T>)) -> Self {
        Self {
            passed,
            value,
            ..Default::default()
        }
    }
}

impl<T> CheckResult<T> {
    /// Create a new CheckResult instance from the url provided.
    pub(crate) fn from_url(url: Option<String>) -> Self {
        Self {
            passed: url.is_some(),
            url,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_result_from_passed() {
        assert_eq!(
            CheckResult::<()>::from(true),
            CheckResult {
                passed: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_value_some() {
        assert_eq!(
            CheckResult::from(Some("value".to_string())),
            CheckResult {
                passed: true,
                value: Some("value".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_value_none() {
        assert_eq!(
            CheckResult::<()>::from(None),
            CheckResult {
                passed: false,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_tuple_passed_value_some() {
        assert_eq!(
            CheckResult::from((true, Some("value".to_string()))),
            CheckResult {
                passed: true,
                value: Some("value".to_string()),
                ..Default::default()
            }
        );
        assert_eq!(
            CheckResult::from((false, Some("value".to_string()))),
            CheckResult {
                value: Some("value".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_tuple_passed_value_none() {
        assert_eq!(
            CheckResult::<(bool, String)>::from((true, None)),
            CheckResult {
                passed: true,
                ..Default::default()
            }
        );
        assert_eq!(
            CheckResult::<(bool, String)>::from((false, None)),
            CheckResult {
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_url_some() {
        assert_eq!(
            CheckResult::<()>::from_url(Some("url".to_string())),
            CheckResult {
                passed: true,
                url: Some("url".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_result_from_url_none() {
        assert_eq!(
            CheckResult::<()>::from_url(None),
            CheckResult {
                passed: false,
                ..Default::default()
            }
        );
    }
}
