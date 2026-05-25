import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildFileRows } from "../viewModels";
import { DisabledActionButton, EmptyState, EventKindBadge, SectionCard, SeverityBadge } from "../components/ui";

export function FilesScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: null, severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const rows = buildFileRows(events);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.files}</p>
        <h1>{t.files.title}</h1>
        <p>{t.files.subtitle}</p>
      </div>
      <SectionCard title={t.files.activityTitle} description={t.files.limitations}>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.files.path}</span>
            <span>{t.files.process}</span>
            <span>{t.files.kind}</span>
            <span>{t.files.timestamp}</span>
            <span>{t.files.severity}</span>
          </div>
          {rows.length === 0 && <EmptyState title={t.files.noFileEvents} />}
          {rows.map((row) => (
            <div className="data-row" key={`${row.path}-${row.timestamp}`}>
              <span>{row.path}</span>
              <span>{row.processName}</span>
              <EventKindBadge kind={row.kind} t={t} />
              <span>{row.timestamp}</span>
              <SeverityBadge severity={row.severity} t={t} />
            </div>
          ))}
        </div>
      </SectionCard>
      <SectionCard title={t.files.actionsTitle} description={t.files.actionsDescription}>
        <div className="action-grid">
          <DisabledActionButton>{t.disabledActions.revealFile}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.detectLockers}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.quarantine}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.deleteOnReboot}</DisabledActionButton>
        </div>
      </SectionCard>
    </section>
  );
}
