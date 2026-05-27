use crate::lexer::span::Span;

/// Describes the type encompassed by the token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// An integer token (`0`)
    Int,
    /// A float token (`0.0`)
    Float,
    /// A string token (`""`)
    Str,
    /// A identifier token (`name`)
    Identifier,
    /// A keyword token (`if`)
    Keyword,
    /// A plus token (`+`)
    Plus,
    /// A minus token (`-`)
    Minus,
    /// A multiply token (`*`)
    Mul,
    /// A divide token (`/`)
    Div,
    /// A pow token (`**`)
    Pow,
    /// A modulo token (`%`)
    Mod,
    /// A left parenthesis token (`(`)
    LParen,
    /// A right parenthesis token (`)`)
    RParen,
    /// A left square token (`[`)
    LSquare,
    /// A right square token (`]`)
    RSquare,
    /// A left bracket token (`{`)
    LBracket,
    /// A right bracket token (`}`)
    RBracket,
    /// A equal token (`=`)
    Eq,
    /// A equal-equal token (`==`)
    EqEq,
    /// A not-equal token (`!=`)
    Ne,
    /// A less-than token (`<`)
    LT,
    /// A greater-than token (`>`)
    GT,
    /// A less-than/equal to token (`<=`)
    LTE,
    /// A greater-than/equal to token (`>=`)
    GTE,
    /// A comma token (`,`)
    Comma,
    /// A period token (`.`)
    Period,
    /// An arrow token (`->`)
    Arrow,
    /// A colon token (`:`)
    Colon,
    /// A semi-colon token (`;`)
    SemiColon,
    /// An and token (`&&`)
    And,
    /// An or token (`||`)
    Or,
    /// An end of file (EOF) token
    End,
}

/// A WebSpeak token object.
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: String,
    span: Span,
}

impl Token {
    /// Constructs a new token.
    ///
    /// - `token_type`: the token's type
    /// - `value`: the optional value of the token (if `None`, the value is left as an empty string)
    /// - `span`: the span of the token
    pub fn new(token_type: TokenType, value: Option<String>, span: Span) -> Self {
        Self {
            token_type,
            value: value.unwrap_or_default(),
            span,
        }
    }

    /// Checks if the type and value of a token match.
    ///
    /// - `token_type`: the token's type
    /// - `value`: the token's value
    ///
    /// This function will return `true` if `token_type` and `value` match with `self`,
    /// otherwise returning `false`.
    pub fn matches(&self, token_type: TokenType, value: &str) -> bool {
        self.token_type == token_type && self.value == value
    }
}
