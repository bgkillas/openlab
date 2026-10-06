use crate::iter::TokenIter;
use core::str::FromStr as _;
use unicode_segmentation::UnicodeSegmentation as _;
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Token<'a> {
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    LeftAngleBracket,
    RightAngleBracket,
    ExclamationMark,
    Caret,
    Asterisk,
    Minus,
    Plus,
    Equal,
    SemiColon,
    SingleQuote,
    Backtick,
    Tilde,
    Comma,
    Period,
    FowardSlash,
    Numeric(Option<u8>, u128, Option<u128>),
    String(&'a str),
    Word(&'a str),
    UnexpectedEnd,
}
fn is_digit(base: u8, s: &str) -> bool {
    if !s.is_ascii() {
        return false;
    }
    let char = s.chars().next().unwrap();
    match base {
        2..=10 => char >= '0' && char <= char::from(b'0' + base - 1),
        11..=36 => char.is_ascii_digit() || (char >= 'a' && char <= char::from(b'a' + (base - 11))),
        _ => unreachable!(),
    }
}
impl<'a> TokenIter<'a> {
    pub fn token(&mut self) -> Option<Token<'a>> {
        self.until(|s| !s.trim().is_empty());
        let (_, grapheme) = self.graphemes.next()?;
        let token = match grapheme {
            "{" => Token::LeftCurlyBracket,
            "}" => Token::RightCurlyBracket,
            "[" => Token::LeftSquareBracket,
            "]" => Token::RightSquareBracket,
            "(" => Token::LeftParenthesis,
            ")" => Token::RightParenthesis,
            "<" => Token::LeftAngleBracket,
            ">" => Token::RightAngleBracket,
            "!" => Token::ExclamationMark,
            "^" => Token::Caret,
            "*" => Token::Asterisk,
            "-" => Token::Minus,
            "+" => Token::Plus,
            "=" => Token::Equal,
            ";" => Token::SemiColon,
            "'" => Token::SingleQuote,
            "`" => Token::Backtick,
            "~" => Token::Tilde,
            "," => Token::Comma,
            "." => Token::Period,
            "/" => Token::FowardSlash,
            "0" if let Some(&(_, "<")) = self.graphemes.peek()
                && let Some((base_str, _)) = self.source[self.cursor + 2..].split_once('>')
                && !base_str.is_empty()
                && base_str.bytes().all(|b| b.is_ascii_digit())
                && let Ok(base) = u8::from_str(base_str)
                && matches!(base, 2..=36) =>
            {
                self.graphemes.nth(base_str.len() + 1);
                self.cursor += base_str.len() + 3;
                self.get_num_with_arbitrary(base)
                    .unwrap_or(Token::UnexpectedEnd)
            }
            "0" if let Some(&(_, base_str)) = self.graphemes.peek()
                && matches!(base_str, "x" | "o" | "b") =>
            {
                self.graphemes.next();
                self.cursor += 2;
                self.get_num_with(base_str).unwrap_or(Token::UnexpectedEnd)
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                self.get_num().unwrap_or(Token::UnexpectedEnd)
            }
            "\"" => self.get_string().unwrap_or(Token::UnexpectedEnd),
            _ => {
                let start = self.cursor;
                self.until(|s| s.trim().is_empty());
                Token::Word(&self.source[start..self.cursor])
            }
        };
        Some(token)
    }
    pub fn get_string(&mut self) -> Option<Token<'a>> {
        let start = self.cursor;
        let mut last_break = false;
        self.until(|s| match s {
            "\"" if !last_break => true,
            "\\" => {
                last_break = true;
                false
            }
            _ => {
                last_break = false;
                false
            }
        });
        (self.cursor, _) = self.graphemes.next()?;
        self.cursor += 1;
        Some(Token::String(&self.source[start + 1..self.cursor - 1]))
    }
    pub fn get_num(&mut self) -> Option<Token<'a>> {
        let mut start = self.cursor;
        let g =
            self.until(|s| !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"));
        let has_deci = g == ".";
        let whole = self.source[start..self.cursor].parse().ok()?;
        let fraction = if has_deci {
            start = self.cursor + 1;
            self.graphemes.next();
            if self.graphemes.peek().is_some_and(|&(_, s)| {
                matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            }) {
                self.until(|s| {
                    !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                });
                Some(self.source[start..self.cursor].parse().ok()?)
            } else {
                None
            }
        } else {
            None
        };
        Some(Token::Numeric(None, whole, fraction))
    }
    pub fn get_num_with(&mut self, base_str: &str) -> Option<Token<'a>> {
        let base: u8 = match base_str {
            "x" => 16,
            "o" => 8,
            "b" => 2,
            _ => unreachable!(),
        };
        self.get_num_with_arbitrary(base)
    }
    pub fn get_num_with_arbitrary(&mut self, base: u8) -> Option<Token<'a>> {
        let mut start = self.cursor;
        let g = self.until(|s| !is_digit(base, s));
        let has_deci = g == ".";
        let whole =
            u128::from_str_radix(&self.source[start..self.cursor], base.strict_cast()).ok()?;
        let fraction = if has_deci {
            start = self.cursor + 1;
            self.graphemes.next();
            if self
                .graphemes
                .peek()
                .is_some_and(|&(_, s)| is_digit(base, s))
            {
                self.until(|s| !is_digit(base, s));
                Some(
                    u128::from_str_radix(&self.source[start..self.cursor], base.strict_cast())
                        .ok()?,
                )
            } else {
                None
            }
        } else {
            None
        };
        Some(Token::Numeric(Some(base), whole, fraction))
    }
    pub fn until<F>(&mut self, mut stop: F) -> &str
    where
        F: FnMut(&str) -> bool,
    {
        while let Some(&(i, grapheme)) = self.graphemes.peek() {
            if stop(grapheme) {
                self.cursor = i;
                return grapheme;
            }
            self.graphemes.next();
        }
        self.cursor = self.source.len();
        ""
    }
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            graphemes: source.grapheme_indices(true).peekable(),
            cursor: 0,
        }
    }
}
