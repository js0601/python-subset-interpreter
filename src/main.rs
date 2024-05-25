mod common;
mod parser;
mod scanner;

use std::{
    cmp::Ordering,
    env,
    fs::read_to_string,
    io::{self, Write},
};

fn main() -> Result<(), io::Error> {
    let args: Vec<_> = env::args().collect();
    match args.len().cmp(&2) {
        // no arguments starts REPL
        Ordering::Less => repl()?,
        // one argument is assumed to be file path
        Ordering::Equal => {
            let path = args.get(1).unwrap();
            let code = read_to_string(path)?;
            run(code);
        }
        Ordering::Greater => println!("Usage: cargo run [path]"),
    }

    Ok(())
}

// TODO: what about multi-line e.g. if, while, def?
fn repl() -> Result<(), io::Error> {
    let mut line = String::new();
    loop {
        line.clear();
        print!(">>> ");
        io::stdout().flush().expect("flush failed");
        match io::stdin().read_line(&mut line) {
            // quit on EOF (ctrl-D / ctrl-Z)
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        run(line.clone());
    }
    Ok(())
}

// TODO: scan, parse and run the code
fn run(code: String) {
    let tokens;
    if let Some(t) = scanner::scan(code) {
        tokens = t;
    } else {
        return;
    }

    // print tokens
    for t in &tokens {
        println!("{:?}, {}, {}", t.token_type, t.line, t.column);
    }

    let e = parser::parse(tokens);
    println!("{e:?}");
}
