#[derive(Debug, Clone)]
pub enum Message {
    Info(String),
    Success(String),
    Warning(String),
    Error(String),
}

impl Message {
    pub fn text(&self) -> &str {
        match self {
            Message::Info(text) | Message::Success(text) | Message::Warning(text) | Message::Error(text) => text,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Message::Error(_))
    }
}
