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
        let mut next;
        (self.cursor, _) = self.until(|s| !s.trim().is_empty());
        let (mut i, grapheme) = self.graphemes.next()?;
        next = i + grapheme.len();
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
                    Some(match g {
                        "x" => 16,
                        "o" => 8,
                        "b" => 2,
                        _ => unreachable!(),
                    })
                } else {
                    None
                };
                let (end, g) = self.until(|s| {
                    !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                });
                next = end + g.len();
                let fraction = if g == "." {
                    self.graphemes.next();
                    let (end_fraction, g_fraction) = self.until(|s| {
                        !matches!(s, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
                    });
                    next = end_fraction + g_fraction.len();
                    Some(
                        u128::from_str_radix(
                            &self.source[end + 1..end_fraction],
                            base.unwrap_or(10).strict_cast(),
                        )
                        .ok()?,
                    )
                } else {
                    None
                };
                let whole = u128::from_str_radix(
                    &self.source[self.cursor..end],
                    base.unwrap_or(10).strict_cast(),
                )
                .ok()?;
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
                (i, _) = self.graphemes.next()?;
                next = i + 1;
                Token::String(&self.source[self.cursor + 1..next - 1])
            }
            _ => {
                let (end, g) = self.until(|s| s.trim().is_empty());
                next = end + g.len();
                Token::Word(&self.source[self.cursor..end])
            }
        };
        self.cursor = next;
        Some(token)
    }
    pub fn until<F>(&mut self, mut stop: F) -> (usize, &str)
    where
        F: FnMut(&str) -> bool,
    {
        while let Some(&(i, grapheme)) = self.graphemes.peek() {
            if stop(grapheme) {
                return (i, grapheme);
            }
            self.graphemes.next();
        }
        (self.source.len(), "")
    }
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            graphemes: source.grapheme_indices(true).peekable(),
            cursor: 0,
        }
    }
}
