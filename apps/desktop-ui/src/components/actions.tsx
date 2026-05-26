import { useEffect, useState, type ReactNode } from "react";
import {
  createActionRequest,
  executeSafeAction,
  loadActionHistory,
  planAction,
  type ActionKind,
  type ActionPlan,
  type ActionResult,
  type ActionRiskLevel,
} from "../actions";
import type { Dictionary } from "../i18n";
import { EmptyState, StatusBadge } from "./ui";

export function ActionRiskBadge({ risk, t }: { risk: ActionRiskLevel; t: Dictionary }) {
  const tone = risk === "safe" ? "success" : risk === "caution" ? "warning" : "critical";
  return <StatusBadge label={t.actionRisk[risk]} tone={tone} />;
}

export function ConfirmationDialog({
  plan,
  t,
  onCancel,
  onConfirm,
}: {
  plan: ActionPlan;
  t: Dictionary;
  onCancel: () => void;
  onConfirm: () => void;
}) {
  return (
    <div className="dialog-backdrop" role="presentation">
      <section className="dialog-panel" role="dialog" aria-modal="true" aria-labelledby="action-dialog-title">
        <div className="section-card-heading">
          <h2 id="action-dialog-title">{plan.confirmation_title}</h2>
          <p>{plan.confirmation_message}</p>
        </div>
        <div className="badge-list">
          <ActionRiskBadge risk={plan.request.risk_level} t={t} />
          <StatusBadge label={t.actionAvailability[plan.availability]} />
          <StatusBadge label={t.actionKinds[plan.request.kind]} tone="info" />
        </div>
        <dl className="details-list">
          <div>
            <dt>{t.actions.displayName}</dt>
            <dd>{plan.request.target_display_name}</dd>
          </div>
          <div>
            <dt>{t.actions.target}</dt>
            <dd>{plan.request.target || t.actions.noTarget}</dd>
          </div>
        </dl>
        <ul className="plain-list">
          {plan.expected_effects.map((effect) => (
            <li key={effect}>{effect}</li>
          ))}
          {plan.warnings.map((warning) => (
            <li key={warning}>{warning}</li>
          ))}
        </ul>
        <div className="action-grid">
          <button className="secondary-button" type="button" onClick={onCancel}>
            {t.actions.cancel}
          </button>
          <button className="primary-button" type="button" onClick={onConfirm}>
            {t.actions.confirmAction}
          </button>
        </div>
      </section>
    </div>
  );
}

export function ActionButton({
  children,
  kind,
  sourceScreen,
  target,
  targetDisplayName,
  metadataJson = null,
  disabled = false,
  disabledReason,
  onCompleted,
  t,
}: {
  children: ReactNode;
  kind: ActionKind;
  sourceScreen: string;
  target: string;
  targetDisplayName: string;
  metadataJson?: Record<string, unknown> | null;
  disabled?: boolean;
  disabledReason?: string;
  onCompleted?: () => void;
  t: Dictionary;
}) {
  const [plan, setPlan] = useState<ActionPlan | null>(null);
  const [dialogPlan, setDialogPlan] = useState<ActionPlan | null>(null);
  const [result, setResult] = useState<ActionResult | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleClick() {
    if (disabled) {
      return;
    }
    setLoading(true);
    setResult(null);
    const request = createActionRequest(kind, target, targetDisplayName, sourceScreen, metadataJson);
    const nextPlan = await planAction(request);
    setPlan(nextPlan);
    setLoading(false);

    if (nextPlan.requires_confirmation && !nextPlan.disabled_reason) {
      setDialogPlan(nextPlan);
    }
  }

  async function handleConfirm() {
    if (!dialogPlan) {
      return;
    }
    setLoading(true);
    const nextResult = await executeSafeAction(dialogPlan.request);
    setResult(nextResult);
    setDialogPlan(null);
    setLoading(false);
    window.dispatchEvent(new Event("jsentinel-action-history-updated"));
    onCompleted?.();
  }

  return (
    <div className="action-button-stack">
      <button className="secondary-button" type="button" onClick={handleClick} disabled={loading || disabled}>
        {loading ? t.common.loading : children}
      </button>
      {disabled && disabledReason && (
        <p className="muted-line">
          {t.actions.disabledReason}: {disabledReason}
        </p>
      )}
      {plan?.disabled_reason && (
        <p className="muted-line">
          {t.actions.disabledReason}: {plan.disabled_reason}
        </p>
      )}
      {result && (
        <p className="muted-line">
          {t.actions.auditResult}: {t.actionStatus[result.status]} - {result.message}
        </p>
      )}
      {dialogPlan && (
        <ConfirmationDialog
          plan={dialogPlan}
          t={t}
          onCancel={() => setDialogPlan(null)}
          onConfirm={handleConfirm}
        />
      )}
    </div>
  );
}

export function ActionHistoryPanel({ t }: { t: Dictionary }) {
  const [history, setHistory] = useState<ActionResult[]>([]);

  useEffect(() => {
    let cancelled = false;
    const refreshHistory = () => {
      void loadActionHistory({ limit: 10 }).then((items) => {
        if (!cancelled) {
          setHistory(items);
        }
      });
    };
    refreshHistory();
    window.addEventListener("jsentinel-action-history-updated", refreshHistory);
    return () => {
      cancelled = true;
      window.removeEventListener("jsentinel-action-history-updated", refreshHistory);
    };
  }, []);

  if (history.length === 0) {
    return <EmptyState title={t.actions.noHistory} description={t.actions.noHistoryDescription} />;
  }

  return (
    <div className="data-table compact-action-table">
      <div className="data-row data-row-head">
        <span>{t.actions.kind}</span>
        <span>{t.actions.status}</span>
        <span>{t.actions.target}</span>
        <span>{t.actions.timestamp}</span>
        <span>{t.actions.result}</span>
      </div>
      {history.map((item) => (
        <div className="data-row" key={item.request_id}>
          <span>{t.actionKinds[item.kind]}</span>
          <StatusBadge label={t.actionStatus[item.status]} />
          <span>{item.target || t.actions.noTarget}</span>
          <span>{item.finished_at}</span>
          <span>{item.message}</span>
        </div>
      ))}
    </div>
  );
}
