#![forbid(unsafe_code)]

use jsentinel_db::{init_db, DashboardSummary, Database, DbResult, EventQuery};
use jsentinel_events::{mock_demo_events, AccessEvent, EventId};
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCapability {
    pub key: String,
    pub description: String,
}

pub trait ReadOnlySystemProvider {
    fn provider_name(&self) -> &'static str;
    fn capabilities(&self) -> Vec<ProviderCapability>;
    fn collect_snapshot_events(&self) -> Vec<AccessEvent>;
}

pub struct MockSystemProvider;

impl ReadOnlySystemProvider for MockSystemProvider {
    fn provider_name(&self) -> &'static str {
        "mock"
    }

    fn capabilities(&self) -> Vec<ProviderCapability> {
        vec![ProviderCapability {
            key: "mock_snapshot_events".to_string(),
            description: "Returns demo-only events without reading real OS state.".to_string(),
        }]
    }

    fn collect_snapshot_events(&self) -> Vec<AccessEvent> {
        mock_demo_events()
    }
}
