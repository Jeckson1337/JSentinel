#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxSupportLevel {
    Planned,
    Beta,
    Stable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinuxBackendPlan {
    pub support_level: LinuxSupportLevel,
    pub implemented: bool,
    pub notes: &'static str,
}

pub fn plan() -> LinuxBackendPlan {
    LinuxBackendPlan {
        support_level: LinuxSupportLevel::Planned,
        implemented: false,
        notes: "Linux backend is part of the architecture but may arrive after Windows v1.",
    }
}
