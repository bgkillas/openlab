use crate::iter::LexerTokenIter;
use crate::token::LexerToken;
#[test]
pub fn str() {
    let iter = LexerTokenIter::new("\" 32 4 \"");
    assert_eq!(&iter.collect::<Vec<_>>(), &[LexerToken::String(" 32 4 "),])
}
#[test]
pub fn num() {
    let iter = LexerTokenIter::new("hi 5.5 who 0xff wow 0o212.11");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[
            LexerToken::Word("hi"),
            LexerToken::Numeric(None, 5, Some(5)),
            LexerToken::Word("who"),
            LexerToken::Numeric(Some(16), 255, None),
            LexerToken::Word("wow"),
            LexerToken::Numeric(Some(8), 138, Some(9))
        ]
    )
}
#[test]
pub fn num_arbitrary() {
    let iter = LexerTokenIter::new("0<6>35132");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[LexerToken::Numeric(Some(6), 5024, None),]
    );
    let iter = LexerTokenIter::new("0<36>ai08vgzz");
    assert_eq!(
        &iter.collect::<Vec<_>>(),
        &[LexerToken::Numeric(Some(36), 822838628303, None),]
    )
}
