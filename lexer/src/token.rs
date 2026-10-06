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
    Numeric(Option<u8>, u128, Option<u128>),
    String(&'a str),
    Word(&'a str),
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
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                let base: Option<u8> = if grapheme == "0"
                    && let Some(&(_, g)) = self.graphemes.peek()
                    && matches!(g, "x" | "o" | "b")
                {
                    self.graphemes.next();
                    self.graphemes.next();
                    self.cursor += 2;
                    start += 2;
                    Some(match g {
                        "x" => 16,
                        "o" => 8,
                        "b" => 2,
                        _ => unreachable!(),
                    })
                } else {
                    None
                };
                let g = self.until(|s| {
                    !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                });
                let has_deci = g == ".";
                let whole = u128::from_str_radix(
                    &self.source[start..self.cursor],
                    base.unwrap_or(10).strict_cast(),
                )
                .ok()?;
                let fraction = if has_deci {
                    start = self.cursor + 1;
                    self.graphemes.next();
                    if self.graphemes.peek().is_some_and(|&(_, s)| {
                        matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                    }) {
                        self.until(|s| {
                            !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                        });
                        Some(
                            u128::from_str_radix(
                                &self.source[start..self.cursor],
                                base.unwrap_or(10).strict_cast(),
                            )
                            .ok()?,
                        )
                    } else {
                        None
                    }
                } else {
                    None
                };
                Token::Numeric(base, whole, fraction)
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
