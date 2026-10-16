use openlab::compute::Compute;
use openlab::parser::options::ParserOptions;
use std::env::args;
fn main() {
    let mut args = args();
    args.next();
    let expr = args.next().unwrap();
    let config_dir = dirs::config_dir().map(|c| c.join("openlab"));
    let config_dir_path = config_dir.as_deref();
    let options = ParserOptions::new(config_dir_path);
    let block = Compute::parse(options, &expr);
    println!("{block:#?}");
    if let Ok(mut inner) = block {
        inner.compute();
    }
}
