#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    Camera,
    Microphone,
    Location,
    UsbStorage,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceAccessObservation {
    pub device_kind: DeviceKind,
    pub process_name: String,
    pub observed_locally: bool,
}

impl DeviceAccessObservation {
    pub fn placeholder(device_kind: DeviceKind) -> Self {
        Self {
            device_kind,
            process_name: "not-collected-yet".to_string(),
            observed_locally: false,
        }
    }
}
