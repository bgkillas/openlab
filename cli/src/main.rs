use openlab::compute::Compute;
use openlab::parser::options::ParserOptions;
use openlab::parser::parser::Block;
use std::env::args;
fn main() {
    let mut args = args();
    args.next();
    let expr = args.next().unwrap();
    let config_dir = dirs::config_dir().map(|c| c.join("openlab"));
    let config_dir_path = config_dir.as_deref();
    let options = ParserOptions::new(config_dir_path);
    if expr.starts_with("./") || expr.starts_with('/') {
        if let Ok(dir) = std::fs::read_dir(&expr) {
            for file in dir {
                let str = std::fs::read_to_string(file.unwrap().path()).unwrap();
                run_expr(options, &str);
            }
        } else {
            let str = std::fs::read_to_string(expr).unwrap();
            run_expr(options, &str);
        }
    } else {
        run_expr(options, &expr);
    }
}
fn run_expr(options: ParserOptions, expr: &str) {
    let block = Block::parse(options, expr);
    /*println!(
        "{:#?}",
        openlab::parser::lexer::iter::LexerIter::new(expr).collect::<Vec<_>>()
    );*/
    //println!("{block:#?}");
    if let Ok(inner) = block {
        let tmr = std::time::Instant::now();
        let compute = Compute::stdout().compute(&inner);
        let time = tmr.elapsed().as_nanos();
        println!("{compute:#?}");
        println!("{time}");
    }
}
