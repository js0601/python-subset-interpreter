# Python Subset Interpreter

This was developed as the project for my bachelor's thesis (and graded with 1.0 😎).

## Usage
First [install Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html).

Clone the project:
```
git clone https://github.com/js0601/python-subset-interpreter.git
```

Change into the project directory and run:
```
cargo run -r [path]
```
to build and run with the specified file (e.g. examples/fib_list.py) or

```
cargo run -r
```
to use the (very basic) REPL.

To get an idea of what you can program in this subset, check out the [examples folder](examples). It's basically bare-bones Python without the syntactic sugar and standard library.

To see the output of the scanner and/or parser, you would need to uncomment the corresponding print statements in [main.rs](src/main.rs).

## Implementation
I only used the Rust standard library for this, so no additional crates or parser generators.

The recursive-descent parser implements the [grammar](grammar.txt).

For more implementation details, look at the [thesis](thesis.pdf).
