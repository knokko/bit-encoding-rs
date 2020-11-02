// Extra modules
mod errors;

// Implementations
mod bool_vec;
mod u32_vec;
mod u8_vec;
mod void;

// Export all as part of this module
pub use bool_vec::*;
pub use errors::*;
pub use u32_vec::*;
pub use u8_vec::*;
pub use void::*;

/// A type to which bools can be written.
///
/// Typical implementations of BitSink would store the bools written to it
/// so that they can be retrieved and used later on. Other implementations
/// of BitSink may send or store the data (almost) right away.
///
/// Every BitEncoder will have a BitSink to which it will write all its data
/// after converting it to sequences of bools.
pub trait BitSink {
    /// Writes the next slice of bools to this BitSink.
    /// Returns Ok if all bools were written successfully.
    /// Returns a WriteError if some error occurred during writing
    /// (for instance IO errors for implementations that write directly
    /// to disk or to a connection).
    fn write(&mut self, bits: &[bool]) -> Result<(), WriteError>;

    /// Marks this BitSink as finished.
    /// After this call, write must not be called anymore,
    /// or undefined behavior will occur (no memory safety or something, but write
    /// may panic, or just continue working).
    ///
    /// This method should be called after the user is done with writing
    /// to this BitSink. Implementations of BitSink may rely on this method
    /// being called, while others may ignore it completely.
    fn finish(&mut self) -> Result<(), WriteError>;

    /// Gets the total number of bools that have been written into this sink so
    /// far.
    fn get_num_bools(&self) -> u64;
}
