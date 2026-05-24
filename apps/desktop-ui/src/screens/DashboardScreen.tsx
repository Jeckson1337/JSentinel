import { useEffect, useState } from "react";
import type { Dictionary } from "../i18n";
import { loadDashboardSummary, summaryTimestamp, type DashboardSummary } from "../events";

type DashboardScreenProps = {
  t: Dictionary;
  refreshToken: number;
};

export function DashboardScreen({ t, refreshToken }: DashboardScreenProps) {
  const [summary, setSummary] = useState<DashboardSummary | null>(null);
  const [dataSource, setDataSource] = useState<"tauri_sqlite" | "frontend_mock">("frontend_mock");

  useEffect(() => {
    let cancelled = false;

    loadDashboardSummary().then((result) => {
      if (!cancelled) {
        setSummary(result.data);
        setDataSource(result.source);
      }
    });

    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const cards = summary
    ? [
        { label: t.dashboardSummary.totalEvents, value: summary.total_events },
        { label: t.dashboardSummary.warnings, value: summary.warnings },
        { label: t.dashboardSummary.critical, value: summary.critical },
        { label: t.dashboardSummary.processActivity, value: summary.process_events },
        { label: t.dashboardSummary.networkActivity, value: summary.network_events },
        { label: t.dashboardSummary.fileActivity, value: summary.file_events },
        { label: t.dashboardSummary.startupEvents, value: summary.startup_events },
        { label: t.dashboardSummary.deviceAccessEvents, value: summary.device_access_events },
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

      <div className="summary-grid">
        {cards.map((card) => (
          <article className="summary-card" key={card.label}>
            <span>{card.label}</span>
            <strong>{card.value}</strong>
          </article>
        ))}
      </div>

      <div className="status-grid">
        <article className="status-tile">
          <span>{t.dashboardSummary.storage}</span>
          <h2>{t.dashboardSummary.localSQLite}</h2>
          <p>
            {dataSource === "tauri_sqlite"
              ? t.dashboardSummary.sqliteActive
              : t.dashboardSummary.frontendFallback}
          </p>
        </article>
        <article className="status-tile">
          <span>{t.dashboardSummary.latestEvent}</span>
          <h2>{summary ? summaryTimestamp(summary) ?? t.timeline.noEventsYet : t.common.loading}</h2>
          <p>{t.dashboardSummary.latestEventDescription}</p>
        </article>
        {t.dashboard.principles.slice(1).map((item) => (
          <article className="status-tile" key={item.title}>
            <span>{item.state}</span>
            <h2>{item.title}</h2>
            <p>{item.description}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
