use std::fmt::Display;
use crate::gui::log_message::MessageType;
use crate::gui::log_message::LogMessage;

#[derive(Default)]
pub struct Terminal {
    pub content: Vec<LogMessage>,
    pub max_lines: usize,
}

impl Terminal {
    pub fn new(max_lines: usize) -> Self {
        Self {
            content: Vec::new(),
            max_lines,
        }
    }
    // Updated methods to handle LogMessage...
    pub fn log<T: Display>(&mut self, message: T) {
        self.add_message(MessageType::Info, format!("{}", message));
    }

    pub fn log_error<T: Display>(&mut self, message: T) {
        self.add_message(MessageType::Error, format!("{}", message));
    }

    pub fn add_message(&mut self, message_type: MessageType, content: String) {
        if self.max_lines > 0 && self.content.len() >= self.max_lines {
            self.content.remove(0);
        }
        self.content.push(LogMessage::new(message_type, content));
    }
    

    pub fn display(&self, ui: &imgui::Ui, only_errors: bool) {
        for message in &self.content {
            if only_errors && message.message_type != MessageType::Error {
                continue; // Skip non-error messages if only_errors is true
            }

            let text = message.to_string();
            let color = match message.message_type {
                MessageType::Info => [0.0, 1.0, 0.0, 1.0], // Green for info
                MessageType::Error => [1.0, 0.0, 0.0, 1.0], // Red for error
            };
            ui.text_colored(color, &text);
        }
    }
}