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
        self.until(|s| !s.trim().is_empty());
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
                let end = self.until(|s| true);
                Token::Word(&self.source[self.cursor..end])
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
                Token::String(&self.source[self.cursor..next])
            }
            _ => {
                let end = self.until(|s| !s.trim().is_empty());
                Token::Word(&self.source[self.cursor..end])
            }
        };
        self.cursor = next;
        Some(token)
    }
    pub fn until<F>(&mut self, mut stop: F) -> usize
    where
        F: FnMut(&str) -> bool,
    {
        while let Some(&(i, grapheme)) = self.graphemes.peek() {
            if stop(grapheme) {
                return i;
            }
            self.graphemes.next();
        }
        self.source.len()
    }
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            graphemes: source.grapheme_indices(true).peekable(),
            cursor: 0,
        }
    }
}
