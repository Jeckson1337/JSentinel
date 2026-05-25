import { useEffect, useMemo, useState } from "react";
import {
  eventId,
  eventTimestampLabel,
  loadEvents,
  type AccessEvent,
  type EventKind,
  type EventSeverity,
  type EventSource,
} from "../events";
import type { Dictionary } from "../i18n";
import {
  EmptyState,
  EventKindBadge,
  FilterSelect,
  SearchInput,
  SectionCard,
  SeverityBadge,
  SourceBadge,
  StatusBadge,
} from "../components/ui";

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
const sources: Array<"all" | EventSource> = [
  "all",
  "mock",
  "user",
  "core",
  "windows_backend",
  "linux_backend",
];

export function TimelineScreen({ t, refreshToken }: TimelineScreenProps) {
  const [kind, setKind] = useState<"all" | EventKind>("all");
  const [severity, setSeverity] = useState<"all" | EventSeverity>("all");
  const [source, setSource] = useState<"all" | EventSource>("all");
  const [text, setText] = useState("");
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [selectedEvent, setSelectedEvent] = useState<AccessEvent | null>(null);
  const [dataSource, setDataSource] = useState<"tauri_sqlite" | "frontend_mock">("frontend_mock");
  const [warning, setWarning] = useState<string | null>(null);

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
        setWarning(result.warning ?? null);
        setSelectedEvent((current) =>
          current && result.data.some((event) => eventId(event) === eventId(current)) ? current : null,
        );
      }
    });

    return () => {
      cancelled = true;
    };
  }, [query, refreshToken]);

  const filteredEvents = source === "all" ? events : events.filter((event) => event.source === source);

  function clearFilters() {
    setKind("all");
    setSeverity("all");
    setSource("all");
    setText("");
  }

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
          {warning ? ` ${t.timeline.invokeFallback}` : ""}
        </span>
      </div>

      <div className="toolbar-grid">
        <SearchInput
          label={t.timeline.search}
          value={text}
          onChange={setText}
          placeholder={t.timeline.searchPlaceholder}
        />
        <FilterSelect
          label={t.timeline.kindFilter}
          value={kind}
          onChange={(value) => setKind(value as "all" | EventKind)}
          options={eventKinds.map((value) => ({
            value,
            label: value === "all" ? t.timeline.allKinds : t.eventKinds[value],
          }))}
        />
        <FilterSelect
          label={t.timeline.severityFilter}
          value={severity}
          onChange={(value) => setSeverity(value as "all" | EventSeverity)}
          options={severities.map((value) => ({
            value,
            label: value === "all" ? t.timeline.allSeverities : t.severity[value],
          }))}
        />
        <FilterSelect
          label={t.timeline.sourceFilter}
          value={source}
          onChange={(value) => setSource(value as "all" | EventSource)}
          options={sources.map((value) => ({
            value,
            label: value === "all" ? t.timeline.allSources : t.eventSources[value],
          }))}
        />
        <button className="secondary-button" type="button" onClick={clearFilters}>
          {t.timeline.clearFilters}
        </button>
      </div>

      <div className="split-layout">
        <SectionCard title={t.timeline.events} description={t.timeline.newestFirst}>
          <div className="timeline-list">
            {filteredEvents.length === 0 && (
              <EmptyState title={t.timeline.noEventsMatch} description={t.timeline.noEventsMatchDescription} />
            )}
            {filteredEvents.map((event) => (
              <button
                className={`event-row event-button severity-${event.severity}`}
                key={eventId(event)}
                type="button"
                onClick={() => setSelectedEvent(event)}
              >
                <div className="event-row-meta">
                  <EventKindBadge kind={event.kind} t={t} />
                  <SeverityBadge severity={event.severity} t={t} />
                  <SourceBadge source={event.source} t={t} />
                  <StatusBadge label={eventTimestampLabel(event)} />
                </div>
                <h2>{event.title}</h2>
                <p>{event.summary}</p>
                <div className="event-row-detail">
                  <span>{event.process?.name ?? t.timeline.noProcess}</span>
                  <span>{event.target ?? t.timeline.noTarget}</span>
                </div>
              </button>
            ))}
          </div>
        </SectionCard>

        <SectionCard title={t.timeline.details} description={t.timeline.detailsDescription}>
          {selectedEvent ? (
            <div className="detail-panel">
              <div className="event-row-meta">
                <EventKindBadge kind={selectedEvent.kind} t={t} />
                <SeverityBadge severity={selectedEvent.severity} t={t} />
                <SourceBadge source={selectedEvent.source} t={t} />
              </div>
              <h2>{selectedEvent.title}</h2>
              <p>{selectedEvent.summary}</p>
              <dl className="details-list">
                <div>
                  <dt>{t.timeline.timestamp}</dt>
                  <dd>{eventTimestampLabel(selectedEvent)}</dd>
                </div>
                <div>
                  <dt>{t.timeline.process}</dt>
                  <dd>{selectedEvent.process?.name ?? t.timeline.noProcess}</dd>
                </div>
                <div>
                  <dt>{t.timeline.target}</dt>
                  <dd>{selectedEvent.target ?? t.timeline.noTarget}</dd>
                </div>
                <div>
                  <dt>{t.timeline.confidence}</dt>
                  <dd>{selectedEvent.confidence ?? t.timeline.bestEffort}</dd>
                </div>
              </dl>
            </div>
          ) : (
            <EmptyState title={t.timeline.selectEvent} description={t.timeline.selectEventDescription} />
          )}
        </SectionCard>
      </div>
    </section>
  );
}
