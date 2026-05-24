#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupSource {
    Unknown,
    WindowsRegistry,
    StartupFolder,
    SystemdUser,
    DesktopEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupEntry {
    pub name: String,
    pub source: StartupSource,
    pub enabled: bool,
}

impl StartupEntry {
    pub fn placeholder(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: StartupSource::Unknown,
            enabled: false,
        }
    }
}
