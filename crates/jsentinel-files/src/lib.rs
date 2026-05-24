#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAttentionKind {
    RecentlyChanged,
    UnusualLocation,
    UnknownPublisher,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileObservation {
    pub path_display: String,
    pub attention_kind: Option<FileAttentionKind>,
    pub read_only_placeholder: bool,
}

impl FileObservation {
    pub fn placeholder(path_display: impl Into<String>) -> Self {
        Self {
            path_display: path_display.into(),
            attention_kind: None,
            read_only_placeholder: true,
        }
    }
}
