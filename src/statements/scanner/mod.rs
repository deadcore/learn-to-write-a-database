use std::convert::TryFrom;
use std::iter::Peekable;

use log::trace;

#[derive(Debug, Clone)]
pub enum ScannerError {
    Error(String),
}

impl std::convert::From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        ScannerError::Error(format!("{}", err))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    U32(u32),
    SemiColon,
    Keyword(KeywordToken),
    Space,
    Identifier(String),
    Assignment,
    NewLine,
    LeftBracket,
    RightBracket,
    Comma,
    Apostrophe,
    Asterisk,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordToken {
    CREATE,
    TABLE,
    SELECT,
    FROM,
    AS,
    INSERT,
    INTO,
    VALUES,
    INT,
    TEXT,
}

impl std::convert::TryFrom<&str> for KeywordToken {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "SELECT" => Ok(KeywordToken::SELECT),
            "FROM" => Ok(KeywordToken::FROM),
            "AS" => Ok(KeywordToken::AS),
            "TABLE" => Ok(KeywordToken::TABLE),
            "CREATE" => Ok(KeywordToken::CREATE),
            "INSERT" => Ok(KeywordToken::INSERT),
            "INTO" => Ok(KeywordToken::INTO),
            "VALUES" => Ok(KeywordToken::VALUES),
            "INT" => Ok(KeywordToken::INT),
            "TEXT" => Ok(KeywordToken::TEXT),
            v => Err(format!("Unable to handle KeywordToken: [{}]", v))
        }
    }
}

pub struct TokenIterator<T: Iterator<Item=char>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=char>> TokenIterator<T> {
    pub fn new_iterator(chars: T) -> TokenIterator<T> {
        TokenIterator::new(chars.peekable())
    }

    fn new(inner: Peekable<T>) -> Self {
        TokenIterator { inner }
    }

    fn read_symbol(&mut self) -> Option<Token> {
        if let Some(t) = self.inner.next() {
            return Some(match t {
                ';' => Token::SemiColon,
                '\n' => Token::NewLine,
                ' ' => Token::Space,
                '(' => Token::LeftBracket,
                ')' => Token::RightBracket,
                ',' => Token::Comma,
                '\'' => Token::Apostrophe,
                '*' => Token::Asterisk,
                v => panic!("Unable to handle token: {:?}", v)
            });
        }
        panic!("Error - Received no token but expected a whitespace")
    }

    fn read_alphabetic_token(&mut self) -> Option<Token> {
        let mut result = String::new();
        while self.inner.peek().map_or_else(|| false, |x| x.is_alphanumeric()) {
            let next = self.inner.next().unwrap();
            result.push(next)
        }

        match KeywordToken::try_from(result.as_str()) {
            Ok(v) => Some(Token::Keyword(v)),
            Err(v) => {
                trace!("Error while reading the keyword [{:?}], defaulting to identifier", v);
                Some(Token::Identifier(result))
            }
        }
    }

    fn read_int_lit_token(&mut self) -> Option<Token> {
        let mut result = 0;

        while self.inner.peek().map_or_else(|| false, |x| x.is_digit(10)) {
            let next = self.inner.next().unwrap();
            result = (result * 10) + next.to_digit(10).unwrap()
        }

        return Some(Token::U32(result));
    }
}

impl<T: Iterator<Item=char>> Iterator for TokenIterator<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if let Some(&c) = self.inner.peek() {
            trace!("Peeked a char: [{}]", c);
            if c.is_digit(10) {
                return self.read_int_lit_token();
            }
            if c.is_alphabetic() {
                return self.read_alphabetic_token();
            }
            return self.read_symbol();
        }

        return None;
    }
}

