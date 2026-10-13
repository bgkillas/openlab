use openlab::compute::Compute;
use std::env::args;
fn main() {
    let mut args = args();
    args.next();
    let expr = args.next().unwrap();
    let block = Compute::parse(&expr);
    println!("{block:#?}");
    if let Ok(mut inner) = block {
        inner.compute();
    }
}
