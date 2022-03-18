/*
Jay error handler
Copyright (C) 2022  Loris Cuntreri
*/
pub(crate) struct SError {
    error_type: String,
    message: String,
}

// TODO: add a proper error handler
impl SError {
    pub fn new(error_type: String, message: String) -> SError {
        SError {
            error_type,
            message,
        }
    }

    pub fn throw_error(&mut self) {
        println!("Error type: {}", self.error_type);
        println!("Error message: {}", self.message);
    }
}
