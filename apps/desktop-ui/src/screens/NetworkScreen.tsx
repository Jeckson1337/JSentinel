import { useEffect, useState } from "react";
import { loadEvents, type AccessEvent } from "../events";
import type { Dictionary } from "../i18n";
import { buildNetworkRows } from "../viewModels";
import { DisabledActionButton, EmptyState, SectionCard, SeverityBadge } from "../components/ui";

export function NetworkScreen({ t, refreshToken }: { t: Dictionary; refreshToken: number }) {
  const [events, setEvents] = useState<AccessEvent[]>([]);

  useEffect(() => {
    let cancelled = false;
    loadEvents({ kind: "network", severity: null, text: null, limit: 100 }).then((result) => {
      if (!cancelled) {
        setEvents(result.data);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [refreshToken]);

  const rows = buildNetworkRows(events);

  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.network}</p>
        <h1>{t.network.title}</h1>
        <p>{t.network.subtitle}</p>
      </div>
      <SectionCard title={t.network.recentConnections} description={t.network.limitations}>
        <div className="data-table">
          <div className="data-row data-row-head">
            <span>{t.network.process}</span>
            <span>{t.network.remote}</span>
            <span>{t.network.protocol}</span>
            <span>{t.network.timestamp}</span>
            <span>{t.network.severity}</span>
          </div>
          {rows.length === 0 && <EmptyState title={t.network.noNetworkEvents} />}
          {rows.map((row) => (
            <div className="data-row" key={`${row.target}-${row.timestamp}`}>
              <span>{row.processName}</span>
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
