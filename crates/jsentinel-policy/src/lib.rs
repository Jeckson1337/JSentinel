#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static ACTION_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

const FUTURE_ACTION_REASON: &str =
    "Action framework is prepared, implementation will come in a later package.";
const STARTUP_DISABLE_PLANNED_REASON: &str =
    "Startup disable is planned but not implemented in Package 4D.";
const STARTUP_RESTORE_PLANNED_REASON: &str =
    "Startup restore is planned but not implemented in Package 4D.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionRisk {
    ReadOnly,
    Reversible,
    Destructive,
    Privileged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDecision {
    AllowReadOnly,
    RequireConfirmation,
    BlockInV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvaluation {
    pub risk: ActionRisk,
    pub decision: ActionDecision,
    pub reason: &'static str,
}

pub fn evaluate_placeholder(risk: ActionRisk) -> PolicyEvaluation {
    let decision = match risk {
        ActionRisk::ReadOnly => ActionDecision::AllowReadOnly,
        ActionRisk::Reversible => ActionDecision::RequireConfirmation,
        ActionRisk::Destructive | ActionRisk::Privileged => ActionDecision::BlockInV1,
    };

    PolicyEvaluation {
        risk,
        decision,
        reason: "Package 4B allows only safe non-destructive actions; privileged actions are not implemented.",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    RevealPath,
    OpenWindowsSettings,
    KillProcess,
    BlockNetwork,
    UnblockNetwork,
    DisableStartup,
    RestoreStartup,
    QuarantineFile,
    RestoreQuarantine,
    ScheduleDeleteOnReboot,
    DetectFileLockers,
}

impl ActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RevealPath => "reveal_path",
            Self::OpenWindowsSettings => "open_windows_settings",
            Self::KillProcess => "kill_process",
            Self::BlockNetwork => "block_network",
            Self::UnblockNetwork => "unblock_network",
            Self::DisableStartup => "disable_startup",
            Self::RestoreStartup => "restore_startup",
            Self::QuarantineFile => "quarantine_file",
            Self::RestoreQuarantine => "restore_quarantine",
            Self::ScheduleDeleteOnReboot => "schedule_delete_on_reboot",
            Self::DetectFileLockers => "detect_file_lockers",
        }
    }
}

impl fmt::Display for ActionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ActionKind {
    type Err = ActionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "reveal_path" => Ok(Self::RevealPath),
            "open_windows_settings" => Ok(Self::OpenWindowsSettings),
            "kill_process" => Ok(Self::KillProcess),
            "block_network" => Ok(Self::BlockNetwork),
            "unblock_network" => Ok(Self::UnblockNetwork),
            "disable_startup" => Ok(Self::DisableStartup),
            "restore_startup" => Ok(Self::RestoreStartup),
            "quarantine_file" => Ok(Self::QuarantineFile),
            "restore_quarantine" => Ok(Self::RestoreQuarantine),
            "schedule_delete_on_reboot" => Ok(Self::ScheduleDeleteOnReboot),
            "detect_file_lockers" => Ok(Self::DetectFileLockers),
            _ => Err(ActionParseError::UnknownKind(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRiskLevel {
    Safe,
    Caution,
    Dangerous,
}

impl ActionRiskLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Caution => "caution",
            Self::Dangerous => "dangerous",
        }
    }
}

impl fmt::Display for ActionRiskLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ActionRiskLevel {
    type Err = ActionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "safe" => Ok(Self::Safe),
            "caution" => Ok(Self::Caution),
            "dangerous" => Ok(Self::Dangerous),
            _ => Err(ActionParseError::UnknownRisk(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAvailability {
    Available,
    Disabled,
    Unsupported,
    Planned,
    RequiresAdmin,
    RequiresConfirmation,
}

impl ActionAvailability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Disabled => "disabled",
            Self::Unsupported => "unsupported",
            Self::Planned => "planned",
            Self::RequiresAdmin => "requires_admin",
            Self::RequiresConfirmation => "requires_confirmation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Succeeded,
    Failed,
    Denied,
    Cancelled,
    Unsupported,
    DryRun,
}

impl ActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
            Self::Unsupported => "unsupported",
            Self::DryRun => "dry_run",
        }
    }
}

