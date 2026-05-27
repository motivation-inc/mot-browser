mod error;

pub use error::Error;
pub const DIGITS: &str = "0123456789";
pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub const LETTERS_DIGITS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub const KEYWORDS: &[&str] = &[
    "let",
    "const",
    "if",
    "else",
    "while",
    "for",
    "func",
    "struct",
    "inherits",
    "component",
];
