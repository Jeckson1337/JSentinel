import { invoke } from "@tauri-apps/api/core";

export type EventSeverity = "info" | "warning" | "critical";
export type EventKind =
  | "process"
  | "network"
  | "file"
  | "startup"
  | "device_access"
  | "locked_file"
  | "security"
  | "system";

export type EventSource = "mock" | "user" | "core" | "windows_backend" | "linux_backend";

export type ProcessRef = {
  pid?: number | null;
  name: string;
  path?: string | null;
};

export type AccessEvent = {
  id: string | { 0: string };
  timestamp: string | { 0: string };
  kind: EventKind;
  severity: EventSeverity;
  source: EventSource;
  process?: ProcessRef | null;
  title: string;
  summary: string;
  target?: string | null;
  metadata_json?: unknown;
  confidence?: string | null;
};

export type EventQuery = {
  kind?: EventKind | null;
  severity?: EventSeverity | null;
  text?: string | null;
  limit?: number | null;
};

export type DashboardSummary = {
  total_events: number;
  warnings: number;
  critical: number;
  process_events: number;
  network_events: number;
  file_events: number;
  startup_events: number;
  device_access_events: number;
  locked_file_events: number;
  security_events: number;
  latest_event_timestamp?: string | { 0: string } | null;
  demo_data_only: boolean;
};

export type EventLoadResult<T> = {
  data: T;
  source: "tauri_sqlite" | "frontend_mock";
  warning?: string;
};

export const fallbackEvents: AccessEvent[] = [
  {
    id: "frontend-mock-process",
    timestamp: "demo:recent",
    kind: "process",
    severity: "info",
    source: "mock",
    process: { pid: 4242, name: "demo-browser.exe", path: "C:\\Program Files\\Demo Browser" },
    title: "Demo process observed",
    summary: "Frontend fallback mock event. No real process inspection occurred.",
    target: "demo-browser.exe",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-network",
    timestamp: "demo:recent",
    kind: "network",
    severity: "warning",
    source: "mock",
    process: { pid: 4242, name: "demo-browser.exe" },
    title: "Demo network connection",
    summary: "Frontend fallback mock event. No real network scan or request occurred.",
    target: "example.invalid:443",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-file",
    timestamp: "demo:recent",
    kind: "file",
    severity: "info",
    source: "mock",
    process: { pid: 3110, name: "demo-editor.exe" },
    title: "Demo file activity",
    summary: "Frontend fallback mock event. No real file watcher is running.",
    target: "C:\\Users\\Example\\Documents\\demo.txt",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-startup",
    timestamp: "demo:recent",
    kind: "startup",
    severity: "warning",
    source: "mock",
    title: "Demo startup entry",
    summary: "Frontend fallback mock event. No registry or systemd access occurred.",
    target: "Demo Helper",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-device",
    timestamp: "demo:recent",
    kind: "device_access",
    severity: "info",
    source: "mock",
    process: { pid: 9001, name: "demo-meeting.exe" },
    title: "Demo device access",
    summary: "Frontend fallback mock event. No camera or microphone monitoring is implemented.",
    target: "camera",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-locked-file",
    timestamp: "demo:recent",
    kind: "locked_file",
    severity: "warning",
    source: "mock",
    process: { pid: 777, name: "demo-sync.exe" },
    title: "Demo locked file",
    summary: "Frontend fallback mock event. No file handles were inspected.",
    target: "C:\\Users\\Example\\Documents\\locked-demo.dat",
    confidence: "demo_only",
  },
  {
    id: "frontend-mock-security",
    timestamp: "demo:recent",
    kind: "security",
    severity: "critical",
    source: "mock",
    title: "Demo security attention item",
    summary: "Frontend fallback mock event for validating critical severity rendering.",
    target: "Local policy demo",
    confidence: "demo_only",
  },
];

export async function loadEvents(query: EventQuery): Promise<EventLoadResult<AccessEvent[]>> {
  try {
    const events = await invoke<AccessEvent[]>("jsentinel_get_events", { query });
    return { data: events, source: "tauri_sqlite" };
  } catch (error) {
    return {
      data: filterFallbackEvents(query),
      source: "frontend_mock",
      warning: String(error),
    };
  }
}

export async function loadDashboardSummary(): Promise<EventLoadResult<DashboardSummary>> {
  try {
    const summary = await invoke<DashboardSummary>("jsentinel_get_dashboard_summary");
    return { data: summary, source: "tauri_sqlite" };
  } catch (error) {
    return {
      data: summarizeEvents(fallbackEvents),
      source: "frontend_mock",
      warning: String(error),
    };
  }
}

export async function seedDemoEvents(): Promise<EventLoadResult<number>> {
  try {
    const inserted = await invoke<number>("jsentinel_seed_mock_events");
    return { data: inserted, source: "tauri_sqlite" };
  } catch (error) {
    return {
      data: fallbackEvents.length,
      source: "frontend_mock",
      warning: String(error),
    };
  }
}

export function eventId(event: AccessEvent): string {
  return typeof event.id === "string" ? event.id : event.id[0];
}

export function eventTimestamp(event: AccessEvent): string {
  return typeof event.timestamp === "string" ? event.timestamp : event.timestamp[0];
}

export function eventTimestampLabel(event: AccessEvent): string {
  const timestamp = eventTimestamp(event);

  if (timestamp.startsWith("unix:")) {
    const seconds = Number(timestamp.replace("unix:", ""));
    if (Number.isFinite(seconds)) {
      return new Date(seconds * 1000).toLocaleString();
    }
  }

  return timestamp;
}

export function summaryTimestamp(summary: DashboardSummary): string | null {
  const timestamp = summary.latest_event_timestamp;
  if (!timestamp) {
    return null;
  }
  return typeof timestamp === "string" ? timestamp : timestamp[0];
}

export function metadataValue(event: AccessEvent, key: string): string | null {
  if (!event.metadata_json || typeof event.metadata_json !== "object") {
    return null;
  }

  const value = (event.metadata_json as Record<string, unknown>)[key];
  return typeof value === "string" || typeof value === "number" ? String(value) : null;
}

function filterFallbackEvents(query: EventQuery): AccessEvent[] {
  const text = query.text?.trim().toLowerCase();

  return fallbackEvents
    .filter((event) => !query.kind || event.kind === query.kind)
    .filter((event) => !query.severity || event.severity === query.severity)
    .filter((event) => {
      if (!text) {
        return true;
      }

      return [event.title, event.summary, event.process?.name, event.target]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(text));
    })
    .slice(0, query.limit ?? 100);
}

function summarizeEvents(events: AccessEvent[]): DashboardSummary {
  return {
    total_events: events.length,
    warnings: events.filter((event) => event.severity === "warning").length,
    critical: events.filter((event) => event.severity === "critical").length,
    process_events: events.filter((event) => event.kind === "process").length,
    network_events: events.filter((event) => event.kind === "network").length,
    file_events: events.filter((event) => event.kind === "file").length,
    startup_events: events.filter((event) => event.kind === "startup").length,
    device_access_events: events.filter((event) => event.kind === "device_access").length,
    locked_file_events: events.filter((event) => event.kind === "locked_file").length,
    security_events: events.filter((event) => event.kind === "security").length,
    latest_event_timestamp: events[0]?.timestamp ?? null,
    demo_data_only: true,
  };
}
