#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static EVENT_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub String);

impl EventId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn generated(prefix: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let counter = EVENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(format!("{prefix}-{nanos}-{counter}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventTimestamp(pub String);

impl EventTimestamp {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn now_utc_best_effort() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        Self(format!("unix:{seconds}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSeverity {
    Info,
    Warning,
    Critical,
}

impl EventSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for EventSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventSeverity {
    type Err = EventParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "critical" => Ok(Self::Critical),
            _ => Err(EventParseError::UnknownSeverity(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Process,
    Network,
    File,
    Startup,
    DeviceAccess,
    LockedFile,
    Security,
    System,
}

impl EventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Process => "process",
            Self::Network => "network",
            Self::File => "file",
            Self::Startup => "startup",
            Self::DeviceAccess => "device_access",
            Self::LockedFile => "locked_file",
            Self::Security => "security",
            Self::System => "system",
        }
    }
}

impl fmt::Display for EventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventKind {
    type Err = EventParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "process" => Ok(Self::Process),
            "network" => Ok(Self::Network),
            "file" => Ok(Self::File),
            "startup" => Ok(Self::Startup),
            "device_access" => Ok(Self::DeviceAccess),
            "locked_file" => Ok(Self::LockedFile),
            "security" => Ok(Self::Security),
            "system" => Ok(Self::System),
            _ => Err(EventParseError::UnknownKind(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSource {
    Mock,
    User,
    Core,
    WindowsBackend,
    LinuxBackend,
}

impl EventSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mock => "mock",
            Self::User => "user",
            Self::Core => "core",
            Self::WindowsBackend => "windows_backend",
            Self::LinuxBackend => "linux_backend",
        }
    }
}

impl fmt::Display for EventSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventSource {
    type Err = EventParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "mock" => Ok(Self::Mock),
            "user" => Ok(Self::User),
            "core" => Ok(Self::Core),
            "windows_backend" => Ok(Self::WindowsBackend),
            "linux_backend" => Ok(Self::LinuxBackend),
            _ => Err(EventParseError::UnknownSource(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessRef {
    pub pid: Option<u32>,
    pub name: String,
    pub path: Option<String>,
}

impl ProcessRef {
    pub fn new(pid: Option<u32>, name: impl Into<String>, path: Option<String>) -> Self {
        Self {
            pid,
            name: name.into(),
            path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessEvent {
    pub id: EventId,
    pub timestamp: EventTimestamp,
    pub kind: EventKind,
    pub severity: EventSeverity,
    pub source: EventSource,
    pub process: Option<ProcessRef>,
    pub title: String,
    pub summary: String,
    pub target: Option<String>,
    pub metadata_json: Option<Value>,
    pub confidence: Option<String>,
}

impl AccessEvent {
    pub fn mock(
        kind: EventKind,
        severity: EventSeverity,
        process: Option<ProcessRef>,
        title: impl Into<String>,
        summary: impl Into<String>,
        target: Option<String>,
        metadata_json: Option<Value>,
    ) -> Self {
        Self {
            id: EventId::generated(kind.as_str()),
            timestamp: EventTimestamp::now_utc_best_effort(),
            kind,
            severity,
            source: EventSource::Mock,
            process,
            title: title.into(),
            summary: summary.into(),
            target,
            metadata_json,
            confidence: Some("demo_only".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventParseError {
    UnknownSeverity(String),
    UnknownKind(String),
    UnknownSource(String),
}

impl fmt::Display for EventParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSeverity(value) => write!(formatter, "unknown event severity: {value}"),
            Self::UnknownKind(value) => write!(formatter, "unknown event kind: {value}"),
            Self::UnknownSource(value) => write!(formatter, "unknown event source: {value}"),
        }
    }
}

impl std::error::Error for EventParseError {}

pub fn mock_process_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::Process,
        EventSeverity::Info,
        Some(ProcessRef::new(
            Some(4242),
            "demo-browser.exe",
            Some("C:\\Program Files\\Demo Browser\\demo-browser.exe".to_string()),
        )),
        "Demo process observed",
        "A mock process event used to validate the local timeline.",
        Some("demo-browser.exe".to_string()),
        Some(json!({ "demo": true, "collector": "mock_provider" })),
    )
}

pub fn mock_network_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::Network,
        EventSeverity::Warning,
        Some(ProcessRef::new(Some(4242), "demo-browser.exe", None)),
        "Demo network connection",
        "A mock network event. No real network scan or outbound request occurred.",
        Some("example.invalid:443".to_string()),
        Some(json!({ "protocol": "tcp", "direction": "outbound", "demo": true })),
    )
}

pub fn mock_file_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::File,
        EventSeverity::Info,
        Some(ProcessRef::new(Some(3110), "demo-editor.exe", None)),
        "Demo file activity",
        "A mock file event. JSentinel did not watch or read a real file.",
        Some("C:\\Users\\Example\\Documents\\demo.txt".to_string()),
        Some(json!({ "operation": "metadata_only_demo", "demo": true })),
    )
}

pub fn mock_startup_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::Startup,
        EventSeverity::Warning,
        None,
        "Demo startup entry",
        "A mock startup entry used for UI and database validation only.",
        Some("Demo Helper".to_string()),
        Some(json!({ "source": "mock_startup_list", "demo": true })),
    )
}

pub fn mock_device_access_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::DeviceAccess,
        EventSeverity::Info,
        Some(ProcessRef::new(Some(9001), "demo-meeting.exe", None)),
        "Demo device access",
        "A mock camera/microphone style event. No real device monitoring is implemented.",
        Some("camera".to_string()),
        Some(json!({ "device": "camera", "demo": true })),
    )
}

pub fn mock_locked_file_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::LockedFile,
        EventSeverity::Warning,
        Some(ProcessRef::new(Some(777), "demo-sync.exe", None)),
        "Demo locked file",
        "A mock locked-file event. JSentinel did not inspect real file handles.",
        Some("C:\\Users\\Example\\Documents\\locked-demo.dat".to_string()),
        Some(json!({ "lock_state": "mocked", "demo": true })),
    )
}

pub fn mock_security_event() -> AccessEvent {
    AccessEvent::mock(
        EventKind::Security,
        EventSeverity::Critical,
        None,
        "Demo security attention item",
        "A mock attention item for validating critical severity rendering.",
        Some("Local policy demo".to_string()),
        Some(json!({ "attention": "demo_only", "demo": true })),
    )
}

pub fn mock_demo_events() -> Vec<AccessEvent> {
    vec![
        mock_process_event(),
        mock_network_event(),
        mock_file_event(),
        mock_startup_event(),
        mock_device_access_event(),
        mock_locked_file_event(),
        mock_security_event(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_event_with_snake_case_enums() {
        let event = mock_network_event();
        let encoded = serde_json::to_string(&event).expect("event should serialize");

        assert!(encoded.contains("\"kind\":\"network\""));
        assert!(encoded.contains("\"severity\":\"warning\""));
        assert!(encoded.contains("\"source\":\"mock\""));

        let decoded: AccessEvent = serde_json::from_str(&encoded).expect("event should deserialize");
        assert_eq!(decoded.kind, EventKind::Network);
        assert_eq!(decoded.source, EventSource::Mock);
    }

    #[test]
    fn mock_events_are_marked_as_mock() {
        for event in mock_demo_events() {
            assert_eq!(event.source, EventSource::Mock);
            assert_eq!(event.confidence.as_deref(), Some("demo_only"));
        }
    }
}
