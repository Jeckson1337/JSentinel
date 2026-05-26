use jsentinel_policy::{
    ActionAvailability, ActionKind, ActionPlan, ActionRequest, ActionResult, ActionStatus,
    ActionRiskLevel, PolicyEngine,
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KillProcessTarget {
    pub pid: u32,
    pub process_name: Option<String>,
    pub process_path: Option<String>,
    pub command_line: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KillProcessSafetyCheck {
    pub allowed: bool,
    pub reason: Option<String>,
    pub risk_level: ActionRiskLevel,
    pub requires_confirmation: bool,
    pub protected_reason: Option<String>,
}

impl KillProcessSafetyCheck {
    pub fn allowed() -> Self {
        Self {
            allowed: true,
            reason: None,
            risk_level: ActionRiskLevel::Dangerous,
            requires_confirmation: true,
            protected_reason: None,
        }
    }

    pub fn denied(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            allowed: false,
            reason: Some(reason.clone()),
            risk_level: ActionRiskLevel::Dangerous,
            requires_confirmation: true,
            protected_reason: Some(reason),
        }
    }
}

pub trait SafeActionAdapter {
    fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError>;
    fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError>;
    fn precheck_kill_process(&self, target: &KillProcessTarget) -> KillProcessSafetyCheck;
    fn kill_process(
        &self,
        target: &KillProcessTarget,
    ) -> Result<KillProcessSafetyCheck, SafeActionError>;
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
            ActionKind::KillProcess => self.execute_kill_process(&plan),
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

    fn execute_kill_process(&self, plan: &ActionPlan) -> ActionResult {
        let Some(target) = kill_process_target_from_request(&plan.request) else {
            return denied_action_result(
                plan,
                "Missing PID metadata. Package 4C requires an explicit single process PID.",
            );
        };

        let safety = self.adapter.precheck_kill_process(&target);
        if !safety.allowed {
            return denied_kill_result(plan, &target, &safety);
        }

        match self.adapter.kill_process(&target) {
            Ok(final_safety) if final_safety.allowed => {
                let mut result = ActionResult::from_plan(
                    plan,
                    ActionStatus::Succeeded,
                    "Terminated one process by PID. No files, startup entries, firewall rules, or child processes were changed.",
                );
                result.metadata_json = Some(kill_metadata(&target, &final_safety));
                result
            }
            Ok(final_safety) => denied_kill_result(plan, &target, &final_safety),
            Err(error) => {
                let mut result = error_result(plan, error);
                result.metadata_json = Some(kill_metadata(&target, &safety));
                result
            }
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

    fn precheck_kill_process(&self, target: &KillProcessTarget) -> KillProcessSafetyCheck {
        let current_pid = current_process_id();
        evaluate_kill_process_safety(target, current_pid, None)
    }

    fn kill_process(
        &self,
        _target: &KillProcessTarget,
    ) -> Result<KillProcessSafetyCheck, SafeActionError> {
        Err(SafeActionError::UnsupportedPlatform(
            "Kill process is only implemented by the Windows backend in Package 4C.".to_string(),
        ))
    }
}

pub fn validate_reveal_path(target: &str) -> Result<ValidatedRevealPath, SafeActionError> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Err(SafeActionError::InvalidTarget(
            "Path target is empty.".to_string(),
        ));
    }

    if trimmed.contains('\0') {
        return Err(SafeActionError::InvalidTarget(
            "Path target contains an embedded null character.".to_string(),
        ));
    }

    if trimmed != target {
        return Err(SafeActionError::InvalidTarget(
            "Path target must not contain leading or trailing whitespace.".to_string(),
        ));
    }

    if trimmed.starts_with("\\\\") || trimmed.starts_with("//") {
        return Err(SafeActionError::InvalidTarget(
            "UNC/network paths are not supported by safe actions yet.".to_string(),
        ));
    }

    let lowered = trimmed.to_ascii_lowercase();
    if matches!(
        lowered.as_str(),
        "cmd" | "cmd.exe" | "powershell" | "powershell.exe" | "pwsh" | "pwsh.exe"
    ) {
        return Err(SafeActionError::InvalidTarget(
            "Raw command names are not valid local path targets.".to_string(),
        ));
    }

    if lowered.starts_with("http://")
        || lowered.starts_with("https://")
        || lowered.starts_with("file://")
        || lowered.starts_with("ms-settings:")
        || lowered.starts_with("shell:")
        || lowered.starts_with("cmd:")
        || lowered.starts_with("powershell:")
        || lowered.starts_with("javascript:")
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
            let target = request.target.as_str();
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
    if uri.is_empty() || uri.contains('\0') || uri.trim() != uri {
        return false;
    }

    ALLOWED_WINDOWS_SETTINGS_URIS
        .iter()
        .any(|allowed| uri.eq_ignore_ascii_case(allowed))
}

pub fn allowed_windows_settings_uris() -> &'static [&'static str] {
    ALLOWED_WINDOWS_SETTINGS_URIS
}

