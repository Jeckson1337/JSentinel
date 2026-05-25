import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildStartupRows } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge, StatusBadge } from "../components/ui";
import {
  loadStartupEntries,
  modeLabel,
  type ReadOnlyQueryResult,
  type StartupEntryInfo,
  type SystemDataMode,
} from "../system";

export function StartupScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [entries, setEntries] = useState<ReadOnlyQueryResult<StartupEntryInfo> | null>(null);
  const [mode, setMode] = useState<SystemDataMode>("mock_fallback");

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: "startup", severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    loadStartupEntries().then((result) => {
      if (!cancelled) {
        setEntries(result.data);
        setMode(result.mode);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const rows = buildStartupRows(events);
  const useLive = mode === "live_windows" && Boolean(entries?.items.length);

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
      <SectionCard title={t.startup.entriesTitle} description={t.startup.limitations}>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.startup.name}</span>
            <span>{t.startup.source}</span>
            <span>{t.startup.state}</span>
            <span>{useLive ? t.startup.scope : t.startup.timestamp}</span>
            <span>{useLive ? t.startup.risk : t.startup.severity}</span>
          </div>
          {useLive &&
            entries?.items.map((entry) => (
              <div className="data-row" key={entry.id}>
                <span>{entry.name}</span>
                <span>{entry.source}</span>
                <StatusBadge
                  label={entry.enabled === false ? t.startup.disabled : t.startup.enabled}
                  tone={entry.enabled === false ? "neutral" : "success"}
                />
                <span>{entry.scope}</span>
                <StatusBadge label={entry.risk ?? t.startup.unknownRisk} />
              </div>
            ))}
          {!useLive && rows.length === 0 && <EmptyState title={t.startup.noStartupEvents} />}
          {!useLive && rows.map((row) => (
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
