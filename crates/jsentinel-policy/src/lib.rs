#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionRisk {
    ReadOnly,
    Reversible,
    Destructive,
    Privileged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDecision {
    AllowReadOnly,
    RequireConfirmation,
    BlockInV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvaluation {
    pub risk: ActionRisk,
    pub decision: ActionDecision,
    pub reason: &'static str,
}

pub fn evaluate_placeholder(risk: ActionRisk) -> PolicyEvaluation {
    let decision = match risk {
        ActionRisk::ReadOnly => ActionDecision::AllowReadOnly,
        ActionRisk::Reversible => ActionDecision::RequireConfirmation,
        ActionRisk::Destructive | ActionRisk::Privileged => ActionDecision::BlockInV1,
    };

    PolicyEvaluation {
        risk,
        decision,
        reason: "Package 0 contains policy stubs only; privileged actions are not implemented.",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionRiskLevel {
    Safe,
    Caution,
    Dangerous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    RequiresConfirmation,
    Denied,
}
