import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildProcessSummaries, type ProcessSummary } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge, StatusBadge } from "../components/ui";
import {
  loadProcessDetails,
  loadProcesses,
  modeLabel,
  type ProcessInfo,
  type ReadOnlyQueryResult,
  type SystemDataMode,
} from "../system";

export function ProcessesScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [selected, setSelected] = useState<ProcessSummary | null>(null);
  const [liveProcesses, setLiveProcesses] = useState<ReadOnlyQueryResult<ProcessInfo> | null>(null);
  const [mode, setMode] = useState<SystemDataMode>("mock_fallback");
  const [selectedLive, setSelectedLive] = useState<ProcessInfo | null>(null);

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: null, severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    loadProcesses().then((result) => {
      if (!cancelled) {
        setLiveProcesses(result.data);
        setMode(result.mode);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const processes = buildProcessSummaries(events);
  const useLive = mode === "live_windows" && Boolean(liveProcesses?.items.length);

  function selectLiveProcess(process: ProcessInfo) {
    setSelected(null);
    setSelectedLive(process);
    loadProcessDetails(process.pid).then((result) => {
      const detailed = result.data.items[0];
      if (detailed) {
        setSelectedLive(detailed);
      }
    });
  }

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.processes}</p>
        <h1>{t.processes.title}</h1>
        <p>{t.processes.subtitle}</p>
      </div>

      <div className="notice-strip">
        <strong>{modeLabel(mode, t.systemDataModes)}</strong>
        <span>{liveProcesses?.capability.limitation ?? t.processes.liveDescription}</span>
      </div>

      <div className="split-layout">
        <SectionCard title={t.processes.tableTitle} description={t.processes.limitations}>
          <div className="data-table">
            <div className="data-row data-row-head">
              <span>{t.processes.processName}</span>
              <span>{t.processes.pid}</span>
              <span>{useLive ? t.processes.parentPid : t.processes.lastActivity}</span>
              <span>{useLive ? t.processes.path : t.processes.eventCount}</span>
              <span>{t.processes.severity}</span>
            </div>
            {useLive &&
              liveProcesses?.items.map((process) => (
                <button
                  className="data-row data-row-button"
                  key={process.pid}
                  type="button"
                  onClick={() => selectLiveProcess(process)}
                >
                  <span>{process.name}</span>
                  <span>{process.pid}</span>
                  <span>{process.parent_pid ?? t.processes.unknown}</span>
                  <span>{process.path ?? t.processes.unknownPath}</span>
                  <StatusBadge label={process.confidence} tone="success" />
                </button>
              ))}
            {!useLive && processes.length === 0 && <EmptyState title={t.processes.noProcesses} />}
            {!useLive && processes.map((process) => (
              <button
                className="data-row data-row-button"
                key={`${process.name}-${process.pid ?? "unknown"}`}
                type="button"
                onClick={() => {
                  setSelectedLive(null);
                  setSelected(process);
                }}
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
          {selectedLive ? (
            <div className="detail-panel">
              <h2>{selectedLive.name}</h2>
              <p className="muted-line">{selectedLive.path ?? t.processes.unknownPath}</p>
              <div className="badge-list">
                <StatusBadge label={`PID ${selectedLive.pid}`} />
                <StatusBadge label={`${t.processes.parentPid} ${selectedLive.parent_pid ?? t.processes.unknown}`} />
                <StatusBadge label={t.system.liveWindowsData} tone="success" />
              </div>
              <dl className="details-list">
                <div>
                  <dt>{t.processes.commandLine}</dt>
                  <dd>{selectedLive.command_line ?? t.processes.unknown}</dd>
                </div>
                <div>
                  <dt>{t.processes.startedAt}</dt>
                  <dd>{selectedLive.started_at ?? t.processes.unknown}</dd>
                </div>
                <div>
                  <dt>{t.processes.limitationsLabel}</dt>
                  <dd>{selectedLive.limitations.join(" ")}</dd>
                </div>
              </dl>
              <div className="action-grid">
                <DisabledActionButton>{t.disabledActions.killProcess}</DisabledActionButton>
                <DisabledActionButton>{t.disabledActions.openLocation}</DisabledActionButton>
                <DisabledActionButton>{t.disabledActions.quarantine}</DisabledActionButton>
              </div>
              <p>{t.processes.backendReadOnlyNotice}</p>
            </div>
          ) : selected ? (
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
