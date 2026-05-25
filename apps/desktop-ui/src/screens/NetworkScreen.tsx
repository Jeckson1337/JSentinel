import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildNetworkRows } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge, StatusBadge } from "../components/ui";
import {
  loadNetworkConnections,
  modeLabel,
  type NetworkConnectionInfo,
  type ReadOnlyQueryResult,
  type SystemDataMode,
} from "../system";

export function NetworkScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);
  const [connections, setConnections] = useState<ReadOnlyQueryResult<NetworkConnectionInfo> | null>(null);
  const [mode, setMode] = useState<SystemDataMode>("mock_fallback");

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: "network", severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    loadNetworkConnections().then((result) => {
      if (!cancelled) {
        setConnections(result.data);
        setMode(result.mode);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const rows = buildNetworkRows(events);
  const useLive = mode === "live_windows" && Boolean(connections?.items.length);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.network}</p>
        <h1>{t.network.title}</h1>
        <p>{t.network.subtitle}</p>
      </div>
      <div className="notice-strip">
        <strong>{modeLabel(mode, t.systemDataModes)}</strong>
        <span>{connections?.capability.limitation ?? t.network.liveDescription}</span>
      </div>
      <SectionCard title={t.network.recentConnections} description={t.network.limitations}>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.network.process}</span>
            <span>{useLive ? t.network.local : t.network.remote}</span>
            <span>{t.network.remote}</span>
            <span>{t.network.protocol}</span>
            <span>{useLive ? t.network.state : t.network.timestamp}</span>
            <span>{t.network.severity}</span>
          </div>
          {useLive &&
            connections?.items.map((connection, index) => (
              <div className="data-row" key={`${connection.protocol}-${connection.local_addr}-${connection.local_port}-${index}`}>
                <span>{connection.process_name ?? `${t.network.pid} ${connection.pid ?? t.network.unknown}`}</span>
                <span>{formatEndpoint(connection.local_addr, connection.local_port)}</span>
                <span>{formatEndpoint(connection.remote_addr, connection.remote_port)}</span>
                <span>{connection.protocol}</span>
                <StatusBadge label={connection.state ?? t.network.unknown} />
                <StatusBadge label={t.system.readOnly} tone="success" />
              </div>
            ))}
          {!useLive && rows.length === 0 && <EmptyState title={t.network.noNetworkEvents} />}
          {!useLive && rows.map((row) => (
            <div className="data-row" key={`${row.target}-${row.timestamp}`}>
              <span>{row.processName}</span>
              <span>{t.network.unknown}</span>
              <span>{row.target}</span>
              <span>{row.protocol}</span>
              <span>{row.timestamp}</span>
              <SeverityBadge severity={row.severity} t={t} />
            </div>
          ))}
        </div>
      </SectionCard>
      <SectionCard title={t.network.rulesTitle} description={t.network.rulesDescription}>
        <div className="action-grid">
          <DisabledActionButton>{t.disabledActions.blockNetwork}</DisabledActionButton>
          <DisabledActionButton>{t.disabledActions.unblockNetwork}</DisabledActionButton>
        </div>
      </SectionCard>
    </section>
  );
}

function formatEndpoint(address?: string | null, port?: number | null): string {
  if (!address) {
    return "unknown";
  }
  return port ? `${address}:${port}` : address;
}
