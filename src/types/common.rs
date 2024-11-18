use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Labels {
    pub label: Vec<Label>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub key: String,
    pub value: String,
}

impl Labels {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.label.push(Label { key: key.to_string(), value: value.to_string() });
        self
    }
}

#[derive(Debug, Default,Serialize, Deserialize)]
pub struct Tags {
    pub tag: Vec<String>,
}