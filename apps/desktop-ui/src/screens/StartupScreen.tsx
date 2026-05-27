import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildStartupRows } from "../viewModels";
import {
  EmptyState,
  ErrorState,
  FilterSelect,
  RefreshBar,
  SectionCard,
  SeverityBadge,
  StatusBadge,
} from "../components/ui";
import { ActionButton } from "../components/actions";
import {
  loadStartupBackups,
  loadStartupEntries,
  modeLabel,
  type ReadOnlyQueryResult,
  type StartupBackupRecord,
  type StartupEntryInfo,
  type SystemDataMode,
} from "../system";

export function StartupScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [entries, setEntries] = useState<ReadOnlyQueryResult<StartupEntryInfo> | null>(null);
  const [mode, setMode] = useState<SystemDataMode>("mock_fallback");
  const [sourceFilter, setSourceFilter] = useState("all");
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [manualRefresh, setManualRefresh] = useState(0);
  const [backups, setBackups] = useState<StartupBackupRecord[]>([]);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    Promise.all([
      loadEvents({ kind: "startup", severity: null, text: null, limit: 100 }),
      loadStartupEntries(),
      loadStartupBackups({ limit: 100 }),
    ]).then(([eventResult, entryResult, backupResult]) => {
      if (!cancelled) {
        setEvents(eventResult.data);
        setEntries(entryResult.data);
        setBackups(backupResult);
        setMode(entryResult.mode);
        setWarning(entryResult.warning ?? null);
        setLastUpdated(new Date().toLocaleTimeString());
        setLoading(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken, manualRefresh]);

  const rows = buildStartupRows(events);
  const sourceOptions = Array.from(new Set(entries?.items.map((entry) => entry.source) ?? []));
  const liveItems =
    entries?.items.filter((entry) => sourceFilter === "all" || entry.source === sourceFilter) ?? [];
  const useLive = (mode === "live_windows" || mode === "partial_support") && Boolean(liveItems.length);
  const displayCount = useLive ? liveItems.length : rows.length;
  const backupByEntry = new Map(backups.map((backup) => [backup.entry_id, backup]));

  function startupMetadata(entry: StartupEntryInfo, backup: StartupBackupRecord | undefined) {
    return {
      entry_id: entry.id,
      name: entry.name,
      source: entry.source,
      scope: entry.scope,
      command: entry.command,
      path: entry.path,
      enabled: entry.enabled,
      original_location: entry.source,
      backup_available: Boolean(backup),
      backup_id: backup?.backup_id,
    };
  }

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.startup}</p>
        <h1>{t.startup.title}</h1>
        <p>{t.startup.subtitle}</p>
      </div>
      <div className="notice-strip">
        <strong>{modeLabel(mode, t.systemDataModes)}</strong>
        <span>{entries?.capability.limitation ?? t.startup.liveDescription}</span>
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
      <SectionCard title={t.startup.entriesTitle} description={t.startup.limitations}>
        <div className="mini-toolbar">
          <FilterSelect
            label={t.startup.source}
            value={sourceFilter}
            onChange={setSourceFilter}
            options={[
              { value: "all", label: t.system.all },
              ...sourceOptions.map((source) => ({ value: source, label: source })),
            ]}
          />
        </div>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.startup.name}</span>
            <span>{t.startup.source}</span>
            <span>{t.startup.state}</span>
            <span>{useLive ? t.startup.scope : t.startup.timestamp}</span>
            <span>{useLive ? t.startup.backup : t.startup.severity}</span>
            <span>{t.startup.actions}</span>
          </div>
          {useLive &&
            liveItems.map((entry) => {
              const backup = backupByEntry.get(entry.id);
              return (
                <div className="data-row" key={entry.id}>
                  <span>{entry.name}</span>
                  <span>{entry.source}</span>
                  <StatusBadge
                    label={entry.enabled === false ? t.startup.disabled : t.startup.enabled}
                    tone={entry.enabled === false ? "neutral" : "success"}
                  />
                  <span>{entry.scope}</span>
                  <StatusBadge
                    label={backup ? t.startup.backupAvailable : t.startup.noBackup}
                    tone={backup ? "success" : "neutral"}
                  />
                  <div className="inline-actions">
                    <ActionButton
                      kind="disable_startup"
                      sourceScreen="startup"
                      target={entry.id}
                      targetDisplayName={entry.name}
                      metadataJson={startupMetadata(entry, backup)}
                      t={t}
                    >
                      {t.disabledActions.disableStartup}
                    </ActionButton>
                    <ActionButton
                      kind="restore_startup"
                      sourceScreen="startup"
                      target={entry.id}
                      targetDisplayName={entry.name}
                      metadataJson={startupMetadata(entry, backup)}
                      disabled={!backup}
                      disabledReason={t.startup.backupRequired}
                      t={t}
                    >
                      {t.disabledActions.restoreStartup}
                    </ActionButton>
                  </div>
                </div>
              );
            })}
          {(mode === "live_windows" || mode === "partial_support") && !useLive && (
            <EmptyState title={t.startup.noStartupEvents} description={t.system.emptySnapshot} />
          )}
          {mode !== "live_windows" && mode !== "partial_support" && rows.length === 0 && (
            <EmptyState title={t.startup.noStartupEvents} />
          )}
          {mode !== "live_windows" && mode !== "partial_support" && rows.map((row) => (
            <div className="data-row" key={`${row.name}-${row.timestamp}`}>
              <span>{row.name}</span>
              <span>{row.source}</span>
              <StatusBadge label={row.enabled} tone="neutral" />
              <span>{row.timestamp}</span>
              <SeverityBadge severity={row.severity} t={t} />
              <span>{t.startup.mockActionsUnavailable}</span>
            </div>
          ))}
        </div>
      </SectionCard>
      <SectionCard title={t.startup.readinessTitle} description={t.startup.readinessDescription}>
        <div className="metric-grid">
          <div className="metric-card">
            <span>{t.startup.readOnlyEntries}</span>
            <strong>{entries?.items.length ?? 0}</strong>
          </div>
          <div className="metric-card">
            <span>{t.startup.backupRecords}</span>
            <strong>{backups.length}</strong>
          </div>
          <div className="metric-card">
            <span>{t.startup.registryWrites}</span>
            <strong>{t.system.unsupported}</strong>
          </div>
        </div>
        <p className="muted-line">{t.startup.noRegistryWrites}</p>
      </SectionCard>
      <SectionCard title={t.startup.actionsTitle} description={t.startup.actionsDescription}>
        <div className="action-grid">
          <ActionButton kind="disable_startup" sourceScreen="startup" target="startup-placeholder" targetDisplayName={t.startup.entriesTitle} t={t}>
            {t.disabledActions.disableStartup}
          </ActionButton>
          <ActionButton kind="restore_startup" sourceScreen="startup" target="startup-placeholder" targetDisplayName={t.startup.entriesTitle} t={t}>
            {t.disabledActions.restoreStartup}
          </ActionButton>
          <ActionButton
            kind="reveal_path"
            sourceScreen="startup"
            target={liveItems.find((entry) => entry.path)?.path ?? ""}
            targetDisplayName={t.startup.entriesTitle}
            disabled={!liveItems.some((entry) => entry.path)}
            disabledReason={t.actions.localPathRequired}
            t={t}
          >
            {t.disabledActions.openSource}
          </ActionButton>
        </div>
      </SectionCard>
    </section>
  );
}
