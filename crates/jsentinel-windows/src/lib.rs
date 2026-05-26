#![deny(unsafe_op_in_unsafe_fn)]

use jsentinel_core::{
    evaluate_kill_process_safety, CapabilityStatus, FileLockerInfo, KillProcessSafetyCheck,
    KillProcessTarget, NetworkConnectionInfo, ProcessInfo, ReadOnlyBackendError,
    ReadOnlyBackendErrorKind, ReadOnlyQueryResult, SafeActionAdapter, SafeActionError,
    StartupEntryInfo, SystemPlatform, ValidatedRevealPath,
};
use serde::Deserialize;

const PROVIDER_NAME: &str = "windows_read_only";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsCapability {
    ProcessInventory,
    NetworkInventory,
    StartupInventory,
    EventCollection,
    KillProcess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsBackendPlan {
    pub primary_platform: bool,
    pub requires_kernel_driver: bool,
    pub implemented: bool,
    pub planned_capabilities: &'static [WindowsCapability],
}

pub fn plan() -> WindowsBackendPlan {
    WindowsBackendPlan {
        primary_platform: true,
        requires_kernel_driver: false,
        implemented: true,
        planned_capabilities: &[
            WindowsCapability::ProcessInventory,
            WindowsCapability::NetworkInventory,
            WindowsCapability::StartupInventory,
            WindowsCapability::EventCollection,
            WindowsCapability::KillProcess,
        ],
    }
}

pub fn system_capabilities() -> Vec<CapabilityStatus> {
    if !cfg!(windows) {
        return unsupported_capabilities("Windows read-only backend is only available on Windows.");
    }

    vec![
        CapabilityStatus::partial(
            "process_inventory",
            "Process inventory",
            "Executable path, command line, owner, and start time are best-effort and may be unavailable.",
        )
        .with_data_source("Win32_Process")
        .admin_may_improve_results(),
        CapabilityStatus::partial(
            "process_details",
            "Process details",
            "Process details are derived from the read-only process snapshot and may omit protected fields.",
        )
        .with_data_source("Win32_Process")
        .admin_may_improve_results(),
        CapabilityStatus::partial(
            "network_connections",
            "Network connections",
            "TCP/UDP data is a point-in-time snapshot. Process mapping is best-effort.",
        )
        .with_data_source("Get-NetTCPConnection/Get-NetUDPEndpoint"),
        CapabilityStatus::partial(
            "startup_entries",
            "Startup entries",
            "Registry Run keys and Startup folders are read best-effort; scheduled tasks are not parsed yet.",
        )
        .with_data_source("Registry Run keys and Startup folders"),
        CapabilityStatus::unsupported(
            "file_lockers",
            "File locker detection",
            "Restart Manager based locker detection is planned; no handles are inspected or closed in this package.",
        )
        .with_data_source("not_implemented"),
        CapabilityStatus::partial(
            "kill_process",
            "Kill process",
            "Terminates one non-protected process by PID after confirmation and safety checks. No process tree kill.",
        )
        .with_data_source("OpenProcess/TerminateProcess"),
    ]
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WindowsActionAdapter;

impl SafeActionAdapter for WindowsActionAdapter {
    fn reveal_path(&self, target: &ValidatedRevealPath) -> Result<(), SafeActionError> {
        let adapter = jsentinel_core::DefaultSafeActionAdapter;
        adapter.reveal_path(target)
    }

    fn open_windows_settings(&self, uri: &str) -> Result<(), SafeActionError> {
        let adapter = jsentinel_core::DefaultSafeActionAdapter;
        adapter.open_windows_settings(uri)
    }

    fn precheck_kill_process(&self, target: &KillProcessTarget) -> KillProcessSafetyCheck {
        precheck_kill_process(target.pid)
    }

    fn kill_process(
        &self,
        target: &KillProcessTarget,
    ) -> Result<KillProcessSafetyCheck, SafeActionError> {
        kill_process(target.pid)
    }
}

pub fn precheck_kill_process(pid: u32) -> KillProcessSafetyCheck {
    #[cfg(windows)]
    {
        let details = get_process_details(pid);
        let Some(process) = details.items.into_iter().next() else {
            return KillProcessSafetyCheck::denied(format!(
                "Process {pid} was not found or disappeared before precheck."
            ));
        };

        let current_pid = std::process::id();
        let current_parent_pid = current_process_parent_pid();
        let target = kill_target_from_process(process);
        evaluate_kill_process_safety(&target, current_pid, current_parent_pid)
    }

    #[cfg(not(windows))]
    {
        let _ = pid;
        KillProcessSafetyCheck::denied("Kill process is unsupported on this platform.")
    }
}

pub fn kill_process(pid: u32) -> Result<KillProcessSafetyCheck, SafeActionError> {
    #[cfg(windows)]
    {
        let check = precheck_kill_process(pid);
        if !check.allowed {
            return Ok(check);
        }

        terminate_process_once(pid)?;
        Ok(check)
    }

    #[cfg(not(windows))]
    {
        let _ = pid;
        Err(SafeActionError::UnsupportedPlatform(
            "Kill process is unsupported on this platform.".to_string(),
        ))
    }
}

pub fn list_processes() -> ReadOnlyQueryResult<ProcessInfo> {
    #[cfg(windows)]
    {
        let capability = CapabilityStatus::partial(
            "process_inventory",
            "Process inventory",
            "Executable path, command line, owner, and start time are best-effort and may be unavailable.",
        )
        .with_data_source("Win32_Process")
        .admin_may_improve_results();

        match powershell_json(PROCESS_QUERY) {
            Ok(value) => ReadOnlyQueryResult {
                platform: SystemPlatform::Windows,
                provider: PROVIDER_NAME.to_string(),
                capability,
                items: parse_processes(&value),
                error: None,
            },
            Err(error) => query_error(
                "process_inventory",
                "Process inventory",
                format!("Failed to query Windows process inventory: {error}"),
            ),
        }
    }

    #[cfg(not(windows))]
    {
        ReadOnlyQueryResult::unsupported(
            PROVIDER_NAME,
            "process_inventory",
            "Process inventory",
            "Windows process inventory is unsupported on this platform.",
        )
    }
}

pub fn get_process_details(pid: u32) -> ReadOnlyQueryResult<ProcessInfo> {
    #[cfg(windows)]
    {
        let mut result = list_processes();
        result.capability.id = "process_details".to_string();
        result.capability.label = "Process details".to_string();
        result.items.retain(|process| process.pid == pid);
        if result.items.is_empty() {
            result.capability.limitation = Some(format!(
                "Process {pid} was not found or could not be read at snapshot time."
            ));
        }
        result
    }

    #[cfg(not(windows))]
    {
        let _ = pid;
        ReadOnlyQueryResult::unsupported(
            PROVIDER_NAME,
            "process_details",
            "Process details",
            "Windows process details are unsupported on this platform.",
        )
    }
}

pub fn list_network_connections() -> ReadOnlyQueryResult<NetworkConnectionInfo> {
    #[cfg(windows)]
    {
        let capability = CapabilityStatus::partial(
            "network_connections",
            "Network connections",
            "Connection data is a point-in-time read-only snapshot. Process mapping is best-effort.",
        )
        .with_data_source("Get-NetTCPConnection/Get-NetUDPEndpoint");
        let processes = list_processes().items;

        match powershell_json(NETWORK_QUERY) {
            Ok(value) => ReadOnlyQueryResult {
                platform: SystemPlatform::Windows,
                provider: PROVIDER_NAME.to_string(),
                capability,
                items: parse_network_connections(&value, &processes),
                error: None,
            },
            Err(error) => query_error(
                "network_connections",
                "Network connections",
                format!("Failed to query Windows network connections: {error}"),
            ),
        }
    }

    #[cfg(not(windows))]
    {
        ReadOnlyQueryResult::unsupported(
            PROVIDER_NAME,
            "network_connections",
            "Network connections",
            "Windows network connection inventory is unsupported on this platform.",
        )
    }
}

pub fn list_startup_entries() -> ReadOnlyQueryResult<StartupEntryInfo> {
    #[cfg(windows)]
    {
        let capability = CapabilityStatus::partial(
            "startup_entries",
            "Startup entries",
            "Registry Run keys and Startup folders are read-only best-effort; scheduled tasks are not parsed yet.",
        )
        .with_data_source("Registry Run keys and Startup folders");

        match powershell_json(STARTUP_QUERY) {
            Ok(value) => ReadOnlyQueryResult {
                platform: SystemPlatform::Windows,
                provider: PROVIDER_NAME.to_string(),
                capability,
                items: parse_startup_entries(&value),
                error: None,
            },
            Err(error) => query_error(
                "startup_entries",
                "Startup entries",
                format!("Failed to query Windows startup entries: {error}"),
            ),
        }
    }

    #[cfg(not(windows))]
    {
        ReadOnlyQueryResult::unsupported(
            PROVIDER_NAME,
            "startup_entries",
            "Startup entries",
            "Windows startup inventory is unsupported on this platform.",
        )
    }
}

pub fn detect_file_lockers(path: impl AsRef<str>) -> ReadOnlyQueryResult<FileLockerInfo> {
    let path = path.as_ref().to_string();
    ReadOnlyQueryResult {
        platform: if cfg!(windows) {
            SystemPlatform::Windows
        } else {
            SystemPlatform::Unsupported
        },
        provider: PROVIDER_NAME.to_string(),
        capability: CapabilityStatus::unsupported(
            "file_lockers",
            "File locker detection",
            "Best-effort Restart Manager support is planned, but this package does not inspect or close handles.",
        ),
        items: vec![FileLockerInfo {
            pid: None,
            process_name: None,
            process_path: None,
            path,
            confidence: "unsupported".to_string(),
            limitation: Some(
                "No file handle inspection, unlock, process termination, or delete-on-reboot action was performed."
                    .to_string(),
            ),
        }],
        error: None,
    }
}

fn kill_target_from_process(process: ProcessInfo) -> KillProcessTarget {
    KillProcessTarget {
        pid: process.pid,
        process_name: Some(process.name),
        process_path: process.path,
        command_line: process.command_line,
    }
}

#[cfg(windows)]
fn current_process_parent_pid() -> Option<u32> {
    let current_pid = std::process::id();
    list_processes()
        .items
        .into_iter()
        .find(|process| process.pid == current_pid)
        .and_then(|process| process.parent_pid)
}

#[cfg(windows)]
fn terminate_process_once(pid: u32) -> Result<(), SafeActionError> {
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError};
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
    };

    // SAFETY: The PID was re-queried and safety-checked immediately before this call.
    // We request only terminate/query rights, call TerminateProcess once, and close only
    // the handle returned by OpenProcess. No process tree, retry loop, or handle unlock is used.
    let handle = unsafe {
        OpenProcess(
            PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION,
            0,
            pid,
        )
    };
    if handle.is_null() {
        return Err(SafeActionError::OsError(format!(
            "OpenProcess failed for PID {pid}: Windows error {}",
            unsafe { GetLastError() }
        )));
    }

    // SAFETY: `handle` is a valid process handle returned by OpenProcess above.
    let terminate_result = unsafe { TerminateProcess(handle, 1) };
    let last_error = if terminate_result == 0 {
        // SAFETY: GetLastError reads thread-local OS error state.
        Some(unsafe { GetLastError() })
    } else {
        None
    };
    // SAFETY: `handle` is closed exactly once by this function.
    unsafe {
        CloseHandle(handle);
    }

    if let Some(error) = last_error {
        Err(SafeActionError::OsError(format!(
            "TerminateProcess failed for PID {pid}: Windows error {error}"
        )))
    } else {
        Ok(())
    }
}

