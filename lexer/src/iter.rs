use crate::token::LexerItem;
use core::iter::Peekable;
use unicode_segmentation::GraphemeIndices;
pub struct LexerIter<'a> {
    pub(crate) source: &'a str,
    pub(crate) graphemes: Peekable<GraphemeIndices<'a>>,
    pub(crate) cursor: usize,
}
impl<'a> Iterator for LexerIter<'a> {
    type Item = LexerItem<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}
