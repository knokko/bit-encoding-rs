use crate::*;

mod digit;
mod simple;

pub use digit::*;
pub use simple::*;

/// A protocol for decoding primitive integers from a *BitSource*. Every 
/// implementation of this trait should have a corresponding *IntEncodingProtocol* 
/// that encoded primitive integers to a *BitSink*.
///
/// A simple implementation of *DecodingProtocol* would for instance decode
/// integers from their binary representation (reading 32 bools to decode back
/// into a u32 value). In fact, this is what *SimpleDecodingProtocol* does.
///
/// Such a simple implementation is perfect when every value has the same chance
/// to be stored. However, smaller values are often more common, so a more clever
/// protocol would exploit this by using lesser bools to store smaller numbers.
///
/// When you have a corresponding pair of *EncodingProtocol* and
/// *DecodingProtocol*, you can use them like this:
///
/// ```
///
/// use bit_encoding::*;
///
/// fn encode_some_data(encoder: &impl IntEncodingProtocol, sink: &mut impl BitSink) {
///     encoder.write_u8(sink, 12).unwrap();
///     encoder.write_i32(sink, 1234).unwrap();
///     encoder.write_i16(sink, -6789).unwrap();
/// }
///
/// fn decode_that_data(decoder: &impl IntDecodingProtocol, source: &mut impl BitSource){
///     assert_eq!(12, decoder.read_u8(source).unwrap());
///     assert_eq!(1234, decoder.read_i32(source).unwrap());
///     assert_eq!(-6789, decoder.read_i16(source).unwrap());
/// }
/// ```
/// Note that the order of writes and reads must be the same and that *source*
/// should read from *sink*.
///
/// Also note that both the read and write methods return *Result*s. That is
/// because implementations of *BitSource* and *BitSink* can be backed by IO
/// operations, which could fail.
///
/// Also, when reading from user input, the
/// user may have given an invalid encoding, which is indicated by a
/// *DecodeError*. If you are reading from user input, you should catch these
/// kind of errors rather than unwrapping like in the example.
pub trait IntDecodingProtocol {
    /// Decodes a u8 value from the bits coming from *source*
    fn read_u8(&self, source: &mut impl BitSource) -> Result<u8, DecodeError>;

    /// Decoes an i18 value from the bits coming from *source*
    fn read_i8(&self, source: &mut impl BitSource) -> Result<i8, DecodeError>;

    /// Decodes a u16 value from the bits coming from *source*
    fn read_u16(&self, source: &mut impl BitSource) -> Result<u16, DecodeError>;

    /// Decodes an i16 value from the bits coming from *source*
    fn read_i16(&self, source: &mut impl BitSource) -> Result<i16, DecodeError>;

    /// Decodes a u32 value from the bits coming from *source*
    fn read_u32(&self, source: &mut impl BitSource) -> Result<u32, DecodeError>;

    /// Decodes an i32 value from the bits coming from *source*
    fn read_i32(&self, source: &mut impl BitSource) -> Result<i32, DecodeError>;

    /// Decodes a u64 value from the bits coming from *source*
    fn read_u64(&self, source: &mut impl BitSource) -> Result<u64, DecodeError>;

    /// Decodes an i64 value from the bits coming from *source*
    fn read_i64(&self, source: &mut impl BitSource) -> Result<i64, DecodeError>;

    /// Decodes a u128 value from the bits coming from *source*
    fn read_u128(&self, source: &mut impl BitSource) -> Result<u128, DecodeError>;

    /// Decodes an i128 value from the bits coming from *source*
    fn read_i128(&self, source: &mut impl BitSource) -> Result<i128, DecodeError>;
}

#[cfg(test)]
pub(crate) mod testing {

    use crate::*;

    fn test_result<R: Eq + std::fmt::Debug>(
        mut action: impl FnMut(&mut BoolSliceBitSource) -> Result<R, DecodeError>,
        value: R,
        encoded: &str,
    ) {
        for character in encoded.chars() {
            assert!(character == '0' || character == '1' || character == ' ');
        }

        let as_bools: Vec<bool> = encoded
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| if c == '0' { false } else { true })
            .collect();

        let mut source = BoolSliceBitSource::new(&as_bools);

        assert_eq!(value, action(&mut source).unwrap());
    }

    pub fn test_u8_result(decoder: &impl IntDecodingProtocol, value: u8, encoded: &str) {
        test_result(|source| decoder.read_u8(source), value, encoded);
    }

    pub fn test_i8_result(decoder: &impl IntDecodingProtocol, value: i8, encoded: &str) {
        test_result(|source| decoder.read_i8(source), value, encoded);
    }

    pub fn test_u16_result(decoder: &impl IntDecodingProtocol, value: u16, encoded: &str) {
        test_result(|source| decoder.read_u16(source), value, encoded);
    }

    pub fn test_i16_result(decoder: &impl IntDecodingProtocol, value: i16, encoded: &str) {
        test_result(|source| decoder.read_i16(source), value, encoded);
    }

    pub fn test_u32_result(decoder: &impl IntDecodingProtocol, value: u32, encoded: &str) {
        test_result(|source| decoder.read_u32(source), value, encoded);
    }

    pub fn test_i32_result(decoder: &impl IntDecodingProtocol, value: i32, encoded: &str) {
        test_result(|source| decoder.read_i32(source), value, encoded);
    }

    pub fn test_u64_result(decoder: &impl IntDecodingProtocol, value: u64, encoded: &str) {
        test_result(|source| decoder.read_u64(source), value, encoded);
    }

    pub fn test_i64_result(decoder: &impl IntDecodingProtocol, value: i64, encoded: &str) {
        test_result(|source| decoder.read_i64(source), value, encoded);
    }

    pub fn test_u128_result(decoder: &impl IntDecodingProtocol, value: u128, encoded: &str) {
        test_result(|source| decoder.read_u128(source), value, encoded);
    }

    pub fn test_i128_result(decoder: &impl IntDecodingProtocol, value: i128, encoded: &str) {
        test_result(|source| decoder.read_i128(source), value, encoded);
    }
}
