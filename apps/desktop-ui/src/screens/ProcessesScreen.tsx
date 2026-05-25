import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildProcessSummaries, type ProcessSummary } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge, StatusBadge } from "../components/ui";

export function ProcessesScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [selected, setSelected] = useState<ProcessSummary | null>(null);

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

  const processes = buildProcessSummaries(events);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.processes}</p>
        <h1>{t.processes.title}</h1>
        <p>{t.processes.subtitle}</p>
      </div>

      <div className="split-layout">
        <SectionCard title={t.processes.tableTitle} description={t.processes.limitations}>
          <div className="data-table">
            <div className="data-row data-row-head">
              <span>{t.processes.processName}</span>
              <span>{t.processes.pid}</span>
              <span>{t.processes.lastActivity}</span>
              <span>{t.processes.eventCount}</span>
              <span>{t.processes.severity}</span>
            </div>
            {processes.length === 0 && <EmptyState title={t.processes.noProcesses} />}
            {processes.map((process) => (
              <button
                className="data-row data-row-button"
                key={`${process.name}-${process.pid ?? "unknown"}`}
                type="button"
                onClick={() => setSelected(process)}
              >
                <span>{process.name}</span>
                <span>{process.pid ?? "unknown"}</span>
                <span>{process.lastActivity}</span>
                <span>{process.eventCount}</span>
                <SeverityBadge severity={process.severity} t={t} />
              </button>
            ))}
          </div>
        </SectionCard>

        <SectionCard title={t.processes.detailsTitle} description={t.processes.detailsDescription}>
          {selected ? (
            <div className="detail-panel">
              <h2>{selected.name}</h2>
              <p className="muted-line">{selected.path ?? t.processes.unknownPath}</p>
              <div className="badge-list">
                <StatusBadge label={`PID ${selected.pid ?? "unknown"}`} />
                <SeverityBadge severity={selected.severity} t={t} />
              </div>
              <div className="action-grid">
                <DisabledActionButton>{t.disabledActions.killProcess}</DisabledActionButton>
                <DisabledActionButton>{t.disabledActions.openLocation}</DisabledActionButton>
                <DisabledActionButton>{t.disabledActions.quarantine}</DisabledActionButton>
              </div>
              <p>{t.processes.backendNotice}</p>
            </div>
          ) : (
            <EmptyState title={t.processes.selectProcess} />
          )}
        </SectionCard>
      </div>
    </section>
  );
}
