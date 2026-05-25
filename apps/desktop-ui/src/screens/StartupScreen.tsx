import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildStartupRows } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge, StatusBadge } from "../components/ui";

export function StartupScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: "startup", severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const rows = buildStartupRows(events);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.startup}</p>
        <h1>{t.startup.title}</h1>
        <p>{t.startup.subtitle}</p>
      </div>
      <SectionCard title={t.startup.entriesTitle} description={t.startup.limitations}>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.startup.name}</span>
            <span>{t.startup.source}</span>
            <span>{t.startup.state}</span>
            <span>{t.startup.timestamp}</span>
            <span>{t.startup.severity}</span>
          </div>
          {rows.length === 0 && <EmptyState title={t.startup.noStartupEvents} />}
          {rows.map((row) => (
            <div className="data-row" key={`${row.name}-${row.timestamp}`}>
              <span>{row.name}</span>
              <span>{row.source}</span>
              <StatusBadge label={row.enabled} tone="neutral" />
              <span>{row.timestamp}</span>
              <SeverityBadge severity={row.severity} t={t} />
            </div>
          ))}
        </div>
      </SectionCard>
      <SectionCard title={t.startup.actionsTitle} description={t.startup.actionsDescription}>
        <div className="action-grid">
          <DisabledActionButton>{t.disabledActions.disableStartup}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.restoreStartup}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.openSource}</DisabledActionButton>
        </div>
      </SectionCard>
    </section>
  );
}
