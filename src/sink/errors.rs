use std::fmt::Debug;

/// Represents an error that occurred during writing data to a BitSink. 
/// This is a struct with a single *error* field of type *Debug*. 
/// 
/// Because all kinds of errors could occur depending on the implementation,
/// the error field of this struct has the Debug type.
#[derive(Debug)]
pub struct WriteError {

    error: Box<dyn Debug>
}

impl WriteError {

    /// Constructs a new WriteError with the given error.
    pub fn new(error: Box<dyn Debug>) -> Self {
        Self {
            error
        }
    }

    /// Gets the error of this WriteError struct
    pub fn get_error(&self) -> &dyn Debug {
        &self.error
    }
}