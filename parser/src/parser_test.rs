use crate::parser::Block;
//fn foo(a: M, b: C) -> (M, C) {
//    return (a, b);
//}
#[test]
fn number() {
    let s = "0<4>121.113";
    let block = Block::parse(s).unwrap();
    println!("{block:?}")
}
