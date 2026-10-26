use crate::compute::Compute;
use parser::functions::Function;
use parser::number::matrix::dense::Matrix;
use parser::operators::Operator;
use parser::options::ParserOptions;
use parser::parser::{Block, BlockToken, Expression, ExpressionToken, Number};
use parser::units::{Dimension, DistanceUnit, MetricPrefix, Prefix, TimeUnit, Unit};
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
fn units(options: ParserOptions<'_>, prefix: Prefix, unit: Unit) -> Number {
    Number::Complex(Dimension::number_inner(options, prefix, unit).unwrap())
}
fn expr_number(f: f64) -> ExpressionToken<'static> {
    ExpressionToken::Number(number(f))
}
fn expr_units(options: ParserOptions<'_>, prefix: Prefix, unit: Unit) -> ExpressionToken<'static> {
    ExpressionToken::Number(units(options, prefix, unit))
}
fn expr_fun(f: Function) -> ExpressionToken<'static> {
    ExpressionToken::Function(f)
}
fn expr_op(o: Operator) -> ExpressionToken<'static> {
    expr_fun(Function::Operator(o))
}
fn mat<const N: usize, const M: usize>(mat: [[f64; N]; M]) -> Number {
    Number::Matrix(Matrix::from(mat))
}
fn assert_block(options: ParserOptions<'_>, s: &str, block: Block<'_>, res: Option<Number>) {
    assert_eq!(Block::parse(options, s).unwrap(), block);
    let mut compute = Compute::from_block(block);
    assert_eq!(compute.compute(), res);
}
#[test]
fn parse_add() {
    assert_block(
        ParserOptions::new(None),
        "1+2+3",
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
#[test]
fn parse_exp() {
    assert_block(
        ParserOptions::new(None),
        "1^2^3",
        block([print_expr([
            expr_number(1.0),
            expr_number(2.0),
            expr_number(3.0),
            expr_op(Operator::Pow),
            expr_op(Operator::Pow),
        ])]),
        Some(number(1.0)),
    )
}
#[test]
fn parse_units() {
    let options = ParserOptions::new(None);
    assert_block(
        options,
        "2 m / s / km",
        block([print_expr([
            expr_number(2.0),
            expr_units(
                options,
                Prefix::None,
                Unit::DistanceUnit(DistanceUnit::Meter),
            ),
            expr_op(Operator::MulInner),
            expr_units(options, Prefix::None, Unit::TimeUnit(TimeUnit::Second)),
            expr_op(Operator::Div),
            expr_units(
                options,
                Prefix::Metric(MetricPrefix::Kilo),
                Unit::DistanceUnit(DistanceUnit::Meter),
            ),
            expr_op(Operator::Div),
        ])]),
        Some(
            (number(2.0 / 1000.0) / units(options, Prefix::None, Unit::TimeUnit(TimeUnit::Second)))
                .unwrap(),
        ),
    )
}
#[test]
fn parse_vector() {
    let options = ParserOptions::new(None);
    assert_block(
        options,
        "[2,3,4]",
        block([print_expr([
            expr_number(2.0),
            expr_number(3.0),
            expr_number(4.0),
            expr_fun(Function::Vector(3)),
        ])]),
        Some(mat([[2.0, 3.0, 4.0]])),
    )
}
#[test]
fn parse_mat() {
    let options = ParserOptions::new(None);
    assert_block(
        options,
        "[[2,3,4],[2,1,3]]",
        block([print_expr([
            expr_number(2.0),
            expr_number(3.0),
            expr_number(4.0),
            expr_fun(Function::Vector(3)),
            expr_number(2.0),
            expr_number(1.0),
            expr_number(3.0),
            expr_fun(Function::Vector(3)),
            expr_fun(Function::Vector(2)),
        ])]),
        Some(mat([[2.0, 3.0, 4.0], [2.0, 1.0, 3.0]])),
    )
}
