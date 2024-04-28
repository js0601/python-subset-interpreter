use std::fmt;

pub struct PyError {
    pub msg: String,
    pub line: u64,
    pub column: u64,
}

impl fmt::Display for PyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n    Line {}, Column {}",
            self.msg, self.line, self.column
        )
    }
}
