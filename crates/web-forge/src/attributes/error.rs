use crate::lexer::Span;

/// A WebSpeak error object.
pub struct Error {
    span: Span,
    message: String,
    contents: String,
    pub(crate) propagates: bool,
}

impl Error {
    /// Constructs a new error object.
    ///
    /// - `message`: the message of the error
    /// - `contents`: the code encompassed by the error
    /// - `span`: the span of the error
    pub fn new(message: &str, contents: &str, span: Span) -> Self {
        Self {
            span,
            message: message.to_owned(),
            contents: contents.to_string(),
            propagates: false,
        }
    }
}
