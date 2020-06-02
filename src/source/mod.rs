mod bool_vec;
mod errors;

pub use bool_vec::*;
pub use errors::*;

/// A type from which bools can be read.
///
/// Typical implementations would read bools from a Vec of bools or binary
/// integers, but other implementations might read them from a file or socket
/// connection.
///
/// Every BitDecoder will have a BitSource from which it will read the data that
/// it decodes.
pub trait BitSource {
    /// Attempts to read bools from this source and put them into *dest*.
    ///
    /// The first bool read will be put in `dest[0]`, the second bool will be
    /// put into `dest[1]`, the third bool will be put into `dest[2]`...
    /// That will continue until *dest* has been filled completely.
    ///
    /// If enough bools could be read to fill *dest* completely, this method
    /// will return `Ok`. If all bools were read before *dest* was filled or
    /// if another error occurred while reading, a `ReadError` will be
    /// returned.
    fn read(&mut self, dest: &mut [bool]) -> Result<(), ReadError>;
}
