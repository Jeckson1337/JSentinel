import type { ReactNode } from "react";
import type { Dictionary } from "../i18n";
import type { EventKind, EventSeverity, EventSource } from "../events";

type Tone = "neutral" | "info" | "warning" | "critical" | "success";

export function StatusBadge({ label, tone = "neutral" }: { label: string; tone?: Tone }) {
  return <span className={`badge badge-${tone}`}>{label}</span>;
}

export function SeverityBadge({ severity, t }: { severity: EventSeverity; t: Dictionary }) {
  return <StatusBadge label={t.severity[severity]} tone={severity} />;
}

export function EventKindBadge({ kind, t }: { kind: EventKind; t: Dictionary }) {
  return <StatusBadge label={t.eventKinds[kind]} tone="info" />;
}

export function SourceBadge({ source, t }: { source: EventSource; t: Dictionary }) {
  return <StatusBadge label={t.eventSources[source]} tone={source === "mock" ? "warning" : "neutral"} />;
}

export function MetricCard({
  label,
  value,
  detail,
  tone = "neutral",
}: {
  label: string;
  value: string | number;
  detail?: string;
  tone?: Tone;
}) {
  return (
    <article className={`metric-card metric-${tone}`}>
      <span>{label}</span>
      <strong>{value}</strong>
      {detail && <p>{detail}</p>}
    </article>
  );
}

export function SectionCard({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: ReactNode;
}) {
  return (
    <section className="section-card">
      <div className="section-card-heading">
        <h2>{title}</h2>
        {description && <p>{description}</p>}
      </div>
      {children}
    </section>
  );
}

export function EmptyState({ title, description }: { title: string; description?: string }) {
  return (
    <div className="state-panel">
      <strong>{title}</strong>
      {description && <p>{description}</p>}
    </div>
  );
}

export function LoadingState({ label }: { label: string }) {
  return <div className="state-panel">{label}</div>;
}

export function ErrorState({ title, description }: { title: string; description?: string }) {
  return (
    <div className="state-panel state-error">
      <strong>{title}</strong>
      {description && <p>{description}</p>}
    </div>
  );
}

export function SearchInput({
  label,
  value,
  placeholder,
  onChange,
}: {
  label: string;
  value: string;
  placeholder: string;
  onChange: (value: string) => void;
}) {
  return (
    <label className="control-field">
      <span>{label}</span>
      <input value={value} onChange={(event) => onChange(event.target.value)} placeholder={placeholder} />
    </label>
  );
}

export function FilterSelect({
  label,
  value,
  options,
  onChange,
}: {
  label: string;
  value: string;
  options: Array<{ value: string; label: string }>;
  onChange: (value: string) => void;
}) {
  return (
    <label className="control-field">
      <span>{label}</span>
      <select value={value} onChange={(event) => onChange(event.target.value)}>
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  );
}

export function DisabledActionButton({ children }: { children: ReactNode }) {
  return (
    <button className="secondary-button" type="button" disabled>
      {children}
    </button>
  );
}

export function TopBar({ title, subtitle }: { title: string; subtitle: string }) {
  return (
    <header className="top-bar">
      <div>
        <h1>{title}</h1>
        <p>{subtitle}</p>
      </div>
      <StatusBadge label="Local-first" tone="success" />
    </header>
  );
}
