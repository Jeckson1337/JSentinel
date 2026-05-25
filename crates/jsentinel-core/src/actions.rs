use jsentinel_policy::{
    ActionAvailability, ActionKind, ActionPlan, ActionRequest, ActionResult, ActionStatus,
    PolicyEngine,
};
use serde_json::Value;
use std::fmt;
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use std::process::Command;

const PACKAGE_4B_DISABLED_REASON: &str =
    "Action is disabled by Package 4B policy and was not executed.";

const ALLOWED_WINDOWS_SETTINGS_URIS: &[&str] = &[
    "ms-settings:privacy",
    "ms-settings:privacy-microphone",
    "ms-settings:privacy-webcam",
    "ms-settings:appsfeatures",
    "ms-settings:startupapps",
    "ms-settings:network-status",
    "ms-settings:windowsdefender",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafeActionError {
    InvalidTarget(String),
    UnsupportedPlatform(String),
    OsError(String),
    Denied(String),
}

impl fmt::Display for SafeActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(message)
            | Self::UnsupportedPlatform(message)
            | Self::OsError(message)
            | Self::Denied(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for SafeActionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedRevealPath {
    pub path: PathBuf,
    pub select_file: bool,
}

pub trait SafeActionAdapter {
    fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError>;
    fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError>;
}

pub struct SafeActionExecutor<A> {
    adapter: A,
}

impl<A> SafeActionExecutor<A>
where
    A: SafeActionAdapter,
{
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    pub fn plan_action(&self, request: ActionRequest) -> ActionPlan {
        PolicyEngine::plan_action(request)
    }

    pub fn execute(&self, request: ActionRequest) -> ActionResult {
        let plan = PolicyEngine::plan_action(request);
        if !PolicyEngine::is_action_enabled(plan.request.kind) {
            return denied_result(&plan);
        }

        match plan.request.kind {
            ActionKind::RevealPath => self.execute_reveal_path(&plan),
            ActionKind::OpenWindowsSettings => self.execute_open_windows_settings(&plan),
            _ => denied_result(&plan),
        }
    }

    fn execute_reveal_path(&self, plan: &ActionPlan) -> ActionResult {
        match validate_reveal_path(&plan.request.target) {
            Ok(target) => match self.adapter.reveal_path(&target) {
                Ok(()) => ActionResult::from_plan(
                    plan,
                    ActionStatus::Succeeded,
                    "Opened local filesystem path without modifying files or settings.",
                ),
                Err(error) => error_result(plan, error),
            },
            Err(error) => error_result(plan, error),
        }
    }

    fn execute_open_windows_settings(&self, plan: &ActionPlan) -> ActionResult {
        let Some(uri) = settings_uri_from_request(&plan.request) else {
            return error_result(
                plan,
                SafeActionError::InvalidTarget(
                    "Missing allowlisted Windows settings page.".to_string(),
                ),
            );
        };

        if !is_allowed_windows_settings_uri(&uri) {
            return error_result(
                plan,
                SafeActionError::Denied(
                    "Windows settings page is not in the Package 4B allowlist.".to_string(),
                ),
            );
        }

        match self.adapter.open_windows_settings(&uri) {
            Ok(()) => ActionResult::from_plan(
                plan,
                ActionStatus::Succeeded,
                "Opened allowlisted Windows settings page without changing settings.",
            ),
            Err(error) => error_result(plan, error),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultSafeActionAdapter;

impl SafeActionAdapter for DefaultSafeActionAdapter {
    fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
        open_local_path(target)
    }

    fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError> {
        open_allowlisted_windows_settings(uri)
    }
}

pub fn validate_reveal_path(target: &str) -> Result<ValidatedRevealPath, SafeActionError> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Err(SafeActionError::InvalidTarget(
            "Path target is empty.".to_string(),
        ));
    }

    let lowered = trimmed.to_ascii_lowercase();
    if lowered.starts_with("http://")
        || lowered.starts_with("https://")
        || lowered.starts_with("file://")
        || lowered.starts_with("ms-settings:")
    {
        return Err(SafeActionError::InvalidTarget(
            "Only local filesystem paths are allowed.".to_string(),
        ));
    }

    let path = PathBuf::from(trimmed);
    if !path.exists() {
        return Err(SafeActionError::InvalidTarget(
            "Path does not exist; JSentinel will not create it.".to_string(),
        ));
    }

    let canonical = path.canonicalize().map_err(|error| {
        SafeActionError::InvalidTarget(format!("Path cannot be canonicalized: {error}"))
    })?;
    let metadata = std::fs::metadata(&canonical).map_err(|error| {
        SafeActionError::InvalidTarget(format!("Path metadata is unavailable: {error}"))
    })?;

    Ok(ValidatedRevealPath {
        path: canonical,
        select_file: metadata.is_file(),
    })
}

pub fn settings_uri_from_request(request: &ActionRequest) -> Option<String> {
    request
        .metadata_json
        .as_ref()
        .and_then(settings_page_from_metadata)
        .or_else(|| {
            let target = request.target.trim();
            (!target.is_empty()).then(|| target.to_string())
        })
}

fn settings_page_from_metadata(metadata: &Value) -> Option<String> {
    metadata
        .get("settings_page")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

pub fn is_allowed_windows_settings_uri(uri: &str) -> bool {
    ALLOWED_WINDOWS_SETTINGS_URIS
        .iter()
        .any(|allowed| uri.eq_ignore_ascii_case(allowed))
}

pub fn allowed_windows_settings_uris() -> &'static [&'static str] {
    ALLOWED_WINDOWS_SETTINGS_URIS
}

fn denied_result(plan: &ActionPlan) -> ActionResult {
    let status = match plan.availability {
        ActionAvailability::Unsupported => ActionStatus::Unsupported,
        _ => ActionStatus::Denied,
    };
    let mut result = ActionResult::from_plan(
        plan,
        status,
        plan.disabled_reason
            .clone()
            .unwrap_or_else(|| PACKAGE_4B_DISABLED_REASON.to_string()),
    );
    result.error = Some(PACKAGE_4B_DISABLED_REASON.to_string());
    result
}

fn error_result(plan: &ActionPlan, error: SafeActionError) -> ActionResult {
    let status = match error {
        SafeActionError::UnsupportedPlatform(_) => ActionStatus::Unsupported,
        SafeActionError::Denied(_) => ActionStatus::Denied,
        SafeActionError::InvalidTarget(_) | SafeActionError::OsError(_) => ActionStatus::Failed,
    };
    let message = error.to_string();
    let mut result = ActionResult::from_plan(plan, status, message.clone());
    result.error = Some(message);
    result
}

#[cfg(target_os = "windows")]
fn open_local_path(target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
    let status = if target.select_file {
        Command::new("explorer.exe")
            .arg(format!("/select,{}", target.path.display()))
            .status()
    } else {
        Command::new("explorer.exe").arg(&target.path).status()
    };

    status
        .map_err(|error| SafeActionError::OsError(format!("Explorer launch failed: {error}")))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(SafeActionError::OsError(format!(
                    "Explorer returned status {status}."
                )))
            }
        })
}

