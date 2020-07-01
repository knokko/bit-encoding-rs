use crate::*;

use std::error::Error;
use std::fmt::Display;

/// Represents an error that occurred while trying to decode the data read
/// from a `BitSource`.
///
/// Most of the errors in this enum indicate programming errors or a corrupted
/// `BitSource`, except for the `Reading` error, which indicates that an
/// error occurred during reading data rather than decoding it.
#[derive(Debug)]
pub enum DecodeError {
    /// Some methods of `BitDecoder` return a `Vec` of elements (for instance
    /// a `Vec` of `u32`s). In order to decode such a vector, its length will
    /// have to be decoded somehow. If that decoded length is very large,
    /// the decoder would try to allocate a very big `Vec`, which could cause
    /// the process to run out of memory.
    ///
    /// To prevent this, some implementations refuse to decode `Vec`s that
    /// would have a length that is greater than the maximum length they
    /// allow. If that occurs, this `BitVecLength` error will be returned
    /// and no attempt will be made to allocate so much memory.
    BigVecLength(LengthExceeded),

    /// Similarly to `BigVecLength`, some implementations of `BitDecoder`
    /// also have a maximum length for strings they are willing to decode.
    ///
    /// If that maximum length would be exceeded, this error will be
    /// returned.
    BigStringLength(LengthExceeded),

    /// Some methods of `BitDecoder` return a `Vec` of elements, for instance
    /// a `Vec<i16>`. The length of the vector will have to be decoded first.
    /// However, the decoded length might be greater than the maximum value
    /// of `usize`. If that occurs, this error will be returned.
    VecLengthOverflow { length: LengthType },

    /// When decoding a `String`, the length of that string will have to be
    /// decoded first. It is possible that the decoded length is greater than
    /// the maximum value of `usize`. If that occurs, this error will be
    /// returned.
    StringLengthOverflow { length: LengthType },

    /// This error indicates that an error occurred while reading the data
    /// needed to decode something.
    Reading(ReadError),
}

/// This indicates that some maximum length was exceeded during decoding
/// something. This struct stores both the maximum length and the
/// actual/read length.
#[derive(Debug)]
pub struct LengthExceeded {
    max_length: LengthType,
    read_length: LengthType,
}

impl LengthExceeded {
    pub fn new(max_length: LengthType, read_length: LengthType) -> Self {
        LengthExceeded {
            max_length,
            read_length,
        }
    }

    /// Gets the maximum length the `BitDecoder` was willing to decode
    pub fn get_max_length(&self) -> LengthType {
        self.max_length
    }

    /// Gets the length of the Vec, String (or whatever else) that was
    /// being decoded
    pub fn get_read_length(&self) -> LengthType {
        self.read_length
    }
}

impl Display for DecodeError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        match self {
            DecodeError::BigVecLength(length) => write!(f, 
            "The decoder was asked to decode a Vec, but the length of the vector
            appears to be {}, which is bigger than the maximum allowed length of
            {}.", length.read_length, length.max_length),

            DecodeError::BigStringLength(length) => write!(f,
            "The decoder was asked to decode a String, but the length of the
            string appears to be {}, which is bigger than the maximum allowed
            length of {}. ", length.read_length, length.max_length),

            DecodeError::VecLengthOverflow{length} => write!(f,
            "The decoder was asked to decode a Vec, but the length of the vector
            appears to be {}, which is bigger than the maximum value of the usize
            on this machine {}. ", length, usize::max_value()),

            DecodeError::StringLengthOverflow{length} => write!(f,
            "The decoder was asked to decode a String, but the length of the
            string appears to be {}, which is bigger than the maximum value of the
            usize on this machine {}. ", length, usize::max_value()),

            DecodeError::Reading(read_error) => write!(f,
            "The following error occurred inside the BitSource the decoder was
            reading from: {}", read_error)
        }
    }
}

impl Error for DecodeError {

}