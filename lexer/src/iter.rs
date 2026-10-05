use crate::token::Token;
use core::iter::Peekable;
use unicode_segmentation::GraphemeIndices;
pub struct TokenIter<'a> {
    pub(crate) source: &'a str,
    pub(crate) graphemes: Peekable<GraphemeIndices<'a>>,
    pub(crate) cursor: usize,
}
impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}