pub fn kill_process_target_from_request(request: &ActionRequest) -> Option<KillProcessTarget> {
    let metadata = request.metadata_json.as_ref()?;
    let pid = metadata.get("pid")?.as_u64().and_then(|value| u32::try_from(value).ok())?;
    Some(KillProcessTarget {
        pid,
        process_name: metadata
            .get("process_name")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned),
        process_path: metadata
            .get("process_path")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned),
        command_line: metadata
            .get("command_line")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned),
    })
}

pub fn evaluate_kill_process_safety(
    target: &KillProcessTarget,
    current_pid: u32,
    parent_pid: Option<u32>,
) -> KillProcessSafetyCheck {
    if target.pid == 0 {
        return KillProcessSafetyCheck::denied("PID 0 cannot be terminated.");
    }
    if is_self_or_parent_process(target.pid, current_pid, parent_pid) {
        if target.pid == current_pid {
            return KillProcessSafetyCheck::denied("JSentinel will not terminate its own process.");
        }
        return KillProcessSafetyCheck::denied(
            "JSentinel will not terminate its parent desktop process.",
        );
    }

    let Some(name) = target
        .process_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return KillProcessSafetyCheck::denied(
            "Process name is unavailable; Package 4C requires verified process details.",
        );
    };

    if name.to_ascii_lowercase().starts_with("pid-") || name.eq_ignore_ascii_case("unknown") {
        return KillProcessSafetyCheck::denied(
            "Process name is not verified; Package 4C refuses kill-by-PID without details.",
        );
    }

    if is_hard_denied_process_name(name) {
        return KillProcessSafetyCheck::denied(format!(
            "{name} is protected by the Package 4C hard-deny process list."
        ));
    }

    let Some(path) = target
        .process_path
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return KillProcessSafetyCheck::denied(
            "Process path is unavailable; Package 4C requires verified process details.",
        );
    };

    if is_windows_system_path(path) {
        return KillProcessSafetyCheck::denied(
            "Processes under Windows system directories are denied in Package 4C.",
        );
    }

    if is_windows_directory_path(path) {
        return KillProcessSafetyCheck::denied(
            "Processes under the Windows directory are denied in Package 4C.",
        );
    }

    KillProcessSafetyCheck::allowed()
}

pub fn current_process_id() -> u32 {
    std::process::id()
}

pub fn is_self_or_parent_process(pid: u32, current_pid: u32, parent_pid: Option<u32>) -> bool {
    pid == current_pid || parent_pid == Some(pid)
}

pub fn is_hard_denied_process_name(name: &str) -> bool {
    let normalized = name.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "system"
            | "idle"
            | "registry"
            | "smss.exe"
            | "csrss.exe"
            | "wininit.exe"
            | "winlogon.exe"
            | "services.exe"
            | "lsass.exe"
            | "lsm.exe"
            | "svchost.exe"
            | "fontdrvhost.exe"
            | "dwm.exe"
            | "conhost.exe"
            | "explorer.exe"
    )
}

pub fn is_windows_system_path(path: &str) -> bool {
    let normalized = path.replace('/', "\\").to_ascii_lowercase();
    normalized.starts_with("c:\\windows\\system32\\")
        || normalized == "c:\\windows\\system32"
        || normalized.starts_with("c:\\windows\\syswow64\\")
        || normalized == "c:\\windows\\syswow64"
}

