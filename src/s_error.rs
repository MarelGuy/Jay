pub(crate) struct SError {
    error_type: String,
    message: String,
}

// Simple error handler
impl SError {
    pub fn new(error_type: String, message: String) -> SError {
        SError {
            error_type,
            message,
        }
    }

    // This functio is just a normal placeholder, we will add the actual error handling later
    pub fn throw_error(&mut self) {
        println!("Error type: {}", self.error_type);
        println!("Error message: {}", self.message);
    }
}
