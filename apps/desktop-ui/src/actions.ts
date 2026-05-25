import { invoke } from "@tauri-apps/api/core";

export type ActionKind =
  | "reveal_path"
  | "open_windows_settings"
  | "kill_process"
  | "block_network"
  | "unblock_network"
  | "disable_startup"
  | "restore_startup"
  | "quarantine_file"
  | "restore_quarantine"
  | "schedule_delete_on_reboot"
  | "detect_file_lockers";

export type ActionRiskLevel = "safe" | "caution" | "dangerous";
export type ActionAvailability =
  | "available"
  | "disabled"
  | "unsupported"
  | "planned"
  | "requires_admin"
  | "requires_confirmation";
export type ActionStatus = "succeeded" | "failed" | "denied" | "cancelled" | "unsupported" | "dry_run";

export type ActionRequest = {
  id: string;
  kind: ActionKind;
  target: string;
  target_display_name: string;
  risk_level: ActionRiskLevel;
  requested_at: string;
  source_screen: string;
  metadata_json?: Record<string, unknown> | null;
};

export type ActionPlan = {
  request: ActionRequest;
  availability: ActionAvailability;
  requires_confirmation: boolean;
  confirmation_title: string;
  confirmation_message: string;
  irreversible: boolean;
  can_undo: boolean;
  disabled_reason?: string | null;
  expected_effects: string[];
  warnings: string[];
};

export type ActionResult = {
  request_id: string;
  kind: ActionKind;
  target: string;
  started_at: string;
  finished_at: string;
  status: ActionStatus;
  message: string;
  error?: string | null;
  metadata_json?: Record<string, unknown> | null;
};

export type ActionHistoryQuery = {
  kind?: ActionKind | null;
  status?: ActionStatus | null;
  text?: string | null;
  limit?: number | null;
};

export function createActionRequest(
  kind: ActionKind,
  target: string,
  targetDisplayName: string,
  sourceScreen: string,
): ActionRequest {
  return {
    id: `action-${Date.now()}-${Math.round(Math.random() * 100000)}`,
    kind,
    target,
    target_display_name: targetDisplayName,
    risk_level: classifyActionRisk(kind),
    requested_at: `unix:${Math.floor(Date.now() / 1000)}`,
    source_screen: sourceScreen,
    metadata_json: null,
  };
}

export function classifyActionRisk(kind: ActionKind): ActionRiskLevel {
  if (kind === "reveal_path" || kind === "open_windows_settings") {
    return "safe";
  }
  if (kind === "detect_file_lockers") {
    return "caution";
  }
  return "dangerous";
}

export async function planAction(request: ActionRequest): Promise<ActionPlan> {
  try {
    return await invoke<ActionPlan>("jsentinel_plan_action", { request });
  } catch (error) {
    return {
      request,
      availability: "unsupported",
      requires_confirmation: false,
      confirmation_title: "Action unavailable",
      confirmation_message: String(error),
      irreversible: false,
      can_undo: false,
      disabled_reason: String(error),
      expected_effects: ["No action was executed."],
      warnings: [String(error)],
    };
  }
}

export async function executeSafeAction(request: ActionRequest): Promise<ActionResult> {
  try {
    return await invoke<ActionResult>("jsentinel_execute_safe_action", { request });
  } catch (error) {
    return {
      request_id: request.id,
      kind: request.kind,
      target: request.target,
      started_at: request.requested_at,
      finished_at: `unix:${Math.floor(Date.now() / 1000)}`,
      status: "failed",
      message: "Action execution failed before reaching policy-gated backend.",
      error: String(error),
      metadata_json: null,
    };
  }
}

export async function loadActionHistory(query: ActionHistoryQuery): Promise<ActionResult[]> {
  try {
    return await invoke<ActionResult[]>("jsentinel_list_action_history", { query });
  } catch {
    return [];
  }
}
