/// Represents an error that occurred during writing data to a BitSink.
///
/// Currently, this is simply a *Box* containing an *Error* because
/// only implementation-specific errors can occur.
pub type WriteError = Box<dyn std::error::Error>;
