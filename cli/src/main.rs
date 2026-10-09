use openlab::parser::parser::Block;
use std::env::args;
fn main() {
    let mut args = args();
    args.next();
    let expr = args.next().unwrap();
    let block = Block::parse(&expr);
    println!("{block:#?}");
}
