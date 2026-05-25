import { invoke } from "@tauri-apps/api/core";

export type SystemPlatform = "windows" | "linux" | "unsupported";
export type CapabilitySupportStatus = "supported" | "partial" | "unsupported";
export type ReadOnlyBackendErrorKind =
  | "unsupported_platform"
  | "permission_denied"
  | "unavailable"
  | "parse_error"
  | "os_error"
  | "unknown";

export type ReadOnlyBackendError = {
  kind: ReadOnlyBackendErrorKind;
  message: string;
};

export type CapabilityStatus = {
  id: string;
  label: string;
  supported: boolean;
  status: CapabilitySupportStatus;
  requires_admin: boolean;
  data_source: string;
  read_only: boolean;
  limitation?: string | null;
};

export type ReadOnlyQueryResult<T> = {
  platform: SystemPlatform;
  provider: string;
  capability: CapabilityStatus;
  items: T[];
  error?: ReadOnlyBackendError | null;
};

export type ProcessInfo = {
  pid: number;
  name: string;
  path?: string | null;
  parent_pid?: number | null;
  command_line?: string | null;
  started_at?: string | null;
  owner?: string | null;
  source: string;
  confidence: string;
  limitations: string[];
};

export type NetworkConnectionInfo = {
  protocol: string;
  local_addr: string;
  local_port?: number | null;
  remote_addr?: string | null;
  remote_port?: number | null;
  state?: string | null;
  pid?: number | null;
  process_name?: string | null;
  process_path?: string | null;
};

export type StartupEntryInfo = {
  id: string;
  name: string;
  source: string;
  command?: string | null;
  path?: string | null;
  enabled?: boolean | null;
  scope: string;
  publisher?: string | null;
  risk?: string | null;
  limitation?: string | null;
};

export type FileLockerInfo = {
  pid?: number | null;
  process_name?: string | null;
  process_path?: string | null;
  path: string;
  confidence: string;
  limitation?: string | null;
};

export type SystemDataMode = "live_windows" | "partial_support" | "mock_fallback" | "unsupported_platform";

export type SystemLoadResult<T> = {
  data: T;
  mode: SystemDataMode;
  warning?: string;
};

export type ReadOnlyDiagnostics = {
  app_version: string;
  platform: string;
  capabilities: CapabilityStatus[];
  process_count: number;
  network_connection_count: number;
  startup_entry_count: number;
};

export async function loadSystemCapabilities(): Promise<SystemLoadResult<CapabilityStatus[]>> {
  try {
    const capabilities = await invoke<CapabilityStatus[]>("jsentinel_get_system_capabilities");
    const hasSupportedCapability = capabilities.some((capability) => capability.supported);
    const hasPartialCapability = capabilities.some((capability) => capability.status === "partial");
    return {
      data: capabilities,
      mode: hasSupportedCapability ? (hasPartialCapability ? "partial_support" : "live_windows") : "unsupported_platform",
    };
  } catch (error) {
    return {
      data: unsupportedCapabilities(String(error)),
      mode: "unsupported_platform",
      warning: String(error),
    };
  }
}

export async function loadProcesses(): Promise<SystemLoadResult<ReadOnlyQueryResult<ProcessInfo>>> {
  return invokeReadOnly<ProcessInfo>("jsentinel_list_processes", {
    id: "process_inventory",
    label: "Process inventory",
  });
}

export async function loadProcessDetails(
  pid: number,
): Promise<SystemLoadResult<ReadOnlyQueryResult<ProcessInfo>>> {
  return invokeReadOnly<ProcessInfo>(
    "jsentinel_get_process_details",
    { id: "process_details", label: "Process details" },
    { pid },
  );
}

export async function loadNetworkConnections(): Promise<
  SystemLoadResult<ReadOnlyQueryResult<NetworkConnectionInfo>>
> {
  return invokeReadOnly<NetworkConnectionInfo>("jsentinel_list_network_connections", {
    id: "network_connections",
    label: "Network connections",
  });
}

export async function loadStartupEntries(): Promise<
  SystemLoadResult<ReadOnlyQueryResult<StartupEntryInfo>>
> {
  return invokeReadOnly<StartupEntryInfo>("jsentinel_list_startup_entries", {
    id: "startup_entries",
    label: "Startup entries",
  });
}

export async function detectFileLockers(
  path: string,
): Promise<SystemLoadResult<ReadOnlyQueryResult<FileLockerInfo>>> {
  return invokeReadOnly<FileLockerInfo>(
    "jsentinel_detect_file_lockers",
    { id: "file_lockers", label: "File locker detection" },
    { path },
  );
}

export async function loadReadOnlyDiagnostics(): Promise<SystemLoadResult<ReadOnlyDiagnostics | null>> {
  try {
    const diagnostics = await invoke<ReadOnlyDiagnostics>("jsentinel_get_read_only_diagnostics");
    const hasPartialCapability = diagnostics.capabilities.some((capability) => capability.status === "partial");
    const hasSupportedCapability = diagnostics.capabilities.some((capability) => capability.supported);
    return {
      data: diagnostics,
      mode: hasSupportedCapability ? (hasPartialCapability ? "partial_support" : "live_windows") : "unsupported_platform",
    };
  } catch (error) {
    return {
      data: null,
      mode: "mock_fallback",
      warning: String(error),
    };
  }
}

export function modeLabel(mode: SystemDataMode, labels: Record<SystemDataMode, string>): string {
  return labels[mode];
}

async function invokeReadOnly<T>(
  command: string,
  capability: Pick<CapabilityStatus, "id" | "label">,
  args?: Record<string, unknown>,
): Promise<SystemLoadResult<ReadOnlyQueryResult<T>>> {
  try {
    const result = await invoke<ReadOnlyQueryResult<T>>(command, args);
    return {
      data: result,
      mode: result.error
        ? "unsupported_platform"
        : result.capability.supported
          ? result.capability.status === "partial"
            ? "partial_support"
            : "live_windows"
          : "unsupported_platform",
      warning: result.error?.message,
    };
  } catch (error) {
    return {
      data: unsupportedResult<T>(capability.id, capability.label, String(error)),
      mode: "mock_fallback",
      warning: String(error),
    };
  }
}

function unsupportedResult<T>(
  id: string,
  label: string,
  limitation: string,
): ReadOnlyQueryResult<T> {
  return {
    platform: "unsupported",
    provider: "frontend_fallback",
    capability: {
      id,
      label,
      supported: false,
      status: "unsupported",
      requires_admin: false,
      data_source: "frontend_fallback",
      read_only: true,
      limitation,
    },
    items: [],
    error: {
      kind: "unavailable",
      message: limitation,
    },
  };
}

function unsupportedCapabilities(limitation: string): CapabilityStatus[] {
  return [
    "process_inventory",
    "process_details",
    "network_connections",
    "startup_entries",
    "file_lockers",
  ].map((id) => ({
    id,
    label: id.replace(/_/g, " "),
    supported: false,
    status: "unsupported",
    requires_admin: false,
    data_source: "frontend_fallback",
    read_only: true,
    limitation,
  }));
}
