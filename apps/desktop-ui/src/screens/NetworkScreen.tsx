import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildNetworkRows } from "../viewModels";
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
  const [protocolFilter, setProtocolFilter] = useState<"all" | "TCP" | "UDP">("all");
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);
  const [warning, setWarning] = useState<string | null>(null);
  const [manualRefresh, setManualRefresh] = useState(0);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    Promise.all([
      loadEvents({ kind: "network", severity: null, text: null, limit: 100 }),
      loadNetworkConnections(),
    ]).then(([eventResult, connectionResult]) => {
      if (!cancelled) {
        setEvents(eventResult.data);
        setConnections(connectionResult.data);
        setMode(connectionResult.mode);
        setWarning(connectionResult.warning ?? null);
        setLastUpdated(new Date().toLocaleTimeString());
        setLoading(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken, manualRefresh]);

  const rows = buildNetworkRows(events);
  const liveItems =
    connections?.items.filter((connection) => protocolFilter === "all" || connection.protocol === protocolFilter) ?? [];
  const useLive = (mode === "live_windows" || mode === "partial_support") && Boolean(liveItems.length);
  const displayCount = useLive ? liveItems.length : rows.length;

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
      <SectionCard title={t.network.recentConnections} description={t.network.limitations}>
        <div className="mini-toolbar">
          <FilterSelect
            label={t.network.protocol}
            value={protocolFilter}
            onChange={(value) => setProtocolFilter(value as "all" | "TCP" | "UDP")}
            options={[
              { value: "all", label: t.system.all },
              { value: "TCP", label: "TCP" },
              { value: "UDP", label: "UDP" },
            ]}
          />
        </div>
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
            liveItems.map((connection, index) => (
              <div className="data-row" key={`${connection.protocol}-${connection.local_addr}-${connection.local_port}-${index}`}>
                <span>{connection.process_name ?? `${t.network.pid} ${connection.pid ?? t.network.unknown}`}</span>
                <span>{formatEndpoint(connection.local_addr, connection.local_port)}</span>
                <span>{formatEndpoint(connection.remote_addr, connection.remote_port)}</span>
                <span>{connection.protocol}</span>
                <StatusBadge label={connection.state ?? t.network.unknown} />
                <StatusBadge label={t.system.readOnly} tone="success" />
              </div>
            ))}
          {(mode === "live_windows" || mode === "partial_support") && !useLive && (
            <EmptyState title={t.network.noNetworkEvents} description={t.system.emptySnapshot} />
          )}
          {mode !== "live_windows" && mode !== "partial_support" && rows.length === 0 && (
            <EmptyState title={t.network.noNetworkEvents} />
          )}
          {mode !== "live_windows" && mode !== "partial_support" && rows.map((row) => (
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
      {useLive && liveItems.some((connection) => connection.process_path) && (
        <SectionCard title={t.network.processLocationsTitle} description={t.network.processLocationsDescription}>
          <div className="action-grid">
            {liveItems
              .filter((connection) => connection.process_path)
              .slice(0, 4)
              .map((connection, index) => (
                <ActionButton
                  kind="reveal_path"
                  sourceScreen="network"
                  target={connection.process_path ?? ""}
                  targetDisplayName={connection.process_name ?? `${t.network.pid} ${connection.pid ?? index}`}
                  key={`${connection.process_path}-${connection.pid ?? index}`}
                  t={t}
                >
                  {connection.process_name ?? `${t.network.pid} ${connection.pid ?? t.network.unknown}`}
                </ActionButton>
              ))}
          </div>
        </SectionCard>
      )}
      <SectionCard title={t.network.rulesTitle} description={t.network.rulesDescription}>
        <div className="action-grid">
          <ActionButton kind="block_network" sourceScreen="network" target="network-policy-placeholder" targetDisplayName={t.network.rulesTitle} t={t}>
            {t.disabledActions.blockNetwork}
          </ActionButton>
          <ActionButton kind="unblock_network" sourceScreen="network" target="network-policy-placeholder" targetDisplayName={t.network.rulesTitle} t={t}>
            {t.disabledActions.unblockNetwork}
          </ActionButton>
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
