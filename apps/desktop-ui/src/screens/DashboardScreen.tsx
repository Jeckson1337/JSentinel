import { useEffect, useState } from "react";
import {
  eventTimestampLabel,
  loadDashboardSummary,
  loadEvents,
  summaryTimestamp,
  type AccessEvent,
  type DashboardSummary,
} from "../events";
import type { Dictionary } from "../i18n";
import type { ScreenId } from "../types";
import {
  EmptyState,
  EventKindBadge,
  MetricCard,
  SectionCard,
  SeverityBadge,
  SourceBadge,
  StatusBadge,
} from "../components/ui";

type DashboardScreenProps = {
  t: Dictionary;
  refreshToken: number;
  onNavigate: (screen: ScreenId) => void;
};

export function DashboardScreen({ t, refreshToken, onNavigate }: DashboardScreenProps) {
  const [summary, setSummary] = useState<DashboardSummary | null>(null);
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [dataSource, setDataSource] = useState<"tauri_sqlite" | "frontend_mock">("frontend_mock");

  useEffect(() => {
    let cancelled = false;

    Promise.all([
      loadDashboardSummary(),
      loadEvents({ kind: null, severity: null, text: null, limit: 5 }),
    ]).then(([summaryResult, eventsResult]) => {
      if (!cancelled) {
        setSummary(summaryResult.data);
        setEvents(eventsResult.data);
        setDataSource(summaryResult.source);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const cards = summary
    ? [
        { label: t.dashboardSummary.totalEvents, value: summary.total_events, tone: "neutral" as const },
        { label: t.dashboardSummary.warnings, value: summary.warnings, tone: "warning" as const },
        { label: t.dashboardSummary.critical, value: summary.critical, tone: "critical" as const },
        { label: t.dashboardSummary.processActivity, value: summary.process_events, tone: "info" as const },
        { label: t.dashboardSummary.networkActivity, value: summary.network_events, tone: "info" as const },
        { label: t.dashboardSummary.fileActivity, value: summary.file_events, tone: "info" as const },
        { label: t.dashboardSummary.startupEvents, value: summary.startup_events, tone: "info" as const },
        {
          label: t.dashboardSummary.deviceAccessEvents,
          value: summary.device_access_events,
          tone: "info" as const,
        },
      ]
    : [];

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.dashboard.eyebrow}</p>
        <h1>{t.dashboard.title}</h1>
        <p>{t.dashboard.subtitle}</p>
      </div>

      <div className="notice-strip">
        <strong>{t.dashboardSummary.mockNoticeTitle}</strong>
        <span>{t.dashboardSummary.mockNoticeBody}</span>
      </div>

      <div className="metric-grid">
        {cards.map((card) => (
          <MetricCard key={card.label} {...card} />
        ))}
      </div>

      <div className="dashboard-layout">
        <SectionCard title={t.dashboard.recentEvents} description={t.dashboard.recentEventsDescription}>
          <div className="compact-list">
            {events.length === 0 && (
              <EmptyState title={t.timeline.noEventsYet} description={t.timeline.noEventsMatch} />
            )}
            {events.map((event) => (
              <article className="compact-event" key={`${event.title}-${eventTimestampLabel(event)}`}>
                <div className="event-row-meta">
                  <EventKindBadge kind={event.kind} t={t} />
                  <SeverityBadge severity={event.severity} t={t} />
                  <SourceBadge source={event.source} t={t} />
                </div>
                <h3>{event.title}</h3>
                <p>{event.summary}</p>
                <span className="muted-line">{eventTimestampLabel(event)}</span>
              </article>
            ))}
          </div>
        </SectionCard>

        <div className="side-stack">
          <SectionCard title={t.dashboard.localPromise} description={t.dashboard.localPromiseDescription}>
            <div className="badge-list">
              <StatusBadge label={t.common.noTelemetry} tone="success" />
              <StatusBadge label={t.common.notAntivirus} tone="warning" />
              <StatusBadge label={t.dashboard.localOnly} tone="success" />
            </div>
            <p className="muted-line">
              {dataSource === "tauri_sqlite"
                ? t.dashboardSummary.sqliteActive
                : t.dashboardSummary.frontendFallback}
            </p>
            <p className="muted-line">
              {t.dashboardSummary.latestEvent}:{" "}
              {summary ? summaryTimestamp(summary) ?? t.timeline.noEventsYet : t.common.loading}
            </p>
          </SectionCard>

          <SectionCard title={t.dashboard.quickActions} description={t.dashboard.quickActionsDescription}>
            <div className="action-grid">
              <button type="button" className="secondary-button" onClick={() => onNavigate("timeline")}>
                {t.dashboard.viewTimeline}
              </button>
              <button type="button" className="secondary-button" onClick={() => onNavigate("processes")}>
                {t.dashboard.inspectProcesses}
              </button>
              <button type="button" className="secondary-button" onClick={() => onNavigate("startup")}>
                {t.dashboard.reviewStartup}
              </button>
              <button type="button" className="secondary-button" onClick={() => onNavigate("network")}>
                {t.dashboard.networkActivity}
              </button>
            </div>
          </SectionCard>
        </div>
      </div>
    </section>
  );
}
