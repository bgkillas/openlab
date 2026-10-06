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
#[test]
pub fn num() {
    let iter = TokenIter::new("hi 5.5 who 0xff wow 0o212.11");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[
            Token::Word("hi"),
            Token::Numeric(None, 5, Some(5)),
            Token::Word("who"),
            Token::Numeric(Some(16), 255, None),
            Token::Word("wow"),
            Token::Numeric(Some(8), 138, Some(9))
        ]
    )
}
#[test]
pub fn num_arbitrary() {
    let iter = TokenIter::new("0<6>35132");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[Token::Numeric(Some(6), 5024, None),]
    );
    let iter = TokenIter::new("0<36>ai08vgzz");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[Token::Numeric(Some(36), 822838628303, None),]
    )
}
