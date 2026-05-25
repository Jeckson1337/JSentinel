#![forbid(unsafe_code)]

use jsentinel_core::{
    CapabilityStatus, EventService, FileLockerInfo, NetworkConnectionInfo, ProcessInfo,
    ReadOnlyQueryResult, StartupEntryInfo,
};
use jsentinel_db::{DashboardSummary, EventQuery};
use jsentinel_events::{AccessEvent, EventId};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;

struct AppState {
    event_service: Mutex<EventService>,
}

#[derive(Debug, Serialize)]
struct ReadOnlyDiagnostics {
    app_version: &'static str,
    platform: &'static str,
    capabilities: Vec<CapabilityStatus>,
    process_count: usize,
    network_connection_count: usize,
    startup_entry_count: usize,
}

#[tauri::command]
fn jsentinel_get_events(
    state: tauri::State<'_, AppState>,
    query: EventQuery,
) -> Result<Vec<AccessEvent>, String> {
    let service = state
        .event_service
        .lock()
        .map_err(|_| "event service lock was poisoned".to_string())?;
    service.list_events(query).map_err(|error| error.to_string())
}

#[tauri::command]
fn jsentinel_get_event(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<Option<AccessEvent>, String> {
    let service = state
        .event_service
        .lock()
        .map_err(|_| "event service lock was poisoned".to_string())?;
    service
        .get_event(&EventId::new(id))
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn jsentinel_seed_mock_events(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    let service = state
        .event_service
        .lock()
        .map_err(|_| "event service lock was poisoned".to_string())?;
    service.seed_mock_events().map_err(|error| error.to_string())
}

#[tauri::command]
fn jsentinel_get_dashboard_summary(
    state: tauri::State<'_, AppState>,
) -> Result<DashboardSummary, String> {
    let service = state
        .event_service
        .lock()
        .map_err(|_| "event service lock was poisoned".to_string())?;
    service.dashboard_summary().map_err(|error| error.to_string())
}

#[tauri::command]
fn jsentinel_get_system_capabilities() -> Vec<CapabilityStatus> {
    jsentinel_windows::system_capabilities()
}

#[tauri::command]
fn jsentinel_list_processes() -> ReadOnlyQueryResult<ProcessInfo> {
    jsentinel_windows::list_processes()
}

#[tauri::command]
fn jsentinel_get_process_details(pid: u32) -> ReadOnlyQueryResult<ProcessInfo> {
    jsentinel_windows::get_process_details(pid)
}

#[tauri::command]
fn jsentinel_list_network_connections() -> ReadOnlyQueryResult<NetworkConnectionInfo> {
    jsentinel_windows::list_network_connections()
}

#[tauri::command]
fn jsentinel_list_startup_entries() -> ReadOnlyQueryResult<StartupEntryInfo> {
    jsentinel_windows::list_startup_entries()
}

#[tauri::command]
fn jsentinel_detect_file_lockers(path: String) -> ReadOnlyQueryResult<FileLockerInfo> {
    jsentinel_windows::detect_file_lockers(path)
}

#[tauri::command]
fn jsentinel_get_read_only_diagnostics() -> ReadOnlyDiagnostics {
    let processes = jsentinel_windows::list_processes();
    let network_connections = jsentinel_windows::list_network_connections();
    let startup_entries = jsentinel_windows::list_startup_entries();

    ReadOnlyDiagnostics {
        app_version: env!("CARGO_PKG_VERSION"),
        platform: std::env::consts::OS,
        capabilities: jsentinel_windows::system_capabilities(),
        process_count: processes.items.len(),
        network_connection_count: network_connections.items.len(),
        startup_entry_count: startup_entries.items.len(),
    }
}

fn main() {
    let database_path = dev_database_path();
    let event_service = EventService::initialize_storage(database_path)
        .expect("failed to initialize local JSentinel SQLite storage");

    tauri::Builder::default()
        .manage(AppState {
            event_service: Mutex::new(event_service),
        })
        .invoke_handler(tauri::generate_handler![
            jsentinel_get_events,
            jsentinel_get_event,
            jsentinel_seed_mock_events,
            jsentinel_get_dashboard_summary,
            jsentinel_get_system_capabilities,
            jsentinel_list_processes,
            jsentinel_get_process_details,
            jsentinel_list_network_connections,
            jsentinel_list_startup_entries,
            jsentinel_detect_file_lockers,
            jsentinel_get_read_only_diagnostics
        ])
        .run(tauri::generate_context!())
        .expect("failed to run JSentinel desktop UI");
}

fn dev_database_path() -> PathBuf {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    root.join(".jsentinel-dev").join("jsentinel.sqlite3")
}
