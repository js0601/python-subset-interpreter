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

// TODO: consider error handling of flush and read_line
// TODO: what about multi-line e.g. if or while?
fn repl() -> Result<(), io::Error> {
    loop {
        print!(">>> ");
        io::stdout().flush().expect("flush failed");
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        // TODO: change this to quit on EOF
        if line.trim() == "quit" {
            break;
        }

        // TODO: might need to trim here?
        run(line);
    }
    Ok(())
}

// TODO: scan, parse and run the code
fn run(code: String) {
    println!("scanning, parsing, rearranging\n{code}");
}
