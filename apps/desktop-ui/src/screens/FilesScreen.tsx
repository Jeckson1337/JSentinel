import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildFileRows } from "../viewModels";
import {
  EmptyState,
  ErrorState,
  EventKindBadge,
  RefreshBar,
  SectionCard,
  SeverityBadge,
  StatusBadge,
} from "../components/ui";
import { ActionButton } from "../components/actions";
import {
  detectFileLockers,
  modeLabel,
  type FileLockerInfo,
  type ReadOnlyQueryResult,
  type SystemDataMode,
} from "../system";

export function FilesScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [path, setPath] = useState("");
  const [lockerResult, setLockerResult] = useState<ReadOnlyQueryResult<FileLockerInfo> | null>(null);
  const [lockerMode, setLockerMode] = useState<SystemDataMode>("unsupported_platform");
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [manualRefresh, setManualRefresh] = useState(0);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    loadEvents({ kind: null, severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
        setLastUpdated(new Date().toLocaleTimeString());
        setLoading(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken, manualRefresh]);

  const rows = buildFileRows(events);

  async function handleDetectLockers() {
    const result = await detectFileLockers(path.trim());
    setLockerResult(result.data);
    setLockerMode(result.mode);
    setWarning(result.warning ?? result.data.error?.message ?? null);
  }

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.files}</p>
        <h1>{t.files.title}</h1>
        <p>{t.files.subtitle}</p>
      </div>
      <RefreshBar
        count={rows.length}
        countLabel={t.system.count}
        lastUpdated={lastUpdated ? `${t.system.lastRefreshed}: ${lastUpdated}` : null}
        loading={loading}
        loadingLabel={t.common.loading}
        onRefresh={() => setManualRefresh((value) => value + 1)}
        refreshLabel={t.system.refresh}
        sourceLabel={t.system.mockEventData}
        sourceTone="warning"
      />
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
        <label className="field-label">
          {t.files.lockedFilePath}
          <input
            className="text-input"
            value={path}
            onChange={(event) => setPath(event.target.value)}
            placeholder={t.files.lockedFilePlaceholder}
          />
        </label>
        <div className="action-grid">
          <ActionButton kind="reveal_path" sourceScreen="files" target={path.trim()} targetDisplayName={path.trim() || t.files.path} t={t}>
            {t.disabledActions.revealFile}
          </ActionButton>
          <button
            className="secondary-button"
            type="button"
            onClick={handleDetectLockers}
            disabled={!path.trim()}
          >
            {t.files.checkLockersReadOnly}
          </button>
          <ActionButton kind="quarantine_file" sourceScreen="files" target={path.trim()} targetDisplayName={path.trim() || t.files.path} t={t}>
            {t.disabledActions.quarantine}
          </ActionButton>
          <ActionButton kind="schedule_delete_on_reboot" sourceScreen="files" target={path.trim()} targetDisplayName={path.trim() || t.files.path} t={t}>
            {t.disabledActions.deleteOnReboot}
          </ActionButton>
        </div>
        {warning && <ErrorState title={t.system.backendWarning} description={warning} />}
        {lockerResult && (
          <div className="detail-panel">
            <div className="badge-list">
              <StatusBadge label={modeLabel(lockerMode, t.systemDataModes)} tone="warning" />
              <StatusBadge
                label={lockerResult.capability.supported ? t.system.supported : t.system.unsupported}
              />
            </div>
            <p>{lockerResult.capability.limitation}</p>
            {lockerResult.items.map((item) => (
              <p className="muted-line" key={`${item.path}-${item.confidence}`}>
                {item.path} - {item.limitation ?? item.confidence}
              </p>
            ))}
          </div>
        )}
      </SectionCard>
    </section>
  );
}
