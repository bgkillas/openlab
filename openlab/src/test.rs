use crate::compute::Compute;
use parser::functions::Function;
use parser::operators::Operator;
use parser::options::ParserOptions;
use parser::parser::{Block, BlockToken, Expression, ExpressionToken, Number};
fn block<const N: usize>(blocks: [BlockToken<'_>; N]) -> Block<'_> {
    Block {
        blocks: Box::new(blocks),
    }
}
#[expect(unused)]
fn pexpr<'a>(options: ParserOptions<'a>, s: &'a str) -> BlockToken<'a> {
    BlockToken::parse_expression_str(options, s).unwrap()
}
#[expect(unused)]
fn expr<const N: usize>(blocks: [ExpressionToken<'_>; N]) -> BlockToken<'_> {
    BlockToken::Expression(Expression {
        tokens: Box::new(blocks),
    })
}
fn print_expr<const N: usize>(blocks: [ExpressionToken<'_>; N]) -> BlockToken<'_> {
    BlockToken::PrintExpression(Expression {
        tokens: Box::new(blocks),
    })
}
fn number(f: f64) -> Number {
    Number::Complex(f.into())
}
fn expr_number(f: f64) -> ExpressionToken<'static> {
    ExpressionToken::Number(number(f))
}
fn expr_fun(f: Function) -> ExpressionToken<'static> {
    ExpressionToken::Function(f)
}
fn expr_op(o: Operator) -> ExpressionToken<'static> {
    expr_fun(Function::Operator(o))
}
fn assert_block(options: ParserOptions<'_>, s: &str, block: Block<'_>, res: Option<Number>) {
    assert_eq!(Block::parse(options, s).unwrap(), block);
    let mut compute = Compute::from_block(block);
    assert_eq!(compute.compute(), res);
}
#[test]
fn parse_add() {
    let options = ParserOptions::new(None);
    let s = "1+2+3";
    assert_block(
        options,
        s,
        block([print_expr([
            expr_number(1.0),
            expr_number(2.0),
            expr_op(Operator::Add),
            expr_number(3.0),
            expr_op(Operator::Add),
        ])]),
        Some(number(6.0)),
    )
}
