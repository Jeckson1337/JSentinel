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
        </div>
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
            {t.actions.confirmDryRun}
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
  t,
}: {
  children: ReactNode;
  kind: ActionKind;
  sourceScreen: string;
  target: string;
  targetDisplayName: string;
  t: Dictionary;
}) {
  const [plan, setPlan] = useState<ActionPlan | null>(null);
  const [dialogPlan, setDialogPlan] = useState<ActionPlan | null>(null);
  const [result, setResult] = useState<ActionResult | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleClick() {
    setLoading(true);
    setResult(null);
    const request = createActionRequest(kind, target, targetDisplayName, sourceScreen);
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
  }

  return (
    <div className="action-button-stack">
      <button className="secondary-button" type="button" onClick={handleClick} disabled={loading}>
        {loading ? t.common.loading : children}
      </button>
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
    loadActionHistory({ limit: 10 }).then((items) => {
      if (!cancelled) {
        setHistory(items);
      }
    });
    return () => {
      cancelled = true;
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
