use chrono::{DateTime, Utc};

#[derive(PartialEq)]
pub enum MessageType {
    Info,
    Error,
}

pub struct LogMessage {
    pub timestamp: DateTime<Utc>,
    pub message_type: MessageType,
    pub content: String,
}

impl LogMessage {
    pub fn new(message_type: MessageType, content: String) -> Self {
        let timestamp = Utc::now();  // Using chrono to get the current time

        Self {
            timestamp,
            message_type,
            content,
        }
    }

    pub fn to_string(&self) -> String {
        let type_str = match self.message_type {
            MessageType::Info => "[INFO]",
            MessageType::Error => "[ERROR]",
        };

        // Format the timestamp into a readable string
        let formatted_timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        format!("{} - {} {}", formatted_timestamp, type_str, self.content)
    }
}