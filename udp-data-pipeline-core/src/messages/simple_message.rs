#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimpleMessage {
    pub id: String,
    pub timestamp: i64,
}

impl Default for SimpleMessage {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Local::now().naive_utc().and_utc().timestamp(),
        }
    }
}
