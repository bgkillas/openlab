use crate::token::LexerToken;
use core::iter::Peekable;
use unicode_segmentation::GraphemeIndices;
pub struct LexerTokenIter<'a> {
    pub(crate) source: &'a str,
    pub(crate) graphemes: Peekable<GraphemeIndices<'a>>,
    pub(crate) cursor: usize,
}
impl<'a> Iterator for LexerTokenIter<'a> {
    type Item = LexerToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}
