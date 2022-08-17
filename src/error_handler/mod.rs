struct Error {
    message: String,
    line: usize,
    column: usize,
}

impl Error {
    fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }
}
