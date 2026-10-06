use crate::iter::TokenIter;
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
}
fn is_digit(base: u8, s: &str) -> bool {
    if !s.is_ascii() {
        return false;
    }
    let char = s.chars().next().unwrap();
    match base {
        2 => matches!(char, '0' | '1'),
        8 => char.is_ascii_octdigit(),
        16 => char.is_ascii_hexdigit(),
        _ => unreachable!(),
    }
}
impl<'a> TokenIter<'a> {
    pub fn token(&mut self) -> Option<Token<'a>> {
        self.until(|s| !s.trim().is_empty());
        let mut start = self.cursor;
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
            "0" if let Some(&(_, base_str)) = self.graphemes.peek()
                && matches!(base_str, "x" | "o" | "b") =>
            {
                self.graphemes.next();
                self.cursor += 2;
                start += 2;
                let base: u8 = match base_str {
                    "x" => 16,
                    "o" => 8,
                    "b" => 2,
                    _ => unreachable!(),
                };
                let g = self.until(|s| !is_digit(base, s));
                let has_deci = g == ".";
                let whole =
                    u128::from_str_radix(&self.source[start..self.cursor], base.strict_cast())
                        .ok()?;
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
                            u128::from_str_radix(
                                &self.source[start..self.cursor],
                                base.strict_cast(),
                            )
                            .ok()?,
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };
                Token::Numeric(Some(base), whole, fraction)
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let g = self.until(|s| {
                    !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                });
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
                Token::Numeric(None, whole, fraction)
            }
            "\"" => {
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
                Token::String(&self.source[start + 1..self.cursor - 1])
            }
            _ => {
                self.until(|s| s.trim().is_empty());
                Token::Word(&self.source[start..self.cursor])
            }
        };
        Some(token)
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
