import {
  eventTimestampLabel,
  metadataValue,
  type AccessEvent,
  type EventSeverity,
} from "./events";

const severityRank: Record<EventSeverity, number> = {
  info: 1,
  warning: 2,
  critical: 3,
};

export type ProcessSummary = {
  name: string;
  pid: number | null;
  path: string | null;
  lastActivity: string;
  eventCount: number;
  severity: EventSeverity;
  events: AccessEvent[];
};

export type NetworkRow = {
  id: string;
  processName: string;
  target: string;
  protocol: string;
  severity: EventSeverity;
  timestamp: string;
  event: AccessEvent;
};

export type FileRow = {
  id: string;
  path: string;
  processName: string;
  severity: EventSeverity;
  timestamp: string;
  kind: "file" | "locked_file";
  event: AccessEvent;
};

export type StartupRow = {
  id: string;
  name: string;
  source: string;
  enabled: string;
  severity: EventSeverity;
  timestamp: string;
  event: AccessEvent;
};

export function highestSeverity(events: AccessEvent[]): EventSeverity {
  return events.reduce<EventSeverity>(
    (current, event) =>
      severityRank[event.severity] > severityRank[current] ? event.severity : current,
    "info",
  );
}

export function buildProcessSummaries(events: AccessEvent[]): ProcessSummary[] {
  const groups = new Map<string, AccessEvent[]>();

  for (const event of events) {
    if (!event.process?.name) {
      continue;
    }
    const key = `${event.process.name}:${event.process.pid ?? "unknown"}`;
    groups.set(key, [...(groups.get(key) ?? []), event]);
  }

  return Array.from(groups.values()).map((processEvents) => {
    const first = processEvents[0];
    return {
      name: first.process?.name ?? "unknown",
      pid: first.process?.pid ?? null,
      path: first.process?.path ?? null,
      lastActivity: eventTimestampLabel(first),
      eventCount: processEvents.length,
      severity: highestSeverity(processEvents),
      events: processEvents,
    };
  });
}

export function buildNetworkRows(events: AccessEvent[]): NetworkRow[] {
  return events
    .filter((event) => event.kind === "network")
    .map((event) => ({
      id: String(event.id),
      processName: event.process?.name ?? "unknown",
      target: event.target ?? "unknown",
      protocol: metadataValue(event, "protocol") ?? "unknown",
      severity: event.severity,
      timestamp: eventTimestampLabel(event),
      event,
    }));
}

export function buildFileRows(events: AccessEvent[]): FileRow[] {
  return events
    .filter((event) => event.kind === "file" || event.kind === "locked_file")
    .map((event) => ({
      id: String(event.id),
      path: event.target ?? "unknown",
      processName: event.process?.name ?? "unknown",
      severity: event.severity,
      timestamp: eventTimestampLabel(event),
      kind: event.kind === "locked_file" ? "locked_file" : "file",
      event,
    }));
}

export function buildStartupRows(events: AccessEvent[]): StartupRow[] {
  const sources = ["Registry Run", "Startup Folder", "Scheduled Task", "Service"];

  return events
    .filter((event) => event.kind === "startup")
    .map((event, index) => ({
      id: String(event.id),
      name: event.target ?? event.title,
      source: sources[index % sources.length],
      enabled: index % 2 === 0 ? "Enabled" : "Unknown",
      severity: event.severity,
      timestamp: eventTimestampLabel(event),
      event,
    }));
}
