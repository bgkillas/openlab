use crate::iter::LexerIter;
use crate::token::LexerToken;
#[test]
pub fn str() {
    let iter = LexerIter::new("\" 32 4 \"");
    assert_eq!(&iter.collect::<Vec<_>>(), &[LexerToken::String(" 32 4 "),])
}
#[test]
pub fn num() {
    let iter = LexerIter::new("hi 5.5 who 0xff wow 0o212.11");
    assert_eq!(
        &iter.map(|(_, t)| t).collect::<Vec<_>>(),
        &[
            LexerToken::Word("hi"),
            LexerToken::Numeric(None, true, "5.5"),
            LexerToken::Word("who"),
            LexerToken::Numeric(Some(16), false, "ff"),
            LexerToken::Word("wow"),
            LexerToken::Numeric(Some(8), true, "212.11")
        ]
    )
}
#[test]
pub fn num_arbitrary() {
    let iter = LexerIter::new("0<6>35132");
    assert_eq!(
        &iter.map(|(_, t)| t).collect::<Vec<_>>(),
        &[LexerToken::Numeric(Some(6), false, "35132"),]
    );
    let iter = LexerIter::new("0<36>ai08vgzz");
    assert_eq!(
        &iter.map(|(_, t)| t).collect::<Vec<_>>(),
        &[LexerToken::Numeric(Some(36), false, "ai08vgzz"),]
    )
}
