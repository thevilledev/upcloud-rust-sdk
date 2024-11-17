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

#[derive(Debug, Default,Serialize, Deserialize)]
pub struct Tags {
    pub tag: Vec<String>,
}