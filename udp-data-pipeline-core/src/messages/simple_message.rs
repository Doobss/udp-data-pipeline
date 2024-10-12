use super::PublishedMessage;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimpleMessage {
    pub id: String,
    pub index: i64,
    pub timestamp: i64,
}

impl SimpleMessage {
    pub fn new(id: Option<String>, index: Option<i64>, timestamp: Option<i64>) -> Self {
        let generate_timestamp = || chrono::Local::now().naive_utc().and_utc().timestamp();
        Self {
            id: id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            index: index.unwrap_or(0),
            timestamp: timestamp.unwrap_or_else(generate_timestamp),
        }
    }
}

impl Default for SimpleMessage {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            index: 0,
            timestamp: chrono::Local::now().naive_utc().and_utc().timestamp(),
        }
    }
}

impl PartialOrd for SimpleMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.index.partial_cmp(&other.index) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.timestamp.partial_cmp(&other.timestamp)
    }

    fn lt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        std::matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        std::matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        std::matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
    }
}

impl PartialEq for SimpleMessage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.index == other.index && self.timestamp == other.timestamp
    }
}

impl PublishedMessage for SimpleMessage {
    fn index(&self) -> usize
    where
        Self: Sized,
    {
        self.index as usize
    }

    fn id(&self) -> &str
    where
        Self: Sized,
    {
        &self.id
    }

    fn with_index(index: i64) -> Self {
        SimpleMessage::new(None, Some(index), None)
    }
}