pub fn is_windows_directory_path(path: &str) -> bool {
    let normalized = path.replace('/', "\\").to_ascii_lowercase();
    normalized.starts_with("c:\\windows\\") || normalized == "c:\\windows"
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

fn denied_action_result(plan: &ActionPlan, reason: impl Into<String>) -> ActionResult {
    let reason = reason.into();
    let mut result = ActionResult::from_plan(plan, ActionStatus::Denied, reason.clone());
    result.error = Some(reason);
    result
}

fn denied_kill_result(
    plan: &ActionPlan,
    target: &KillProcessTarget,
    safety: &KillProcessSafetyCheck,
) -> ActionResult {
    let reason = safety
        .reason
        .clone()
        .unwrap_or_else(|| "Kill process was denied by safety policy.".to_string());
    let mut result = ActionResult::from_plan(plan, ActionStatus::Denied, reason.clone());
    result.error = Some(reason);
    result.metadata_json = Some(kill_metadata(target, safety));
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

fn kill_metadata(target: &KillProcessTarget, safety: &KillProcessSafetyCheck) -> Value {
    serde_json::json!({
        "pid": target.pid,
        "process_name": target.process_name,
        "process_path": target.process_path,
        "metadata_note": "process_name/process_path are request context; Windows backend re-queries PID before execution. Command line is intentionally not stored in audit metadata.",
        "safety": safety,
    })
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

    #[derive(Default)]
    struct FailingKillAdapter;

    impl SafeActionAdapter for MockAdapter {
        fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
            self.revealed.borrow_mut().push(target.path.clone());
            Ok(())
        }

        fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError> {
            self.settings.borrow_mut().push(uri.to_string());
            Ok(())
        }

        fn precheck_kill_process(&self, target: &KillProcessTarget) -> KillProcessSafetyCheck {
            evaluate_kill_process_safety(target, 9000, Some(9001))
        }

        fn kill_process(
            &self,
            target: &KillProcessTarget,
        ) -> Result<KillProcessSafetyCheck, SafeActionError> {
            Ok(self.precheck_kill_process(target))
        }
    }

    impl SafeActionAdapter for FailingKillAdapter {
        fn reveal_path(&self, _target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
            Ok(())
        }

        fn open_windows_settings(&self, _uri: &str) -> Result<(), SafeActionError> {
            Ok(())
        }

        fn precheck_kill_process(&self, target: &KillProcessTarget) -> KillProcessSafetyCheck {
            evaluate_kill_process_safety(target, 9000, Some(9001))
        }

        fn kill_process(
            &self,
            _target: &KillProcessTarget,
        ) -> Result<KillProcessSafetyCheck, SafeActionError> {
            Err(SafeActionError::OsError(
                "mock terminate process failure".to_string(),
            ))
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
            ActionKind::BlockNetwork,
            "tcp:443",
            "TCP 443",
            "network",
        ));

        assert_eq!(result.status, ActionStatus::Denied);
        assert!(result.error.is_some());
    }

    #[test]
    fn kill_process_missing_pid_metadata_is_denied() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let result = executor.execute(ActionRequest::new(
            ActionKind::KillProcess,
            "demo.exe",
            "Demo process",
            "processes",
        ));

        assert_eq!(result.status, ActionStatus::Denied);
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
            "http://example.com",
            "https://example.com",
            "file:///C:/Windows/System32/cmd.exe",
            "cmd.exe",
            "powershell.exe",
            "powershell",
            "ms-settings:../../cmd",
            "ms-settings:privacy;cmd",
            "shell:AppsFolder",
            "javascript:alert(1)",
            " ms-settings:privacy",
            "ms-settings:privacy ",
            "ms-settings:privacy\0",
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
        for target in [
            "http://example.com",
            "https://example.com",
            "file:///C:/Windows/System32/cmd.exe",
            "ms-settings:privacy",
            "shell:AppsFolder",
            "cmd.exe",
            "powershell.exe",
            "cmd:dir",
            "powershell:Start-Process",
            "javascript:alert(1)",
        ] {
            let error = validate_reveal_path(target).expect_err("URL target must be rejected");

            assert!(matches!(error, SafeActionError::InvalidTarget(_)));
        }
    }

    #[test]
    fn reveal_path_validation_rejects_unc_and_null_targets() {
        for target in ["\\\\server\\share", "//server/share", "C:\\Temp\0file.txt"] {
            let error = validate_reveal_path(target).expect_err("target must be rejected");

            assert!(matches!(error, SafeActionError::InvalidTarget(_)));
        }
    }

    #[test]
    fn reveal_path_validation_rejects_nonexistent_path() {
        let missing = std::env::temp_dir().join("jsentinel-definitely-missing-path-for-test");
        let error = validate_reveal_path(missing.to_string_lossy().as_ref())
            .expect_err("missing path must be rejected");

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
    fn executor_denies_whitespace_padded_settings_metadata() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let mut request = ActionRequest::new(
            ActionKind::OpenWindowsSettings,
            "",
            "Privacy settings",
            "settings",
        );
        request.metadata_json = Some(json!({ "settings_page": " ms-settings:privacy " }));

        let result = executor.execute(request);

        assert_eq!(result.status, ActionStatus::Denied);
    }

    #[test]
    fn executor_returns_unsupported_for_unsupported_action() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let result = executor.execute(ActionRequest::new(
            ActionKind::DetectFileLockers,
            "C:\\Demo\\locked.txt",
            "Locked file",
            "files",
        ));

        assert_eq!(result.status, ActionStatus::Unsupported);
        assert!(result.error.is_some());
    }

    #[test]
    fn kill_safety_denies_pid_zero_current_and_parent() {
        let target = KillProcessTarget {
            pid: 0,
            process_name: Some("demo.exe".to_string()),
            process_path: Some("C:\\Users\\Demo\\demo.exe".to_string()),
            command_line: None,
        };
        assert!(!evaluate_kill_process_safety(&target, 100, Some(200)).allowed);

        let target = KillProcessTarget { pid: 100, ..target };
        assert!(!evaluate_kill_process_safety(&target, 100, Some(200)).allowed);

        let target = KillProcessTarget { pid: 200, ..target };
        assert!(!evaluate_kill_process_safety(&target, 100, Some(200)).allowed);
    }

    #[test]
    fn kill_safety_denies_critical_names_and_system_paths() {
        for name in ["System", "lsass.exe", "svchost.exe", "conhost.exe", "explorer.exe"] {
            let target = KillProcessTarget {
                pid: 42,
                process_name: Some(name.to_string()),
                process_path: Some(format!("C:\\Users\\Demo\\{name}")),
                command_line: None,
            };

            assert!(!evaluate_kill_process_safety(&target, 100, None).allowed);
        }

        let system_path_target = KillProcessTarget {
            pid: 42,
            process_name: Some("demo.exe".to_string()),
            process_path: Some("C:\\Windows\\System32\\demo.exe".to_string()),
            command_line: None,
        };
        assert!(!evaluate_kill_process_safety(&system_path_target, 100, None).allowed);

        let windows_path_target = KillProcessTarget {
            pid: 42,
            process_name: Some("demo.exe".to_string()),
            process_path: Some("C:\\Windows\\Temp\\demo.exe".to_string()),
            command_line: None,
        };
        assert!(!evaluate_kill_process_safety(&windows_path_target, 100, None).allowed);
    }

    #[test]
    fn kill_safety_denies_unknown_or_unverified_target() {
        for target in [
            KillProcessTarget {
                pid: 42,
                process_name: None,
                process_path: Some("C:\\Users\\Demo\\demo.exe".to_string()),
                command_line: None,
            },
            KillProcessTarget {
                pid: 42,
                process_name: Some("pid-42".to_string()),
                process_path: Some("C:\\Users\\Demo\\demo.exe".to_string()),
                command_line: None,
            },
            KillProcessTarget {
                pid: 42,
                process_name: Some("unknown".to_string()),
                process_path: Some("C:\\Users\\Demo\\demo.exe".to_string()),
                command_line: None,
            },
            KillProcessTarget {
                pid: 42,
                process_name: Some("demo.exe".to_string()),
                process_path: None,
                command_line: None,
            },
        ] {
            assert!(!evaluate_kill_process_safety(&target, 100, None).allowed);
        }
    }

    #[test]
    fn kill_safety_allows_mock_user_process() {
        let target = KillProcessTarget {
            pid: 42,
            process_name: Some("demo.exe".to_string()),
            process_path: Some("C:\\Users\\Demo\\AppData\\Local\\Demo\\demo.exe".to_string()),
            command_line: Some("demo.exe".to_string()),
        };

        assert!(evaluate_kill_process_safety(&target, 100, Some(200)).allowed);
    }

    #[test]
    fn executor_kill_process_success_with_mock_adapter() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let mut request = ActionRequest::new(
            ActionKind::KillProcess,
            "42",
            "Demo process",
            "processes",
        );
        request.metadata_json = Some(serde_json::json!({
            "pid": 42,
            "process_name": "demo.exe",
            "process_path": "C:\\Users\\Demo\\demo.exe",
            "command_line": "demo.exe --sensitive-token-placeholder"
        }));

        let result = executor.execute(request);

        assert_eq!(result.status, ActionStatus::Succeeded);
        let metadata = result.metadata_json.expect("kill metadata is recorded");
        assert_eq!(metadata["pid"].as_u64(), Some(42));
        assert!(metadata.get("command_line").is_none());
    }

    #[test]
    fn executor_kill_process_denied_path_writes_denied_result() {
        let executor = SafeActionExecutor::new(MockAdapter::default());
        let mut request = ActionRequest::new(
            ActionKind::KillProcess,
            "0",
            "PID 0",
            "processes",
        );
        request.metadata_json = Some(serde_json::json!({
            "pid": 0,
            "process_name": "System",
            "process_path": "C:\\Windows\\System32\\System"
        }));

        let result = executor.execute(request);

        assert_eq!(result.status, ActionStatus::Denied);
        assert!(result.error.is_some());
        assert!(result.metadata_json.is_some());
    }

    #[test]
    fn executor_kill_process_os_error_writes_failed_result() {
        let executor = SafeActionExecutor::new(FailingKillAdapter);
        let mut request = ActionRequest::new(
            ActionKind::KillProcess,
            "42",
            "Demo process",
            "processes",
        );
        request.metadata_json = Some(serde_json::json!({
            "pid": 42,
            "process_name": "demo.exe",
            "process_path": "C:\\Users\\Demo\\demo.exe"
        }));

        let result = executor.execute(request);

        assert_eq!(result.status, ActionStatus::Failed);
        assert!(result.error.is_some());
        assert!(result.metadata_json.is_some());
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
