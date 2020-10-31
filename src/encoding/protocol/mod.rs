use crate::*;

mod digit;
mod simple;

pub use digit::*;
pub use simple::*;

/// A protocol for encoding simple data types (integers, floating point numbers,
/// strings...) into a *BitSink*. Every implementation of this trait should have
/// a corresponding *DecodingProtocol* that can decode the data types that were
/// encoded by this encoder.
///
/// A simple implementation of *EncodingProtocol* would for instance encode
/// integers using their binary representation (writing 32 bools to encode
/// a u32 value). In fact, this is what *SimpleEncodingProtocol* does.
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
/// fn encode_some_data(encoder: &impl EncodingProtocol, sink: &mut impl BitSink) {
///     encoder.write_u8(sink, 12).unwrap();
///     encoder.write_i32(sink, 1234).unwrap();
///     encoder.write_i16(sink, -6789).unwrap();
/// }
///
/// fn decode_that_data(decoder: &impl DecodingProtocol, source: &mut impl BitSource){
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
/// operations, which could fail. Furthermore, the *DecodingProtocol* has to
/// be careful because it might deal with user input.
pub trait EncodingProtocol {
    /// Encodes the given u8 value and writes it to *sink*
    fn write_u8(&self, sink: &mut impl BitSink, value: u8) -> Result<(), WriteError>;

    /// Encodes the given i8 value and writes it to *sink*
    fn write_i8(&self, sink: &mut impl BitSink, value: i8) -> Result<(), WriteError>;

    /// Encodes the given u16 value and writes it to *sink*
    fn write_u16(&self, sink: &mut impl BitSink, value: u16) -> Result<(), WriteError>;

    /// Encodes the given i16 value and writes it to *sink*
    fn write_i16(&self, sink: &mut impl BitSink, value: i16) -> Result<(), WriteError>;

    /// Encoding the given u32 value and writes it to *sink*
    fn write_u32(&self, sink: &mut impl BitSink, value: u32) -> Result<(), WriteError>;

    /// Encoding the given i32 value and writes it to *sink*
    fn write_i32(&self, sink: &mut impl BitSink, value: i32) -> Result<(), WriteError>;

    /// Encodes the given u64 value and writes it to *sink*
    fn write_u64(&self, sink: &mut impl BitSink, value: u64) -> Result<(), WriteError>;

    /// Encodes the given i64 value and writes it to *sink*
    fn write_i64(&self, sink: &mut impl BitSink, value: i64) -> Result<(), WriteError>;

    /// Encodes the given u128 value and writes it to *sink*
    fn write_u128(&self, sink: &mut impl BitSink, value: u128) -> Result<(), WriteError>;

    /// Encodes the given i128 value and writes it to *sink*
    fn write_i128(&self, sink: &mut impl BitSink, value: i128) -> Result<(), WriteError>;
}

/*
 * For some reason, I get dead code warnings for all methods in the testing module
 * unless I allow it like here.
 */
#[cfg(test)]
pub(crate) mod testing {

    use crate::*;

    use rand::distributions::Standard;
    use rand::prelude::*;

    pub fn test_encoding_pair(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_u8(encoder, decoder);
        test_i8(encoder, decoder);
        test_u16(encoder, decoder);
        test_i16(encoder, decoder);
        test_u32(encoder, decoder);
        test_i32(encoder, decoder);
        test_u64(encoder, decoder);
        test_i64(encoder, decoder);
        test_u128(encoder, decoder);
        test_i128(encoder, decoder);

        // Combining them...
        let mut sink = BoolVecBitSink::new();
        write_combined(&mut sink, encoder).unwrap();

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        read_combined(&mut source, decoder).unwrap();
    }

    fn write_combined(sink: &mut impl BitSink, encoder: &impl EncodingProtocol) -> Result<(), WriteError> {
        encoder.write_i128(sink, -123456)?;
        sink.write(&[true, false])?;
        encoder.write_i64(sink, 987654321)?;
        encoder.write_i32(sink, -13579)?;
        encoder.write_i16(sink, 24680)?;
        encoder.write_i8(sink, 123)?;
        sink.write(&[false])?;
        encoder.write_u8(sink, 200)?;
        encoder.write_u16(sink, 23456)?;
        encoder.write_u32(sink, 123456789)?;
        sink.write(&[true])?;
        encoder.write_u64(sink, 0)?;
        encoder.write_u128(sink, 345678901234567)?;
        Ok(())
    }

    fn read_combined(source: &mut impl BitSource, decoder: &impl DecodingProtocol) -> Result<(), DecodeError> {
        assert_eq!(-123456, decoder.read_i128(source)?);
        let mut dest = [false; 2];
        source.read(&mut dest).unwrap();
        assert_eq!(&[true, false], &dest);
        assert_eq!(987654321, decoder.read_i64(source)?);
        assert_eq!(-13579, decoder.read_i32(source)?);
        assert_eq!(24680, decoder.read_i16(source)?);
        assert_eq!(123, decoder.read_i8(source)?);
        let mut dest = [true];
        source.read(&mut dest).unwrap();
        assert!(!dest[0]);
        assert_eq!(200, decoder.read_u8(source)?);
        assert_eq!(23456, decoder.read_u16(source)?);
        assert_eq!(123456789, decoder.read_u32(source)?);
        source.read(&mut dest).unwrap();
        assert!(dest[0]);
        assert_eq!(0, decoder.read_u64(source)?);
        assert_eq!(345678901234567, decoder.read_u128(source)?);
        Ok(())
    }