#[cfg(not(target_os = "windows"))]
fn open_local_path(_target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
    Err(SafeActionError::UnsupportedPlatform(
        "Reveal path is implemented only for Windows in Package 4B.".to_string(),
    ))
}

#[cfg(target_os = "windows")]
fn open_allowlisted_windows_settings(uri: &str) -> Result<(), SafeActionError> {
    if !is_allowed_windows_settings_uri(uri) {
        return Err(SafeActionError::Denied(
            "Windows settings page is not allowlisted.".to_string(),
        ));
    }

    Command::new("explorer.exe")
        .arg(uri)
        .status()
        .map_err(|error| {
            SafeActionError::OsError(format!("Windows Settings launch failed: {error}"))
        })
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(SafeActionError::OsError(format!(
                    "Windows Settings returned status {status}."
                )))
            }
        })
}

#[cfg(not(target_os = "windows"))]
fn open_allowlisted_windows_settings(_uri: &str) -> Result<(), SafeActionError> {
    Err(SafeActionError::UnsupportedPlatform(
        "Windows Settings actions are unsupported on this platform.".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsentinel_policy::ActionRiskLevel;
    use serde_json::json;
    use std::cell::RefCell;
    use std::fs;

    #[derive(Default)]
    struct MockAdapter {
        revealed: RefCell<Vec<PathBuf>>,
        settings: RefCell<Vec<String>>,
    }

    impl SafeActionAdapter for MockAdapter {
        fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
            self.revealed.borrow_mut().push(target.path.clone());
            Ok(())
        }

        fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError> {
            self.settings.borrow_mut().push(uri.to_string());
            Ok(())
        }
    }

    #[test]
    fn policy_marks_safe_actions_available_with_confirmation() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let reveal_plan = executor.plan_action(ActionRequest::new(
            ActionKind::RevealPath,
            ".",
            "Current directory",
            "files",
        ));
        let settings_plan = executor.plan_action(ActionRequest::new(
            ActionKind::OpenWindowsSettings,
            "ms-settings:privacy",
            "Privacy settings",
            "settings",
        ));

        assert_eq!(reveal_plan.availability, ActionAvailability::Available);
        assert_eq!(settings_plan.availability, ActionAvailability::Available);
        assert!(reveal_plan.requires_confirmation);
        assert!(settings_plan.requires_confirmation);
        assert_eq!(reveal_plan.request.risk_level, ActionRiskLevel::Safe);
    }

    #[test]
    fn dangerous_actions_stay_planned_and_disabled() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let result = executor.execute(ActionRequest::new(
            ActionKind::KillProcess,
            "42",
            "PID 42",
            "processes",
        ));

        assert_eq!(result.status, ActionStatus::Denied);
        assert!(result.error.is_some());
    }

    #[test]
    fn settings_allowlist_accepts_known_pages() {
        assert!(is_allowed_windows_settings_uri("ms-settings:privacy"));
        assert!(is_allowed_windows_settings_uri(
            "MS-SETTINGS:PRIVACY-MICROPHONE"
        ));
    }

    #[test]
    fn settings_allowlist_rejects_arbitrary_urls_and_commands() {
        for value in [
            "https://example.com",
            "file://C:/Windows/System32/cmd.exe",
            "cmd.exe",
            "powershell",
            "ms-settings:../../cmd",
        ] {
            assert!(!is_allowed_windows_settings_uri(value));
        }
    }

    #[test]
    fn reveal_path_validation_rejects_empty_target() {
        let error = validate_reveal_path("").expect_err("empty path must be rejected");

        assert!(matches!(error, SafeActionError::InvalidTarget(_)));
    }

    #[test]
    fn reveal_path_validation_rejects_url_targets() {
        let error =
            validate_reveal_path("https://example.com").expect_err("URL target must be rejected");

        assert!(matches!(error, SafeActionError::InvalidTarget(_)));
    }

    #[test]
    fn executor_opens_allowlisted_settings_with_mock_adapter() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let mut request = ActionRequest::new(
            ActionKind::OpenWindowsSettings,
            "",
            "Privacy settings",
            "settings",
        );
        request.metadata_json = Some(json!({ "settings_page": "ms-settings:privacy" }));

        let result = executor.execute(request);

        assert_eq!(result.status, ActionStatus::Succeeded);
    }

    #[test]
    fn executor_denies_non_allowlisted_settings_target() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let result = executor.execute(ActionRequest::new(
            ActionKind::OpenWindowsSettings,
            "https://example.com",
            "External URL",
            "settings",
        ));

        assert_eq!(result.status, ActionStatus::Denied);
    }

    #[test]
    fn executor_reveals_existing_path_with_mock_adapter() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let temp = std::env::temp_dir().join(format!(
            "jsentinel-action-test-{}",
            nanos
        ));
        fs::create_dir_all(&temp).expect("temp directory should be created for validation");

        let executor = SafeActionExecutor::new(MockAdapter::default());
        let result = executor.execute(ActionRequest::new(
            ActionKind::RevealPath,
            temp.to_string_lossy().to_string(),
            "Temp directory",
            "files",
        ));

        let _ = fs::remove_dir_all(&temp);
        assert_eq!(result.status, ActionStatus::Succeeded);
    }
}
