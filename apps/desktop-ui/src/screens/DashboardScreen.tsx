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
import {
  loadReadOnlyDiagnostics,
  loadSystemCapabilities,
  modeLabel,
  type CapabilityStatus,
  type ReadOnlyDiagnostics,
  type SystemDataMode,
} from "../system";
import type { ScreenId } from "../types";
import {
  EmptyState,
  EventKindBadge,
  MetricCard,
  RefreshBar,
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
  const [capabilities, setCapabilities] = useState<CapabilityStatus[]>([]);
  const [backendMode, setBackendMode] = useState<SystemDataMode>("unsupported_platform");
  const [diagnostics, setDiagnostics] = useState<ReadOnlyDiagnostics | null>(null);
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);
  const [manualRefresh, setManualRefresh] = useState(0);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);

    Promise.all([
      loadDashboardSummary(),
      loadEvents({ kind: null, severity: null, text: null, limit: 5 }),
      loadSystemCapabilities(),
      loadReadOnlyDiagnostics(),
    ]).then(([summaryResult, eventsResult, capabilityResult, diagnosticsResult]) => {
      if (!cancelled) {
        setSummary(summaryResult.data);
        setEvents(eventsResult.data);
        setDataSource(summaryResult.source);
        setCapabilities(capabilityResult.data);
        setBackendMode(diagnosticsResult.mode === "mock_fallback" ? capabilityResult.mode : diagnosticsResult.mode);
        setDiagnostics(diagnosticsResult.data);
        setLastUpdated(new Date().toLocaleTimeString());
        setLoading(false);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [refreshToken, manualRefresh]);

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

      <RefreshBar
        count={capabilities.length}
        countLabel={t.system.capabilities}
        lastUpdated={lastUpdated ? `${t.system.lastRefreshed}: ${lastUpdated}` : null}
        loading={loading}
        loadingLabel={t.common.loading}
        onRefresh={() => setManualRefresh((value) => value + 1)}
        refreshLabel={t.system.refresh}
        sourceLabel={modeLabel(backendMode, t.systemDataModes)}
        sourceTone={backendMode === "live_windows" ? "success" : backendMode === "partial_support" ? "warning" : "neutral"}
      />

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

          <SectionCard title={t.dashboard.windowsBackend} description={t.dashboard.windowsBackendDescription}>
            <div className="badge-list">
              <StatusBadge
                label={modeLabel(backendMode, t.systemDataModes)}
                tone={backendMode === "live_windows" ? "success" : "warning"}
              />
            </div>
            <div className="capability-list">
              {capabilities.slice(0, 4).map((capability) => (
                <div className="capability-row" key={capability.id}>
                  <span>{capability.label}</span>
                  <span>{capability.data_source}</span>
                  <StatusBadge
                    label={t.capabilityStatus[capability.status]}
                    tone={capability.status === "supported" ? "success" : capability.status === "partial" ? "warning" : "neutral"}
                  />
                </div>
              ))}
            </div>
            <p className="muted-line">{t.dashboard.notRealTime}</p>
          </SectionCard>

          <SectionCard title={t.dashboard.diagnostics} description={t.dashboard.diagnosticsDescription}>
            <dl className="details-list">
              <div>
                <dt>{t.dashboard.platform}</dt>
                <dd>{diagnostics?.platform ?? "unknown"}</dd>
              </div>
              <div>
                <dt>{t.dashboard.processCount}</dt>
                <dd>{diagnostics?.process_count ?? 0}</dd>
              </div>
              <div>
                <dt>{t.dashboard.networkCount}</dt>
                <dd>{diagnostics?.network_connection_count ?? 0}</dd>
              </div>
              <div>
                <dt>{t.dashboard.startupCount}</dt>
                <dd>{diagnostics?.startup_entry_count ?? 0}</dd>
              </div>
            </dl>
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
