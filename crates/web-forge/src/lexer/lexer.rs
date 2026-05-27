use crate::{
    DIGITS, Error, KEYWORDS, LETTERS, LETTERS_DIGITS,
    lexer::{
        Span,
        position::Position,
        token::{Token, TokenType},
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

/// A lexer object.
pub struct Lexer {
    characters: Rc<[char]>,
    cursor: Position,
    current_char: Option<char>,
    contents: String,
}

impl Lexer {
    /// Constructs a new lexer.
    ///
    /// - `text`: the text to tokenize
    ///
    /// # Example
    ///
    /// ```
    /// use web_forge::lexer::{Lexer, TokenType};
    ///
    /// let mut lexer = Lexer::new("example(1 + 1.0);");
    /// let tokens = lexer.make_tokens().ok().unwrap();
    ///
    /// assert_eq!(tokens.len(), 8); // including EOF token
    /// assert!(tokens[0].matches(TokenType::Identifier, "example"));
    /// assert!(tokens[1].matches(TokenType::LParen, ""));
    /// assert!(tokens[2].matches(TokenType::Int, "1"));
    /// assert!(tokens[3].matches(TokenType::Plus, ""));
    /// assert!(tokens[4].matches(TokenType::Float, "1.0"));
    /// assert!(tokens[5].matches(TokenType::RParen, ""));
    /// assert!(tokens[6].matches(TokenType::SemiColon, ""));
    /// assert!(tokens[7].matches(TokenType::End, ""));
    /// ```
    pub fn new(text: &str) -> Self {
        let contents = text.replace("\r\n", "\n"); // remove any '\r'
        let contents = contents.trim_end(); // we trim the end of the contents so that the lexer can't advance into an empty newline
        let chars: Rc<[char]> = contents.chars().collect::<Vec<_>>().into();

        let lexer = Self {
            characters: chars.clone(),
            cursor: Position::new(0, 1, 1),
            current_char: if chars.len() > 0 {
                Some(chars[0])
            } else {
                None
            },
            contents: contents.to_owned(),
        };

        lexer
    }

    fn advance(&mut self) {
        self.cursor.advance(self.current_char);

        if (self.cursor.index as usize) < self.characters.len() {
            self.current_char = Some(self.characters[self.cursor.index as usize]);
        } else {
            self.current_char = None;
        }
    }

    /// Parses the text of the lexer into a `Vec` of `Token` objects.
    ///
    /// # Errors
    ///
    /// This function will `Err` if the lexing process fails at any point.
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char {
            let pos_start = self.cursor.clone();

            let token = match current_char {
                ' ' | '\t' | '\n' => {
                    self.advance();

                    continue;
                }
                ';' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::SemiColon,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                c if DIGITS.contains(c) => Some(self.make_number()?),
                c if LETTERS.contains(c) => Some(self.make_identifier()),
                '"' => Some(self.make_string()?),
                '+' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::Plus,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '-' => Some(self.make_minus_or_arrow()),
                '*' => Some(self.make_mul_or_pow()),
                '/' => self.make_div_or_comment(),
                '%' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::Mod,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '(' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::LParen,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                ')' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::RParen,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '[' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::LSquare,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                ']' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::RSquare,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '{' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::LBracket,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '}' => {
                    self.advance();
                    Some(Token::new(
                        TokenType::RBracket,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '!' => Some(self.make_not_equals()?),
                '=' => Some(self.make_equals()),
                '<' => Some(self.make_less_than()),
                '>' => Some(self.make_greater_than()),
                ',' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::Comma,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }
                '.' => {
                    self.advance();

                    Some(Token::new(
                        TokenType::Period,
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ))
                }

                unknown_char => {
                    return Err(Error::new(
                        format!("unknown character '{unknown_char}'").as_str(),
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ));
                }
            };

            if let Some(t) = token {
                tokens.push(t);
            }
        }

        tokens.push(Token::new(
            TokenType::End,
            None,
            Span::new(self.cursor.clone(), self.cursor.clone()),
        ));

        Ok(tokens)
    }

    fn make_number(&mut self) -> Result<Token, Error> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let pos_start = self.cursor.clone();

        while let Some(character) = self.current_char {
            if character.is_ascii_digit() {
                num_str.push(character);
            } else if character == '.' {
                if dot_count == 1 {
                    return Err(Error::new(
                        "invalid numerical value",
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ));
                }

                dot_count += 1;
                num_str.push('.');
            } else if LETTERS.contains(character) {
                return Err(Error::new(
                    "identifiers cannot start with numerical values",
                    None,
                    Span::new(pos_start, self.cursor.clone()),
                ));
            } else {
                break;
            }

            self.advance();
        }

        let pos_end = self.cursor.clone();

        Ok(Token::new(
            if dot_count > 0 {
                TokenType::Float
            } else {
                TokenType::Int
            },
            Some(num_str),
            Span::new(pos_start, pos_end),
        ))
    }

    fn make_identifier(&mut self) -> Token {
        let mut id_string = String::new();
        let pos_start = self.cursor.clone();

        while let Some(character) = self.current_char {
            if LETTERS_DIGITS.contains(character) {
                id_string.push(character);

                self.advance();
            } else {
                break;
            }
        }

        let pos_end = self.cursor.clone();

        let token_type = if KEYWORDS.contains(&id_string.as_str()) {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        };

        Token::new(token_type, Some(id_string), Span::new(pos_start, pos_end))
    }

    fn make_string(&mut self) -> Result<Token, Error> {
        let mut string = String::new();
        let pos_start = self.cursor.clone();
        let mut escape_char = false;

        self.advance();

        let mut escape_chars = HashMap::new();
        escape_chars.insert('r', '\r');
        escape_chars.insert('e', '\x1b');
        escape_chars.insert('n', '\n');
        escape_chars.insert('t', '\t');
        escape_chars.insert('\\', '\\');
        escape_chars.insert('"', '\"');

        while let Some(character) = self.current_char {
            if character == '"' && !escape_char {
                break;
            }

            if escape_char {
                if character == 'e' {
                    string.push('\x1b');
                    self.advance();

                    if self.current_char == Some('[') {
                        string.push('[');
                        self.advance();

                        while let Some(c) = self.current_char {
                            string.push(c);
                            self.advance();
                            if c == 'm' {
                                break;
                            }
                        }
                    } else {
                        return Err(Error::new(
                            "invalid ANSI escape sequence (expected '[')",
                            None,
                            Span::new(pos_start, self.cursor.clone()),
                        ));
                    }
                } else if let Some(replacement) = escape_chars.get(&character) {
                    string.push(*replacement);
                    self.advance();
                } else {
                    return Err(Error::new(
                        "invalid escape character",
                        None,
                        Span::new(pos_start, self.cursor.clone()),
                    ));
                }

                escape_char = false;

                continue;
            }

            if character == '\\' {
                escape_char = true;
            } else {
                string.push(character);
            }

            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(Error::new(
                "unfinished string",
                Some("add a '\"' at the end of the string to close it"),
                Span::new(pos_start, self.cursor.clone()),
            ));
        }

        self.advance();

        Ok(Token::new(
            TokenType::Str,
            Some(string),
            Span::new(pos_start, self.cursor.clone()),
        ))
    }

    fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::Minus;
        let pos_start = self.cursor.clone();

        self.advance();

        if let Some(character) = self.current_char
            && character == '>'
        {
            self.advance();
            token_type = TokenType::Arrow;
        }

        Token::new(token_type, None, Span::new(pos_start, self.cursor.clone()))
    }

    fn make_mul_or_pow(&mut self) -> Token {
        let mut token_type = TokenType::Mul;
        let pos_start = self.cursor.clone();

        self.advance();

        if let Some(character) = self.current_char
            && character == '*'
        {
            self.advance();
            token_type = TokenType::Pow;
        }

        Token::new(token_type, None, Span::new(pos_start, self.cursor.clone()))
    }

    fn make_div_or_comment(&mut self) -> Option<Token> {
        let pos_start = self.cursor.clone();

        self.advance();

        match self.current_char {
            Some('/') => {
                while let Some(c) = self.current_char {
                    if c == '\n' {
                        break;
                    }

                    self.advance();
                }

                None
            }
            Some('*') => {
                self.advance();

                while let Some(c) = self.current_char {
                    if c == '*' {
                        self.advance();

                        if self.current_char == Some('/') {
                            self.advance();
                            break;
                        }

                        continue;
                    }

                    self.advance();
                }

                None
            }
            _ => Some(Token::new(
                TokenType::Div,
                None,
                Span::new(pos_start, self.cursor.clone()),
            )),
        }
    }

    fn make_equals(&mut self) -> Token {
        let mut token_type = TokenType::Eq;
        let pos_start = self.cursor.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::EqEq;
        }

        let pos_end = self.cursor.clone();

        Token::new(token_type, None, Span::new(pos_start, pos_end))
    }

    fn make_not_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.cursor.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();

            let pos_end = self.cursor.clone();

            return Ok(Token::new(
                TokenType::Ne,
                None,
                Span::new(pos_start, pos_end),
            ));
        }

        self.advance();

        let pos_end = self.cursor.clone();

        Err(Error::new(
            "expected '=' after '!'",
            Some("add a '=' after the '!' character"),
            Span::new(pos_start, pos_end),
        ))
    }

    fn make_less_than(&mut self) -> Token {
        let mut token_type = TokenType::LT;
        let pos_start = self.cursor.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::LTE;
        }

        let pos_end = self.cursor.clone();

        Token::new(token_type, None, Span::new(pos_start, pos_end))
    }

    fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::GT;
        let pos_start = self.cursor.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::GTE;
        }

        let pos_end = self.cursor.clone();

        Token::new(token_type, None, Span::new(pos_start, pos_end))
    }
}
