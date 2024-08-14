# Python Subset Interpreter

This is a tree-walk interpreter written in Rust for a subset of Python, developed as a part of my bachelor's thesis.

## Usage
First [install Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html).

Clone the project:
```
git clone https://github.com/js0601/python-subset-interpreter.git
```

Change into the project directory and run:
```
cargo run --release [path]
```
to build and run with the specified file (e.g. examples/fib_list.py) or

```
cargo run --release
```
to use the (very basic) REPL.
