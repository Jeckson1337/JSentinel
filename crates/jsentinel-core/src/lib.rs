#![forbid(unsafe_code)]

pub mod actions;

pub use actions::{
    allowed_windows_settings_uris, current_process_id, evaluate_kill_process_safety,
    is_allowed_windows_settings_uri, is_hard_denied_process_name, is_self_or_parent_process,
    is_windows_directory_path, is_windows_system_path, kill_process_target_from_request,
    settings_uri_from_request, validate_reveal_path, DefaultSafeActionAdapter,
    KillProcessSafetyCheck, KillProcessTarget, SafeActionAdapter, SafeActionError,
    SafeActionExecutor, ValidatedRevealPath,
};

use jsentinel_db::{init_db, DashboardSummary, Database, DbResult, EventQuery};
use jsentinel_events::{mock_demo_events, AccessEvent, EventId};
use jsentinel_policy::{ActionHistoryQuery, ActionResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformTarget {
    Windows,
    Linux,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    Planned,
    Stub,
    Ready,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentStatus {
    pub name: &'static str,
    pub state: ComponentState,
    pub notes: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafetyPrinciple {
    pub key: &'static str,
    pub description: &'static str,
}

pub fn foundation_principles() -> &'static [SafetyPrinciple] {
    &[
        SafetyPrinciple {
            key: "local_first",
            description: "JSentinel works locally by default and does not require cloud services.",
        },
        SafetyPrinciple {
            key: "no_telemetry",
            description: "The application must not collect telemetry, tracking IDs, or analytics.",
        },
        SafetyPrinciple {
            key: "confirm_dangerous_actions",
            description: "Potentially destructive or privileged actions require explicit confirmation.",
        },
    ]
}

pub struct EventService {
    database: Database,
}

impl EventService {
    pub fn initialize_storage(path: impl AsRef<Path>) -> DbResult<Self> {
        Ok(Self {
            database: init_db(path)?,
        })
    }

    pub fn list_events(&self, query: EventQuery) -> DbResult<Vec<AccessEvent>> {
        self.database.list_events(query)
    }

    pub fn get_event(&self, id: &EventId) -> DbResult<Option<AccessEvent>> {
        self.database.get_event(id)
    }

    pub fn seed_mock_events(&self) -> DbResult<usize> {
        self.database.seed_mock_events()
    }

    pub fn dashboard_summary(&self) -> DbResult<DashboardSummary> {
        self.database.dashboard_summary()
    }

    pub fn insert_action_history(&self, result: &ActionResult) -> DbResult<()> {
        self.database.insert_action_history(result)
    }

    pub fn list_action_history(&self, query: ActionHistoryQuery) -> DbResult<Vec<ActionResult>> {
        self.database.list_action_history(query)
    }

    pub fn get_action_history(&self, id: &str) -> DbResult<Option<ActionResult>> {
        self.database.get_action_history(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCapability {
    pub key: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilitySupportStatus {
    Supported,
    Partial,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemPlatform {
    Windows,
    Linux,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadOnlyBackendErrorKind {
    UnsupportedPlatform,
    PermissionDenied,
    Unavailable,
    ParseError,
    OsError,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadOnlyBackendError {
    pub kind: ReadOnlyBackendErrorKind,
    pub message: String,
}

impl ReadOnlyBackendError {
    pub fn new(kind: ReadOnlyBackendErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityStatus {
    pub id: String,
    pub label: String,
    pub supported: bool,
    pub status: CapabilitySupportStatus,
    pub requires_admin: bool,
    pub data_source: String,
    pub read_only: bool,
    pub limitation: Option<String>,
}

impl CapabilityStatus {
    pub fn supported(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            supported: true,
            status: CapabilitySupportStatus::Supported,
            requires_admin: false,
            data_source: "local_os_snapshot".to_string(),
            read_only: true,
            limitation: None,
        }
    }

    pub fn partial(
        id: impl Into<String>,
        label: impl Into<String>,
        limitation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            supported: true,
            status: CapabilitySupportStatus::Partial,
            requires_admin: false,
            data_source: "local_os_snapshot".to_string(),
            read_only: true,
            limitation: Some(limitation.into()),
        }
    }

    pub fn with_data_source(mut self, data_source: impl Into<String>) -> Self {
        self.data_source = data_source.into();
        self
    }

    pub fn with_limitation(mut self, limitation: impl Into<String>) -> Self {
        self.limitation = Some(limitation.into());
        self
    }

    pub fn admin_may_improve_results(mut self) -> Self {
        self.limitation = Some(match self.limitation {
            Some(existing) => format!(
                "{existing} Admin rights may improve visibility for protected data."
            ),
            None => "Admin rights may improve visibility for protected data.".to_string(),
        });
        self
    }

    pub fn unsupported(
        id: impl Into<String>,
        label: impl Into<String>,
        limitation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            supported: false,
            status: CapabilitySupportStatus::Unsupported,
            requires_admin: false,
            data_source: "not_available".to_string(),
            read_only: true,
            limitation: Some(limitation.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadOnlyQueryResult<T> {
    pub platform: SystemPlatform,
    pub provider: String,
    pub capability: CapabilityStatus,
    pub items: Vec<T>,
    pub error: Option<ReadOnlyBackendError>,
}

impl<T> ReadOnlyQueryResult<T> {
    pub fn unsupported(
        provider: impl Into<String>,
        capability_id: impl Into<String>,
        capability_label: impl Into<String>,
        limitation: impl Into<String>,
    ) -> Self {
        Self {
            platform: SystemPlatform::Unsupported,
            provider: provider.into(),
            capability: CapabilityStatus::unsupported(capability_id, capability_label, limitation),
            items: Vec::new(),
            error: Some(ReadOnlyBackendError::new(
                ReadOnlyBackendErrorKind::UnsupportedPlatform,
                "This read-only backend is unsupported on the current platform.",
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: Option<String>,
    pub parent_pid: Option<u32>,
    pub command_line: Option<String>,
    pub started_at: Option<String>,
    pub owner: Option<String>,
    pub source: String,
    pub confidence: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkConnectionInfo {
    pub protocol: String,
    pub local_addr: String,
    pub local_port: Option<u16>,
    pub remote_addr: Option<String>,
    pub remote_port: Option<u16>,
    pub state: Option<String>,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub process_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupEntryInfo {
    pub id: String,
    pub name: String,
    pub source: String,
    pub command: Option<String>,
    pub path: Option<String>,
    pub enabled: Option<bool>,
    pub scope: String,
    pub publisher: Option<String>,
    pub risk: Option<String>,
    pub limitation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileLockerInfo {
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub process_path: Option<String>,
    pub path: String,
    pub confidence: String,
    pub limitation: Option<String>,
}

pub trait ReadOnlySystemProvider {
    fn provider_name(&self) -> &'static str;
    fn platform(&self) -> SystemPlatform;
    fn capabilities(&self) -> Vec<CapabilityStatus>;
    fn list_processes(&self) -> ReadOnlyQueryResult<ProcessInfo>;
    fn get_process_details(&self, pid: u32) -> ReadOnlyQueryResult<ProcessInfo>;
    fn list_network_connections(&self) -> ReadOnlyQueryResult<NetworkConnectionInfo>;
    fn list_startup_entries(&self) -> ReadOnlyQueryResult<StartupEntryInfo>;
    fn detect_file_lockers(&self, path: &str) -> ReadOnlyQueryResult<FileLockerInfo>;
    fn collect_snapshot_events(&self) -> Vec<AccessEvent>;
}

pub struct MockSystemProvider;

impl ReadOnlySystemProvider for MockSystemProvider {
    fn provider_name(&self) -> &'static str {
        "mock"
    }

    fn platform(&self) -> SystemPlatform {
        SystemPlatform::Unsupported
    }

    fn capabilities(&self) -> Vec<CapabilityStatus> {
        vec![CapabilityStatus::unsupported(
            "mock_snapshot_events",
            "Mock snapshot events",
            "Returns demo-only events without reading real OS state.",
        )]
    }

    fn list_processes(&self) -> ReadOnlyQueryResult<ProcessInfo> {
        ReadOnlyQueryResult::unsupported(
            self.provider_name(),
            "process_inventory",
            "Process inventory",
            "Mock provider does not inspect real processes.",
        )
    }

    fn get_process_details(&self, _pid: u32) -> ReadOnlyQueryResult<ProcessInfo> {
        ReadOnlyQueryResult::unsupported(
            self.provider_name(),
            "process_details",
            "Process details",
            "Mock provider does not inspect real processes.",
        )
    }

    fn list_network_connections(&self) -> ReadOnlyQueryResult<NetworkConnectionInfo> {
        ReadOnlyQueryResult::unsupported(
            self.provider_name(),
            "network_connections",
            "Network connections",
            "Mock provider does not inspect real network state.",
        )
    }

    fn list_startup_entries(&self) -> ReadOnlyQueryResult<StartupEntryInfo> {
        ReadOnlyQueryResult::unsupported(
            self.provider_name(),
            "startup_entries",
            "Startup entries",
            "Mock provider does not inspect startup sources.",
        )
    }

    fn detect_file_lockers(&self, path: &str) -> ReadOnlyQueryResult<FileLockerInfo> {
        ReadOnlyQueryResult {
            platform: self.platform(),
            provider: self.provider_name().to_string(),
            capability: CapabilityStatus::unsupported(
                "file_lockers",
                "File locker detection",
                "Mock provider does not inspect file handles.",
            ),
            items: vec![FileLockerInfo {
                pid: None,
                process_name: None,
                process_path: None,
                path: path.to_string(),
                confidence: "unsupported".to_string(),
                limitation: Some("No handle inspection was performed.".to_string()),
            }],
            error: None,
        }
    }

    fn collect_snapshot_events(&self) -> Vec<AccessEvent> {
        mock_demo_events()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CapabilityStatus, CapabilitySupportStatus, MockSystemProvider, ProcessInfo,
        ReadOnlyBackendError, ReadOnlyBackendErrorKind, ReadOnlySystemProvider, SystemPlatform,
    };

    #[test]
    fn capability_status_serializes() {
        let capability = CapabilityStatus::supported("process_inventory", "Process inventory");
        let json = serde_json::to_string(&capability).expect("capability should serialize");

        assert!(json.contains("process_inventory"));
        assert!(json.contains("\"supported\":true"));
        assert!(json.contains("\"status\":\"supported\""));
        assert!(json.contains("\"read_only\":true"));
    }

    #[test]
    fn partial_capability_status_serializes() {
        let capability = CapabilityStatus::partial(
            "startup_entries",
            "Startup entries",
            "Scheduled tasks are not parsed yet.",
        );
        let json = serde_json::to_string(&capability).expect("capability should serialize");
        let restored: CapabilityStatus =
            serde_json::from_str(&json).expect("capability should deserialize");

        assert_eq!(restored.status, CapabilitySupportStatus::Partial);
        assert!(restored.supported);
        assert!(restored.read_only);
    }

    #[test]
    fn read_only_backend_error_serializes() {
        let error = ReadOnlyBackendError::new(
            ReadOnlyBackendErrorKind::PermissionDenied,
            "Some fields are unavailable.",
        );
        let json = serde_json::to_string(&error).expect("error should serialize");

        assert!(json.contains("permission_denied"));
    }

    #[test]
    fn process_info_round_trips() {
        let process = ProcessInfo {
            pid: 42,
            name: "demo.exe".to_string(),
            path: Some("C:\\Demo\\demo.exe".to_string()),
            parent_pid: Some(1),
            command_line: Some("demo.exe --read-only".to_string()),
            started_at: Some("2026-01-01T00:00:00Z".to_string()),
            owner: None,
            source: "windows_backend".to_string(),
            confidence: "best_effort".to_string(),
            limitations: vec!["Some fields may be unavailable without elevation.".to_string()],
        };

        let json = serde_json::to_string(&process).expect("process should serialize");
        let restored: ProcessInfo =
            serde_json::from_str(&json).expect("process should deserialize");

        assert_eq!(restored, process);
    }

    #[test]
    fn mock_provider_reports_unsupported_read_only_capability() {
        let provider = MockSystemProvider;
        let result = provider.list_processes();

        assert_eq!(provider.platform(), SystemPlatform::Unsupported);
        assert!(!result.capability.supported);
        assert!(result.items.is_empty());
    }
}