impl fmt::Display for ActionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ActionStatus {
    type Err = ActionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "denied" => Ok(Self::Denied),
            "cancelled" => Ok(Self::Cancelled),
            "unsupported" => Ok(Self::Unsupported),
            "dry_run" => Ok(Self::DryRun),
            _ => Err(ActionParseError::UnknownStatus(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allowed,
    RequiresConfirmation,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionRequest {
    pub id: String,
    pub kind: ActionKind,
    pub target: String,
    pub target_display_name: String,
    pub risk_level: ActionRiskLevel,
    pub requested_at: String,
    pub source_screen: String,
    pub metadata_json: Option<Value>,
}

impl ActionRequest {
    pub fn new(
        kind: ActionKind,
        target: impl Into<String>,
        target_display_name: impl Into<String>,
        source_screen: impl Into<String>,
    ) -> Self {
        let risk_level = PolicyEngine::classify_risk(kind);

        Self {
            id: generated_action_id(),
            kind,
            target: target.into(),
            target_display_name: target_display_name.into(),
            risk_level,
            requested_at: now_timestamp(),
            source_screen: source_screen.into(),
            metadata_json: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionPlan {
    pub request: ActionRequest,
    pub availability: ActionAvailability,
    pub requires_confirmation: bool,
    pub confirmation_title: String,
    pub confirmation_message: String,
    pub irreversible: bool,
    pub can_undo: bool,
    pub disabled_reason: Option<String>,
    pub expected_effects: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartupEntryTarget {
    pub entry_id: String,
    pub name: String,
    pub source: String,
    pub scope: String,
    pub command: Option<String>,
    pub path: Option<String>,
    pub enabled: Option<bool>,
    pub original_location: Option<String>,
    pub metadata_json: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartupBackupRecord {
    pub backup_id: String,
    pub entry_id: String,
    pub created_at: String,
    pub source: String,
    pub original_name: String,
    pub original_command: String,
    pub original_path: Option<String>,
    pub original_enabled_state: String,
    pub restore_strategy: String,
    pub metadata_json: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupBackupQuery {
    pub entry_id: Option<String>,
    pub source: Option<String>,
    pub limit: Option<u32>,
}

impl Default for StartupBackupQuery {
    fn default() -> Self {
        Self {
            entry_id: None,
            source: None,
            limit: Some(50),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartupActionPlan {
    pub action_kind: ActionKind,
    pub target: StartupEntryTarget,
    pub risk_level: ActionRiskLevel,
    pub requires_confirmation: bool,
    pub backup_required: bool,
    pub backup_available: bool,
    pub expected_effects: Vec<String>,
    pub warnings: Vec<String>,
    pub disabled_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionResult {
    pub request_id: String,
    pub kind: ActionKind,
    pub target: String,
    pub started_at: String,
    pub finished_at: String,
    pub status: ActionStatus,
    pub message: String,
    pub error: Option<String>,
    pub metadata_json: Option<Value>,
}

impl ActionResult {
    pub fn from_plan(plan: &ActionPlan, status: ActionStatus, message: impl Into<String>) -> Self {
        let timestamp = now_timestamp();
        Self {
            request_id: plan.request.id.clone(),
            kind: plan.request.kind,
            target: plan.request.target.clone(),
            started_at: timestamp.clone(),
            finished_at: timestamp,
            status,
            message: message.into(),
            error: None,
            metadata_json: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionHistoryQuery {
    pub kind: Option<ActionKind>,
    pub status: Option<ActionStatus>,
    pub text: Option<String>,
    pub limit: Option<u32>,
}

impl Default for ActionHistoryQuery {
    fn default() -> Self {
        Self {
            kind: None,
            status: None,
            text: None,
            limit: Some(50),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionParseError {
    UnknownKind(String),
    UnknownRisk(String),
    UnknownStatus(String),
}

impl fmt::Display for ActionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKind(value) => write!(formatter, "unknown action kind: {value}"),
            Self::UnknownRisk(value) => write!(formatter, "unknown action risk level: {value}"),
            Self::UnknownStatus(value) => write!(formatter, "unknown action status: {value}"),
        }
    }
}

impl std::error::Error for ActionParseError {}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn plan_action(request: ActionRequest) -> ActionPlan {
        let risk_level = Self::classify_risk(request.kind);
        let mut request = request;
        request.risk_level = risk_level;

        match request.kind {
            ActionKind::RevealPath | ActionKind::OpenWindowsSettings => ActionPlan {
                confirmation_title: "Confirm safe local action".to_string(),
                confirmation_message:
                    "This opens a local Windows UI/path only. It does not modify files or settings."
                        .to_string(),
                request,
                availability: ActionAvailability::Available,
                requires_confirmation: true,
                irreversible: false,
                can_undo: false,
                disabled_reason: None,
                expected_effects: vec![
                    "No files, processes, registry keys, firewall rules, or startup entries are modified."
                        .to_string(),
                ],
                warnings: vec![
                    "The action is limited to opening Explorer or an allowlisted Windows Settings page."
                        .to_string(),
                ],
            },
            ActionKind::KillProcess => {
                if !request_has_pid_metadata(&request) {
                    return planned_plan(
                        request,
                        ActionAvailability::Disabled,
                        "Kill process requires explicit PID metadata and cannot use a name-only target.",
                    );
                }

                ActionPlan {
                    confirmation_title: "Confirm process termination".to_string(),
                    confirmation_message:
                        "This terminates one running process by PID. Unsaved work in that process may be lost."
                            .to_string(),
                    request,
                    availability: ActionAvailability::RequiresConfirmation,
                    requires_confirmation: true,
                    irreversible: true,
                    can_undo: false,
                    disabled_reason: None,
                    expected_effects: vec![
                        "Terminates one running process by PID.".to_string(),
                        "Unsaved work in that process may be lost.".to_string(),
                        "This does not delete files.".to_string(),
                        "This does not remove startup entries.".to_string(),
                        "This does not block network access.".to_string(),
                    ],
                    warnings: vec![
                        "Do not terminate system processes.".to_string(),
                        "Terminating apps can cause data loss.".to_string(),
                    ],
                }
            }
            ActionKind::DisableStartup => startup_action_plan(
                request,
                STARTUP_DISABLE_PLANNED_REASON,
                true,
                false,
                vec![
                    "Would create local backup metadata before any future disable.".to_string(),
                    "Would mark one startup entry as disabled in a later package.".to_string(),
                    "Package 4D does not write the registry, modify services, or change scheduled tasks."
                        .to_string(),
                ],
                vec![
                    "Startup changes can affect app launch behavior after sign-in.".to_string(),
                    "This package only prepares the plan and backup model.".to_string(),
                ],
            ),
            ActionKind::RestoreStartup => startup_action_plan(
                request,
                STARTUP_RESTORE_PLANNED_REASON,
                false,
                request_has_startup_backup_available(&request),
                vec![
                    "Would use a local startup backup record in a later package.".to_string(),
                    "Would restore one startup entry if a matching backup is available.".to_string(),
                    "Package 4D does not write the registry, modify services, or change scheduled tasks."
                        .to_string(),
                ],
                vec![
                    "Restore requires a trusted backup record from before the disable action.".to_string(),
                    "This package only previews the restore plan.".to_string(),
                ],
            ),
            ActionKind::DetectFileLockers => planned_plan(
                request,
                ActionAvailability::Unsupported,
                "File locker detection is planned but not implemented in Package 4C.",
            ),
            _ => planned_plan(request, ActionAvailability::Planned, FUTURE_ACTION_REASON),
        }
    }

    pub fn classify_risk(kind: ActionKind) -> ActionRiskLevel {
        match kind {
            ActionKind::RevealPath | ActionKind::OpenWindowsSettings => ActionRiskLevel::Safe,
            ActionKind::DetectFileLockers
            | ActionKind::DisableStartup
            | ActionKind::RestoreStartup => ActionRiskLevel::Caution,
            ActionKind::KillProcess
            | ActionKind::BlockNetwork
            | ActionKind::UnblockNetwork
            | ActionKind::QuarantineFile
            | ActionKind::RestoreQuarantine
            | ActionKind::ScheduleDeleteOnReboot => ActionRiskLevel::Dangerous,
        }
    }

    pub fn plan_startup_action(
        target: StartupEntryTarget,
        action_kind: ActionKind,
        backup_available: bool,
    ) -> StartupActionPlan {
        let risk_level = Self::classify_risk(action_kind);
        match action_kind {
            ActionKind::DisableStartup => StartupActionPlan {
                action_kind,
                target,
                risk_level,
                requires_confirmation: true,
                backup_required: true,
                backup_available,
                expected_effects: vec![
                    "Would prepare backup metadata before a future disable.".to_string(),
                    "Would disable one startup entry only in a later implementation package."
                        .to_string(),
                ],
                warnings: vec![
                    STARTUP_DISABLE_PLANNED_REASON.to_string(),
                    "No registry, scheduled task, service, or startup folder modification is performed."
                        .to_string(),
                ],
                disabled_reason: Some(STARTUP_DISABLE_PLANNED_REASON.to_string()),
            },
            ActionKind::RestoreStartup => StartupActionPlan {
                action_kind,
                target,
                risk_level,
                requires_confirmation: true,
                backup_required: false,
                backup_available,
                expected_effects: vec![
                    "Would restore one startup entry from local backup metadata in a later package."
                        .to_string(),
                ],
                warnings: vec![
                    STARTUP_RESTORE_PLANNED_REASON.to_string(),
                    "Restore requires a matching trusted backup record.".to_string(),
                ],
                disabled_reason: Some(STARTUP_RESTORE_PLANNED_REASON.to_string()),
            },
            _ => StartupActionPlan {
                action_kind,
                target,
                risk_level,
                requires_confirmation: false,
                backup_required: false,
                backup_available,
                expected_effects: vec!["No startup action will be executed.".to_string()],
                warnings: vec!["Unsupported startup action kind.".to_string()],
                disabled_reason: Some("Unsupported startup action kind.".to_string()),
            },
        }
    }

    pub fn is_action_enabled(kind: ActionKind) -> bool {
        matches!(
            kind,
            ActionKind::RevealPath | ActionKind::OpenWindowsSettings | ActionKind::KillProcess
        )
    }
}

fn request_has_pid_metadata(request: &ActionRequest) -> bool {
    request
        .metadata_json
        .as_ref()
        .and_then(|metadata| metadata.get("pid"))
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .is_some()
}

fn request_has_startup_backup_available(request: &ActionRequest) -> bool {
    request
        .metadata_json
        .as_ref()
        .and_then(|metadata| metadata.get("backup_available"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn startup_action_plan(
    request: ActionRequest,
    reason: &str,
    backup_required: bool,
    backup_available: bool,
    expected_effects: Vec<String>,
    warnings: Vec<String>,
) -> ActionPlan {
    let mut request = request;
    let mut metadata = request
        .metadata_json
        .take()
        .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    if let Value::Object(object) = &mut metadata {
        object.insert("backup_required".to_string(), Value::Bool(backup_required));
        object.insert("backup_available".to_string(), Value::Bool(backup_available));
    }
    request.metadata_json = Some(metadata);

    ActionPlan {
        confirmation_title: "Startup Guard action planned".to_string(),
        confirmation_message: reason.to_string(),
        request,
        availability: ActionAvailability::Planned,
        requires_confirmation: true,
        irreversible: false,
        can_undo: false,
        disabled_reason: Some(reason.to_string()),
        expected_effects,
        warnings,
    }
}

fn planned_plan(
    request: ActionRequest,
    availability: ActionAvailability,
    reason: impl Into<String>,
) -> ActionPlan {
    let reason = reason.into();
    ActionPlan {
        confirmation_title: "Action unavailable".to_string(),
        confirmation_message: reason.clone(),
        request,
        availability,
        requires_confirmation: false,
        irreversible: true,
        can_undo: false,
        disabled_reason: Some(reason.clone()),
        expected_effects: vec!["No action will be executed in Package 4B.".to_string()],
        warnings: vec![reason],
    }
}

pub fn cancellation_result(request_id: impl Into<String>) -> ActionResult {
    let timestamp = now_timestamp();
    ActionResult {
        request_id: request_id.into(),
        kind: ActionKind::RevealPath,
        target: String::new(),
        started_at: timestamp.clone(),
        finished_at: timestamp,
        status: ActionStatus::Cancelled,
        message: "Action was cancelled before execution.".to_string(),
        error: None,
        metadata_json: None,
    }
}

fn generated_action_id() -> String {
    let counter = ACTION_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("action-{}-{counter}", timestamp_seconds())
}

pub fn now_timestamp() -> String {
    format!("unix:{}", timestamp_seconds())
}

fn timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_kind_serializes_as_snake_case() {
        let encoded = serde_json::to_string(&ActionKind::KillProcess).expect("kind serializes");

        assert_eq!(encoded, "\"kill_process\"");
        assert_eq!(
            serde_json::from_str::<ActionKind>(&encoded).expect("kind deserializes"),
            ActionKind::KillProcess
        );
    }

    #[test]
    fn classify_risk_for_safe_and_dangerous_actions() {
        assert_eq!(
            PolicyEngine::classify_risk(ActionKind::RevealPath),
            ActionRiskLevel::Safe
        );
        assert_eq!(
            PolicyEngine::classify_risk(ActionKind::KillProcess),
            ActionRiskLevel::Dangerous
        );
    }

    #[test]
    fn safe_action_requires_confirmation() {
        let request =
            ActionRequest::new(ActionKind::RevealPath, "C:\\Demo", "Demo folder", "files");
        let plan = PolicyEngine::plan_action(request);

        assert_eq!(plan.availability, ActionAvailability::Available);
        assert!(plan.requires_confirmation);
        assert!(!plan.irreversible);
    }

    #[test]
    fn dangerous_action_is_planned_not_executable() {
        let request = ActionRequest::new(ActionKind::BlockNetwork, "tcp:443", "TCP 443", "network");
        let plan = PolicyEngine::plan_action(request);

        assert_eq!(plan.availability, ActionAvailability::Planned);
        assert!(!plan.requires_confirmation);
        assert!(plan.disabled_reason.is_some());
    }

    #[test]
    fn action_result_serializes() {
        let request =
            ActionRequest::new(ActionKind::RevealPath, "C:\\Demo", "Demo folder", "files");
        let plan = PolicyEngine::plan_action(request);
        let result = ActionResult::from_plan(&plan, ActionStatus::DryRun, "Dry run only.");
        let encoded = serde_json::to_string(&result).expect("result serializes");

        assert!(encoded.contains("\"status\":\"dry_run\""));
        assert!(encoded.contains("\"kind\":\"reveal_path\""));
    }

    #[test]
    fn kill_process_requires_pid_metadata() {
        let request = ActionRequest::new(ActionKind::KillProcess, "42", "PID 42", "processes");
        let plan = PolicyEngine::plan_action(request);

        assert_eq!(plan.availability, ActionAvailability::Disabled);
        assert!(!plan.requires_confirmation);
        assert!(plan.disabled_reason.is_some());
    }

    #[test]
    fn kill_process_with_pid_metadata_requires_confirmation() {
        let mut request =
            ActionRequest::new(ActionKind::KillProcess, "42", "Demo process", "processes");
        request.metadata_json = Some(serde_json::json!({ "pid": 42, "process_name": "demo.exe" }));
        let plan = PolicyEngine::plan_action(request);

        assert_eq!(plan.request.risk_level, ActionRiskLevel::Dangerous);
        assert_eq!(plan.availability, ActionAvailability::RequiresConfirmation);
        assert!(plan.requires_confirmation);
        assert!(plan.irreversible);
        assert!(!plan.can_undo);
    }

    #[test]
    fn startup_backup_record_serializes() {
        let record = StartupBackupRecord {
            backup_id: "backup-1".to_string(),
            entry_id: "entry-1".to_string(),
            created_at: "unix:1".to_string(),
            source: "HKCU Run".to_string(),
            original_name: "Demo".to_string(),
            original_command: "demo.exe".to_string(),
            original_path: Some("C:\\Demo\\demo.exe".to_string()),
            original_enabled_state: "enabled".to_string(),
            restore_strategy: "restore_run_value".to_string(),
            metadata_json: Some(serde_json::json!({ "planned_only": true })),
        };

        let encoded = serde_json::to_string(&record).expect("record serializes");
        let decoded: StartupBackupRecord =
            serde_json::from_str(&encoded).expect("record deserializes");

        assert_eq!(decoded, record);
    }

    #[test]
    fn startup_actions_are_caution_and_planned() {
        let disable_request =
            ActionRequest::new(ActionKind::DisableStartup, "entry-1", "Demo", "startup");
        let restore_request =
            ActionRequest::new(ActionKind::RestoreStartup, "entry-1", "Demo", "startup");
        let disable_plan = PolicyEngine::plan_action(disable_request);
        let restore_plan = PolicyEngine::plan_action(restore_request);

        assert_eq!(disable_plan.request.risk_level, ActionRiskLevel::Caution);
        assert_eq!(restore_plan.request.risk_level, ActionRiskLevel::Caution);
        assert_eq!(disable_plan.availability, ActionAvailability::Planned);
        assert_eq!(restore_plan.availability, ActionAvailability::Planned);
        assert!(disable_plan.requires_confirmation);
        assert!(restore_plan.requires_confirmation);
        assert!(disable_plan.disabled_reason.is_some());
        assert!(restore_plan.disabled_reason.is_some());
    }

    #[test]
    fn startup_specific_plan_tracks_backup_requirements() {
        let target = StartupEntryTarget {
            entry_id: "entry-1".to_string(),
            name: "Demo".to_string(),
            source: "HKCU Run".to_string(),
            scope: "CurrentUser".to_string(),
            command: Some("demo.exe".to_string()),
            path: Some("C:\\Demo\\demo.exe".to_string()),
            enabled: Some(true),
            original_location: Some("HKCU Run".to_string()),
            metadata_json: None,
        };

        let disable = PolicyEngine::plan_startup_action(
            target.clone(),
            ActionKind::DisableStartup,
            false,
        );
        let restore = PolicyEngine::plan_startup_action(
            target,
            ActionKind::RestoreStartup,
            true,
        );

        assert!(disable.backup_required);
        assert!(!disable.backup_available);
        assert!(!restore.backup_required);
        assert!(restore.backup_available);
        assert!(disable.disabled_reason.is_some());
        assert!(restore.disabled_reason.is_some());
    }
}
