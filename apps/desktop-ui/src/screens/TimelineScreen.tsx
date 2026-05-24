import { useEffect, useMemo, useState } from "react";
import type { Dictionary } from "../i18n";
import {
  eventId,
  eventTimestamp,
  loadEvents,
  type AccessEvent,
  type EventKind,
  type EventSeverity,
} from "../events";

type TimelineScreenProps = {
  t: Dictionary;
  refreshToken: number;
};

const eventKinds: Array<"all" | EventKind> = [
  "all",
  "process",
  "network",
  "file",
  "startup",
  "device_access",
  "locked_file",
  "security",
  "system",
];

const severities: Array<"all" | EventSeverity> = ["all", "info", "warning", "critical"];

export function TimelineScreen({ t, refreshToken }: TimelineScreenProps) {
  const [kind, setKind] = useState<"all" | EventKind>("all");
  const [severity, setSeverity] = useState<"all" | EventSeverity>("all");
  const [text, setText] = useState("");
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [dataSource, setDataSource] = useState<"tauri_sqlite" | "frontend_mock">("frontend_mock");

  const query = useMemo(
    () => ({
      kind: kind === "all" ? null : kind,
      severity: severity === "all" ? null : severity,
      text: text.trim() ? text.trim() : null,
      limit: 100,
    }),
    [kind, severity, text],
  );

  useEffect(() => {
    let cancelled = false;

    loadEvents(query).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
        setDataSource(result.source);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [query, refreshToken]);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.timeline}</p>
        <h1>{t.timeline.title}</h1>
        <p>{t.timeline.subtitle}</p>
      </div>

      <div className="notice-strip">
        <strong>{t.timeline.mockData}</strong>
        <span>
          {dataSource === "tauri_sqlite" ? t.timeline.sqliteSource : t.timeline.frontendFallback}
        </span>
      </div>

      <div className="timeline-filters">
        <label>
          <span>{t.timeline.kindFilter}</span>
          <select
            value={kind}
            onChange={(event) => setKind(event.target.value as "all" | EventKind)}
          >
            {eventKinds.map((value) => (
              <option key={value} value={value}>
                {value === "all" ? t.timeline.allKinds : t.eventKinds[value]}
              </option>
            ))}
          </select>
        </label>

        <label>
          <span>{t.timeline.severityFilter}</span>
          <select
            value={severity}
            onChange={(event) => setSeverity(event.target.value as "all" | EventSeverity)}
          >
            {severities.map((value) => (
              <option key={value} value={value}>
                {value === "all" ? t.timeline.allSeverities : t.severity[value]}
              </option>
            ))}
          </select>
        </label>

        <label>
          <span>{t.timeline.search}</span>
          <input
            value={text}
            onChange={(event) => setText(event.target.value)}
            placeholder={t.timeline.searchPlaceholder}
          />
        </label>
      </div>

      <div className="timeline-list">
        {events.length === 0 && <div className="empty-state compact">{t.timeline.noEventsYet}</div>}
        {events.map((event) => (
          <article className={`event-row severity-${event.severity}`} key={eventId(event)}>
            <div className="event-row-meta">
              <span>{eventTimestamp(event)}</span>
              <span>{t.eventKinds[event.kind]}</span>
              <span>{t.severity[event.severity]}</span>
              <span>{t.eventSources[event.source]}</span>
            </div>
            <h2>{event.title}</h2>
            <p>{event.summary}</p>
            <div className="event-row-detail">
              <span>{event.process?.name ?? t.timeline.noProcess}</span>
              <span>{event.target ?? t.timeline.noTarget}</span>
            </div>
          </article>
        ))}
      </div>
    </section>
  );
}
