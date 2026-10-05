use crate::iter::TokenIter;
use crate::token::Token;
#[test]
pub fn str() {
    let iter = TokenIter::new("hi wow   \" 32 4 \" 5 6");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[
            Token::Word("hi"),
            Token::Word("wow"),
            Token::String(" 32 4 "),
            Token::Numeric(None, 5, None),
            Token::Numeric(None, 6, None)
        ]
    )
}
