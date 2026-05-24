#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsCapability {
    ProcessInventory,
    NetworkInventory,
    StartupInventory,
    EventCollection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsBackendPlan {
    pub primary_platform: bool,
    pub requires_kernel_driver: bool,
    pub implemented: bool,
    pub planned_capabilities: &'static [WindowsCapability],
}

pub fn plan() -> WindowsBackendPlan {
    WindowsBackendPlan {
        primary_platform: true,
        requires_kernel_driver: false,
        implemented: false,
        planned_capabilities: &[
            WindowsCapability::ProcessInventory,
            WindowsCapability::NetworkInventory,
            WindowsCapability::StartupInventory,
            WindowsCapability::EventCollection,
        ],
    }
}
