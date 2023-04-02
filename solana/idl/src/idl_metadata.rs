use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlMetadata {
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl IdlMetadata {
    pub fn new(address: &str) -> Self {
        Self {
            origin: "nautilus".to_string(),
            address: Some(address.to_string()),
        }
    }
    pub fn new_with_no_id() -> Self {
        Self {
            origin: "nautilus".to_string(),
            address: None,
        }
    }
}
