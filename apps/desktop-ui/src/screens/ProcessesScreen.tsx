import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildProcessSummaries, type ProcessSummary } from "../viewModels";
import {
  EmptyState,
  ErrorState,
  RefreshBar,
  SectionCard,
  SeverityBadge,
  StatusBadge,
} from "../components/ui";
import { ActionButton } from "../components/actions";
import { precheckKillProcess, type KillProcessSafetyCheck } from "../actions";
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
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [manualRefresh, setManualRefresh] = useState(0);
  const [killSafety, setKillSafety] = useState<KillProcessSafetyCheck | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    Promise.all([
      loadEvents({ kind: null, severity: null, text: null, limit: 100 }),
      loadProcesses(),
    ]).then(([eventResult, processResult]) => {
      if (!cancelled) {
        setEvents(eventResult.data);
        setLiveProcesses(processResult.data);
        setMode(processResult.mode);
        setWarning(processResult.warning ?? null);
        setLastUpdated(new Date().toLocaleTimeString());
        setLoading(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken, manualRefresh]);

  const processes = buildProcessSummaries(events);
  const useLive =
    (mode === "live_windows" || mode === "partial_support") && Boolean(liveProcesses?.items.length);
  const displayCount = useLive ? liveProcesses?.items.length ?? 0 : processes.length;

  function selectLiveProcess(process: ProcessInfo) {
    setSelected(null);
    setSelectedLive(process);
    setKillSafety(null);
    loadProcessDetails(process.pid).then((result) => {
      const detailed = result.data.items[0];
      if (detailed) {
        setSelectedLive(detailed);
        precheckKillProcess(detailed.pid).then(setKillSafety);
      } else {
        precheckKillProcess(process.pid).then(setKillSafety);
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

      <RefreshBar
        count={displayCount}
        countLabel={t.system.count}
        lastUpdated={lastUpdated ? `${t.system.lastRefreshed}: ${lastUpdated}` : null}
        loading={loading}
        loadingLabel={t.common.loading}
        onRefresh={() => setManualRefresh((value) => value + 1)}
        refreshLabel={t.system.refresh}
        sourceLabel={modeLabel(mode, t.systemDataModes)}
        sourceTone={mode === "live_windows" ? "success" : mode === "partial_support" ? "warning" : "neutral"}
      />
      {warning && <ErrorState title={t.system.backendWarning} description={warning} />}

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
            {(mode === "live_windows" || mode === "partial_support") && !useLive && (
              <EmptyState title={t.processes.noProcesses} description={t.system.emptySnapshot} />
            )}
            {mode !== "live_windows" && mode !== "partial_support" && processes.length === 0 && (
              <EmptyState title={t.processes.noProcesses} />
            )}
            {mode !== "live_windows" && mode !== "partial_support" && processes.map((process) => (
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
                <ActionButton
                  kind="kill_process"
                  sourceScreen="processes"
                  target={String(selectedLive.pid)}
                  targetDisplayName={`${selectedLive.name} (PID ${selectedLive.pid})`}
                  metadataJson={{
                    pid: selectedLive.pid,
                    process_name: selectedLive.name,
                    process_path: selectedLive.path,
                    command_line: selectedLive.command_line,
                  }}
                  disabled={!killSafety || !killSafety.allowed}
                  disabledReason={killSafety?.reason ?? t.processes.killPrecheckPending}
                  onCompleted={() => setManualRefresh((value) => value + 1)}
                  t={t}
                >
                  {t.disabledActions.killProcess}
                </ActionButton>
                <ActionButton
                  kind="reveal_path"
                  sourceScreen="processes"
                  target={selectedLive.path ?? ""}
                  targetDisplayName={selectedLive.name}
                  disabled={!selectedLive.path}
                  disabledReason={t.actions.localPathRequired}
                  t={t}
                >
                  {t.disabledActions.openLocation}
                </ActionButton>
                <ActionButton kind="quarantine_file" sourceScreen="processes" target={selectedLive.path ?? ""} targetDisplayName={selectedLive.name} t={t}>
                  {t.disabledActions.quarantine}
                </ActionButton>
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
                <ActionButton
                  kind="kill_process"
                  sourceScreen="processes"
                  target={String(selected.pid ?? "")}
                  targetDisplayName={selected.name}
                  disabled
                  disabledReason={t.processes.liveBackendRequired}
                  t={t}
                >
                  {t.disabledActions.killProcess}
                </ActionButton>
                <ActionButton
                  kind="reveal_path"
                  sourceScreen="processes"
                  target={selected.path ?? ""}
                  targetDisplayName={selected.name}
                  disabled={!selected.path}
                  disabledReason={t.actions.localPathRequired}
                  t={t}
                >
                  {t.disabledActions.openLocation}
                </ActionButton>
                <ActionButton kind="quarantine_file" sourceScreen="processes" target={selected.path ?? ""} targetDisplayName={selected.name} t={t}>
                  {t.disabledActions.quarantine}
                </ActionButton>
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
