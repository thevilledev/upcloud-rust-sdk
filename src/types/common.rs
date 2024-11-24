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

// Add this new struct
#[derive(Debug, Default)]
pub struct LabelFilter {
    labels: Vec<String>,
}

impl LabelFilter {
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }

    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.labels.push(format!("{}={}", key, value));
        self
    }

    pub fn add_label(&mut self, key: &str) -> &mut Self {
        self.labels.push(key.to_string());
        self
    }

    pub fn add_label_value(&mut self, key: &str, value: &str) -> &mut Self {
        self.labels.push(format!("{}={}", key, value));
        self
    }

    pub fn to_query_params(&self) -> String {
        if self.labels.is_empty() {
            String::new()
        } else {
            self.labels
                .iter()
                .map(|l| format!("label={}", urlencoding::encode(l)))
                .collect::<Vec<_>>()
                .join("&")
        }
    }
}