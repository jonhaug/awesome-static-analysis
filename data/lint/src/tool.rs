use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Tool,
    Service,
    List,
}

#[derive(Debug, Serialize, Deserialize, Eq)]
pub struct Tool {
    pub name: String,
    pub url: String,
    pub description: String,
    pub categories: Vec<String>,
    pub proprietary: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "type")]
    pub ttype: Type,
}

impl PartialEq for Tool {
    fn eq(&self, other: &Tool) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl PartialOrd for Tool {
    fn partial_cmp(&self, other: &Tool) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tool {
    fn cmp(&self, other: &Tool) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}