    fn test_u8(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(256 * 8);
        for value in 0..=255 {
            encoder.write_u8(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in 0..=255 {
            assert_eq!(value, decoder.read_u8(&mut source).unwrap());
        }
    }

    fn test_i8(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(256 * 8);
        for value in -128..=127 {
            encoder.write_i8(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in -128..=127 {
            assert_eq!(value, decoder.read_i8(&mut source).unwrap());
        }
    }

    fn test_u16(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(65536 * 16);
        for value in 0..=65535 {
            encoder.write_u16(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in 0..=65535 {
            assert_eq!(value, decoder.read_u16(&mut source).unwrap());
        }
    }

    fn test_i16(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(656536 * 16);
        for value in -32768..=32767 {
            encoder.write_i16(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in -32768..=32767 {
            assert_eq!(value, decoder.read_i16(&mut source).unwrap());
        }
    }

    const RANDOM_AMOUNT: usize = 10_000;

    fn test_random_symmetry<T: Copy + Eq + std::fmt::Debug>(
        write_method: impl Fn(&mut BoolVecBitSink, T) -> Result<(), WriteError>,
        read_method: impl Fn(&mut BoolSliceBitSource) -> Result<T, DecodeError>,
    ) where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        let mut values = Vec::with_capacity(RANDOM_AMOUNT);
        for _counter in 0..RANDOM_AMOUNT {
            values.push(rng.gen());
        }

        let mut sink = BoolVecBitSink::new();
        for value in &values {
            write_method(&mut sink, *value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in &values {
            assert_eq!(*value, read_method(&mut source).unwrap());
        }
    }

    fn test_given_symmetry<T: Copy + Eq + std::fmt::Debug>(
        values: &[T],
        write_method: impl Fn(&mut BoolVecBitSink, T) -> Result<(), WriteError>,
        read_method: impl Fn(&mut BoolSliceBitSource) -> Result<T, DecodeError>,
    ) {
        let mut sink = BoolVecBitSink::new();
        for value in values {
            write_method(&mut sink, *value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in values {
            assert_eq!(*value, read_method(&mut source).unwrap());
        }
    }

    fn test_u32(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, u32::max_value()],
            |sink, value| encoder.write_u32(sink, value),
            |source| decoder.read_u32(source),
        );

        test_random_symmetry(|sink, value| encoder.write_u32(sink, value), |source| {
            decoder.read_u32(source)
        });
    }

    fn test_i32(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, -1, i32::max_value(), i32::min_value()],
            |sink, value| encoder.write_i32(sink, value),
            |source| decoder.read_i32(source),
        );

        test_random_symmetry(|sink, value| encoder.write_i32(sink, value), |source| {
            decoder.read_i32(source)
        });
    }

    fn test_u64(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, u64::max_value()],
            |sink, value| encoder.write_u64(sink, value),
            |source| decoder.read_u64(source),
        );

        test_random_symmetry(|sink, value| encoder.write_u64(sink, value), |source| {
            decoder.read_u64(source)
        });
    }

    fn test_i64(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, -1, i64::max_value(), i64::min_value()],
            |sink, value| encoder.write_i64(sink, value),
            |source| decoder.read_i64(source),
        );

        test_random_symmetry(|sink, value| encoder.write_i64(sink, value), |source| {
            decoder.read_i64(source)
        });
    }

    fn test_u128(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, u128::max_value()],
            |sink, value| encoder.write_u128(sink, value),
            |source| decoder.read_u128(source),
        );

        test_random_symmetry(|sink, value| encoder.write_u128(sink, value), |source| {
            decoder.read_u128(source)
        });
    }

    fn test_i128(encoder: &impl EncodingProtocol, decoder: &impl DecodingProtocol) {
        test_given_symmetry(
            &[0, 1, -1, i128::max_value(), i128::min_value()],
            |sink, value| encoder.write_i128(sink, value),
            |source| decoder.read_i128(source),
        );

        test_random_symmetry(|sink, value| encoder.write_i128(sink, value), |source| {
            decoder.read_i128(source)
        });
    }

    fn test_result(
        action: &mut dyn FnMut(&mut BoolVecBitSink) -> Result<(), WriteError>,
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
        let mut sink = BoolVecBitSink::with_capacity(as_bools.len());

        action(&mut sink).unwrap();

        assert_eq!(as_bools, sink.get_bits());
    }

    pub fn test_u8_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: u8,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_u8(sink, value), encoded);
        decoding::testing::test_u8_result(decoder, value, encoded);
    }

    pub fn test_i8_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: i8,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_i8(sink, value), encoded);
        decoding::testing::test_i8_result(decoder, value, encoded);
    }

    pub fn test_u16_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: u16,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_u16(sink, value), encoded);
        decoding::testing::test_u16_result(decoder, value, encoded);
    }

    pub fn test_i16_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: i16,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_i16(sink, value), encoded);
        decoding::testing::test_i16_result(decoder, value, encoded);
    }

    pub fn test_u32_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: u32,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_u32(sink, value), encoded);
        decoding::testing::test_u32_result(decoder, value, encoded);
    }

    pub fn test_i32_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: i32,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_i32(sink, value), encoded);
        decoding::testing::test_i32_result(decoder, value, encoded);
    }

    pub fn test_u64_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: u64,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_u64(sink, value), encoded);
        decoding::testing::test_u64_result(decoder, value, encoded);
    }

    pub fn test_i64_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: i64,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_i64(sink, value), encoded);
        decoding::testing::test_i64_result(decoder, value, encoded);
    }

    pub fn test_u128_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: u128,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_u128(sink, value), encoded);
        decoding::testing::test_u128_result(decoder, value, encoded);
    }

    pub fn test_i128_result(
        encoder: &impl EncodingProtocol,
        decoder: &impl DecodingProtocol,
        value: i128,
        encoded: &str,
    ) {
        test_result(&mut |sink| encoder.write_i128(sink, value), encoded);
        decoding::testing::test_i128_result(decoder, value, encoded);
    }
}
