use parser::options::ParserOptions;
use parser::parser::{Block, BlockToken, Number};
fn block<const N: usize>(blocks: [BlockToken<'_>; N]) -> Block {
    Block {
        blocks: Box::new(blocks),
    }
}
fn expr<'a>(options: ParserOptions<'a>, s: &'a str) -> BlockToken<'a> {
    BlockToken::parse_expression_str(options, s).unwrap()
}
fn assert_block(options: ParserOptions<'_>, s: &str, block: Block, res: Number) {
    assert_eq!(Block::parse(options, s).unwrap(), block);
}
#[test]
fn number() {
    let s = "0<4>121";
    let options = ParserOptions::new(None);
    assert_eq!(Block::parse(options, s).unwrap(), block([expr(options, s)]));
}
