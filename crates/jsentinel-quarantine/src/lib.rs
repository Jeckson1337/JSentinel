#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarantineStatus {
    NotImplemented,
    Planned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantinePlan {
    pub status: QuarantineStatus,
    pub reversible_by_design: bool,
    pub force_delete_supported: bool,
}

pub fn v1_plan() -> QuarantinePlan {
    QuarantinePlan {
        status: QuarantineStatus::Planned,
        reversible_by_design: true,
        force_delete_supported: false,
    }
}
