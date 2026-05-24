#![forbid(unsafe_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkObservation {
    pub process_name: String,
    pub remote_endpoint: String,
    pub protocol: String,
    pub attention_hint: Option<String>,
}

impl NetworkObservation {
    pub fn placeholder() -> Self {
        Self {
            process_name: "not-collected-yet".to_string(),
            remote_endpoint: "local-placeholder".to_string(),
            protocol: "unknown".to_string(),
            attention_hint: None,
        }
    }
}
