use openlab::compute::Compute;
use openlab::parser::lexer::iter::LexerIter;
use openlab::parser::options::ParserOptions;
use openlab::parser::parser::Block;
use std::env::args;
fn main() {
    let mut args = args();
    args.next();
    let expr = args.next().unwrap();
    let config_dir = dirs::config_dir().map(|c| c.join("openlab"));
    let config_dir_path = config_dir.as_deref();
    println!("{:#?}", LexerIter::new(&expr).collect::<Vec<_>>());
    let options = ParserOptions::new(config_dir_path);
    let block = Block::parse(options, &expr);
    println!("{block:#?}");
    if let Ok(inner) = block {
        let tmr = std::time::Instant::now();
        let compute = Compute::stdout().compute(&inner);
        let time = tmr.elapsed().as_nanos();
        println!("{compute:#?}");
        println!("{time}");
    }
}
