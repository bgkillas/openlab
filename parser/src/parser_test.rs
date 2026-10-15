use crate::parser::Block;
//fn foo(a: M, b: C) -> (M, C) {
//    return (a, b);
//}
#[test]
fn number() {
    let s = "0<4>121";
    let _ = Block::parse(s).unwrap();
    //TODO
}