#[cfg(windows)]
const PROCESS_QUERY: &str = r#"
$ErrorActionPreference = 'SilentlyContinue'
Get-CimInstance Win32_Process |
  Select-Object ProcessId, ParentProcessId, Name, ExecutablePath, CommandLine, CreationDate |
  ConvertTo-Json -Depth 4 -Compress
"#;

#[cfg(windows)]
const NETWORK_QUERY: &str = r#"
$ErrorActionPreference = 'SilentlyContinue'
$tcp = Get-NetTCPConnection |
  Select-Object @{Name='Protocol';Expression={'TCP'}}, LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess
$udp = Get-NetUDPEndpoint |
  Select-Object @{Name='Protocol';Expression={'UDP'}}, LocalAddress, LocalPort, @{Name='RemoteAddress';Expression={$null}}, @{Name='RemotePort';Expression={$null}}, @{Name='State';Expression={$null}}, OwningProcess
@($tcp + $udp) | ConvertTo-Json -Depth 4 -Compress
"#;

#[cfg(windows)]
const STARTUP_QUERY: &str = r#"
$ErrorActionPreference = 'SilentlyContinue'
$entries = @()
$registrySources = @(
  @{ Path = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'; Source = 'HKCU Run'; Scope = 'CurrentUser' },
  @{ Path = 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Run'; Source = 'HKLM Run'; Scope = 'LocalMachine' },
  @{ Path = 'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run'; Source = 'HKLM Wow6432Node Run'; Scope = 'LocalMachine' }
)
foreach ($source in $registrySources) {
  $item = Get-ItemProperty -Path $source.Path
  if ($null -ne $item) {
    foreach ($property in $item.PSObject.Properties) {
      if ($property.Name -notmatch '^PS') {
        $entries += [PSCustomObject]@{
          Id = "$($source.Source):$($property.Name)"
          Name = $property.Name
          Source = $source.Source
          Command = [string]$property.Value
          Path = $null
          Enabled = $true
          Scope = $source.Scope
          Limitation = 'Read-only registry Run key snapshot.'
        }
      }
    }
  }
}
$folders = @(
  @{ Path = [Environment]::GetFolderPath('Startup'); Source = 'Current user Startup folder'; Scope = 'CurrentUser' },
  @{ Path = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Startup"; Source = 'All users Startup folder'; Scope = 'AllUsers' }
)
foreach ($folder in $folders) {
  if (Test-Path $folder.Path) {
    Get-ChildItem -LiteralPath $folder.Path -File | ForEach-Object {
      $entries += [PSCustomObject]@{
        Id = "$($folder.Source):$($_.Name)"
        Name = $_.BaseName
        Source = $folder.Source
        Command = $null
        Path = $_.FullName
        Enabled = $true
        Scope = $folder.Scope
        Limitation = 'Read-only Startup folder snapshot.'
      }
    }
  }
}
$entries | ConvertTo-Json -Depth 4 -Compress
"#;

#[cfg(windows)]
fn powershell_json(script: &str) -> Result<serde_json::Value, String> {
    use std::process::Command;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|error| error.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return Ok(serde_json::Value::Array(Vec::new()));
    }

    serde_json::from_str(trimmed).map_err(|error| error.to_string())
}

fn unsupported_capabilities(limitation: &str) -> Vec<CapabilityStatus> {
    vec![
        CapabilityStatus::unsupported("process_inventory", "Process inventory", limitation),
        CapabilityStatus::unsupported("process_details", "Process details", limitation),
        CapabilityStatus::unsupported("network_connections", "Network connections", limitation),
        CapabilityStatus::unsupported("startup_entries", "Startup entries", limitation),
        CapabilityStatus::unsupported("file_lockers", "File locker detection", limitation),
        CapabilityStatus::unsupported("kill_process", "Kill process", limitation),
    ]
}

fn query_error<T>(
    capability_id: impl Into<String>,
    capability_label: impl Into<String>,
    limitation: impl Into<String>,
) -> ReadOnlyQueryResult<T> {
    let limitation = limitation.into();
    ReadOnlyQueryResult {
        platform: if cfg!(windows) {
            SystemPlatform::Windows
        } else {
            SystemPlatform::Unsupported
        },
        provider: PROVIDER_NAME.to_string(),
        capability: CapabilityStatus::unsupported(
            capability_id,
            capability_label,
            limitation.clone(),
        )
        .with_data_source("local_os_snapshot"),
        items: Vec::new(),
        error: Some(classify_backend_error(&limitation)),
    }
}

fn classify_backend_error(message: &str) -> ReadOnlyBackendError {
    let lowered = message.to_lowercase();
    let kind = if lowered.contains("access is denied") || lowered.contains("permission") {
        ReadOnlyBackendErrorKind::PermissionDenied
    } else if lowered.contains("not recognized") || lowered.contains("not found") {
        ReadOnlyBackendErrorKind::Unavailable
    } else if lowered.contains("json") || lowered.contains("expected") {
        ReadOnlyBackendErrorKind::ParseError
    } else if lowered.is_empty() {
        ReadOnlyBackendErrorKind::Unknown
    } else {
        ReadOnlyBackendErrorKind::OsError
    };

    ReadOnlyBackendError::new(kind, message)
}

fn parse_json_array(value: &serde_json::Value) -> Vec<serde_json::Value> {
    match value {
        serde_json::Value::Array(items) => items.clone(),
        serde_json::Value::Null => Vec::new(),
        other => vec![other.clone()],
    }
}

fn parse_processes(value: &serde_json::Value) -> Vec<ProcessInfo> {
    parse_json_array(value)
        .into_iter()
        .filter_map(|item| serde_json::from_value::<RawProcess>(item).ok())
        .filter_map(|raw| {
            let pid = raw.process_id?;
            Some(ProcessInfo {
                pid,
                name: raw.name.unwrap_or_else(|| format!("pid-{pid}")),
                path: raw.executable_path.filter(|value| !value.trim().is_empty()),
                parent_pid: raw.parent_process_id,
                command_line: raw.command_line.filter(|value| !value.trim().is_empty()),
                started_at: raw.creation_date.filter(|value| !value.trim().is_empty()),
                owner: None,
                source: "windows_backend".to_string(),
                confidence: "best_effort".to_string(),
                limitations: vec![
                    "Read-only Win32_Process snapshot; protected process details may be unavailable."
                        .to_string(),
                ],
            })
        })
        .collect()
}

fn parse_network_connections(
    value: &serde_json::Value,
    processes: &[ProcessInfo],
) -> Vec<NetworkConnectionInfo> {
    parse_json_array(value)
        .into_iter()
        .filter_map(|item| serde_json::from_value::<RawNetworkConnection>(item).ok())
        .map(|raw| {
            let process = raw
                .owning_process
                .and_then(|pid| processes.iter().find(|process| process.pid == pid));

            NetworkConnectionInfo {
                protocol: raw.protocol.unwrap_or_else(|| "unknown".to_string()),
                local_addr: raw.local_address.unwrap_or_else(|| "unknown".to_string()),
                local_port: raw.local_port.and_then(to_u16),
                remote_addr: raw.remote_address.filter(|value| !value.trim().is_empty()),
                remote_port: raw.remote_port.and_then(to_u16),
                state: raw.state.filter(|value| !value.trim().is_empty()),
                pid: raw.owning_process,
                process_name: process.map(|process| process.name.clone()),
                process_path: process.and_then(|process| process.path.clone()),
            }
        })
        .collect()
}

fn parse_startup_entries(value: &serde_json::Value) -> Vec<StartupEntryInfo> {
    parse_json_array(value)
        .into_iter()
        .filter_map(|item| serde_json::from_value::<RawStartupEntry>(item).ok())
        .filter_map(|raw| {
            let id = raw.id?;
            Some(StartupEntryInfo {
                id,
                name: raw.name.unwrap_or_else(|| "Startup entry".to_string()),
                source: raw.source.unwrap_or_else(|| "Unknown".to_string()),
                command: raw.command.filter(|value| !value.trim().is_empty()),
                path: raw.path.filter(|value| !value.trim().is_empty()),
                enabled: raw.enabled,
                scope: raw.scope.unwrap_or_else(|| "Unknown".to_string()),
                publisher: None,
                risk: Some("unknown".to_string()),
                limitation: raw.limitation,
            })
        })
        .collect()
}

fn to_u16(value: u32) -> Option<u16> {
    u16::try_from(value).ok()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawProcess {
    process_id: Option<u32>,
    parent_process_id: Option<u32>,
    name: Option<String>,
    executable_path: Option<String>,
    command_line: Option<String>,
    creation_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawNetworkConnection {
    protocol: Option<String>,
    local_address: Option<String>,
    local_port: Option<u32>,
    remote_address: Option<String>,
    remote_port: Option<u32>,
    state: Option<String>,
    owning_process: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawStartupEntry {
    id: Option<String>,
    name: Option<String>,
    source: Option<String>,
    command: Option<String>,
    path: Option<String>,
    enabled: Option<bool>,
    scope: Option<String>,
    limitation: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        classify_backend_error, detect_file_lockers, parse_network_connections, parse_processes,
        parse_startup_entries, precheck_kill_process, system_capabilities,
    };
    use jsentinel_core::{CapabilitySupportStatus, ReadOnlyBackendErrorKind};
    use serde_json::json;

    #[test]
    fn parses_process_snapshot() {
        let value = json!({
            "ProcessId": 100,
            "ParentProcessId": 4,
            "Name": "demo.exe",
            "ExecutablePath": "C:\\Demo\\demo.exe",
            "CommandLine": "demo.exe --flag",
            "CreationDate": "20260101000000.000000+000"
        });

        let processes = parse_processes(&value);

        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].pid, 100);
        assert_eq!(processes[0].name, "demo.exe");
    }

    #[test]
    fn parses_network_snapshot_and_maps_process() {
        let processes = parse_processes(&json!({
            "ProcessId": 100,
            "Name": "demo.exe"
        }));
        let value = json!({
            "Protocol": "TCP",
            "LocalAddress": "127.0.0.1",
            "LocalPort": 5000,
            "RemoteAddress": "127.0.0.1",
            "RemotePort": 443,
            "State": "Established",
            "OwningProcess": 100
        });

        let connections = parse_network_connections(&value, &processes);

        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].protocol, "TCP");
        assert_eq!(connections[0].process_name.as_deref(), Some("demo.exe"));
    }

    #[test]
    fn parses_startup_snapshot() {
        let value = json!({
            "Id": "HKCU Run:Demo",
            "Name": "Demo",
            "Source": "HKCU Run",
            "Command": "demo.exe",
            "Path": null,
            "Enabled": true,
            "Scope": "CurrentUser",
            "Limitation": "Read-only registry Run key snapshot."
        });

        let entries = parse_startup_entries(&value);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source, "HKCU Run");
        assert_eq!(entries[0].enabled, Some(true));
    }

    #[test]
    fn file_locker_detection_is_honest_unsupported_contract() {
        let result = detect_file_lockers("C:\\Demo\\locked.txt");

        assert!(!result.capability.supported);
        assert_eq!(result.capability.status, CapabilitySupportStatus::Unsupported);
        assert!(result.capability.read_only);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].confidence, "unsupported");
    }

    #[test]
    fn capabilities_include_read_only_metadata() {
        let capabilities = system_capabilities();

        assert!(capabilities.iter().all(|capability| capability.read_only));
        assert!(capabilities.iter().all(|capability| !capability.data_source.is_empty()));
    }

    #[test]
    fn classifies_permission_denied_error() {
        let error = classify_backend_error("Access is denied");

        assert_eq!(error.kind, ReadOnlyBackendErrorKind::PermissionDenied);
    }

    #[test]
    fn kill_precheck_denies_pid_zero() {
        let check = precheck_kill_process(0);

        assert!(!check.allowed);
        assert!(check.reason.is_some());
    }

    #[cfg(not(windows))]
    #[test]
    fn non_windows_process_inventory_is_unsupported() {
        let result = super::list_processes();

        assert!(!result.capability.supported);
        assert_eq!(result.capability.status, CapabilitySupportStatus::Unsupported);
        assert_eq!(
            result.error.as_ref().map(|error| error.kind),
            Some(ReadOnlyBackendErrorKind::UnsupportedPlatform)
        );
    }
}
