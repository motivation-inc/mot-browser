use crate::lexer::Span;

/// A WebSpeak error object.
pub struct Error {
    pub span: Span,
    pub message: String,
    pub help_message: Option<String>,
    pub contents: String,
    pub propagates: bool,
}

impl Error {
    /// Constructs a new error object.
    ///
    /// - `message`: the message to construct
    pub fn new(message: &str, help_message: Option<&str>, span: Span) -> Self {
        Self {
            span,
            message: message.to_owned(),
            help_message: Some(help_message.unwrap_or("").to_owned()),
            contents: String::new(),
            propagates: false,
        }
    }
}
