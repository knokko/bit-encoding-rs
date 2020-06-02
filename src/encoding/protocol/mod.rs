use crate::*;

mod simple;

pub use simple::*;

pub trait EncodingProtocol {
    fn write_u8(&self, sink: &mut dyn BitSink, value: u8) -> Result<(), WriteError>;

    fn write_i8(&self, sink: &mut dyn BitSink, value: i8) -> Result<(), WriteError>;

    fn write_u16(&self, sink: &mut dyn BitSink, value: u16) -> Result<(), WriteError>;

    fn write_i16(&self, sink: &mut dyn BitSink, value: i16) -> Result<(), WriteError>;

    fn write_u32(&self, sink: &mut dyn BitSink, value: u32) -> Result<(), WriteError>;

    fn write_i32(&self, sink: &mut dyn BitSink, value: i32) -> Result<(), WriteError>;

    fn write_u64(&self, sink: &mut dyn BitSink, value: u64) -> Result<(), WriteError>;

    fn write_i64(&self, sink: &mut dyn BitSink, value: i64) -> Result<(), WriteError>;

    fn write_u128(&self, sink: &mut dyn BitSink, value: u128) -> Result<(), WriteError>;

    fn write_i128(&self, sink: &mut dyn BitSink, value: i128) -> Result<(), WriteError>;
}

/*
 * For some reason, I get dead code warnings for all methods in the testing module
 * unless I allow it like here.
 */
#[allow(dead_code)]
mod testing {

    use crate::*;

    pub fn test_encoding_pair(encoder: &dyn EncodingProtocol, decoder: &dyn DecodingProtocol) {
        test_u8(encoder, decoder);
        test_i8(encoder, decoder);
        test_u16(encoder, decoder);
        test_i16(encoder, decoder);
    }

    fn test_u8(encoder: &dyn EncodingProtocol, decoder: &dyn DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(256 * 8);
        for value in 0..=255 {
            encoder.write_u8(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in 0..=255 {
            assert_eq!(value, decoder.read_u8(&mut source).unwrap());
        }
    }

    fn test_i8(encoder: &dyn EncodingProtocol, decoder: &dyn DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(256 * 8);
        for value in -128..=127 {
            encoder.write_i8(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in -128..=127 {
            assert_eq!(value, decoder.read_i8(&mut source).unwrap());
        }
    }

    fn test_u16(encoder: &dyn EncodingProtocol, decoder: &dyn DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(65536 * 16);
        for value in 0..=65535 {
            encoder.write_u16(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in 0..=65535 {
            assert_eq!(value, decoder.read_u16(&mut source).unwrap());
        }
    }

    fn test_i16(encoder: &dyn EncodingProtocol, decoder: &dyn DecodingProtocol) {
        let mut sink = BoolVecBitSink::with_capacity(656536 * 16);
        for value in -32768..=32767 {
            encoder.write_i16(&mut sink, value).unwrap();
        }

        let mut source = BoolSliceBitSource::new(sink.get_bits());
        for value in -32768..=32767 {
            assert_eq!(value, decoder.read_i16(&mut source).unwrap());
        }
    }

    fn test_result(action: &mut dyn FnMut(&mut dyn BitSink), encoded: &str) {
        for character in encoded.chars() {
            assert!(character == '0' || character == '1');
        }

        let as_bools: Vec<bool> = encoded
            .chars()
            .map(|c| if c == '0' { false } else { true })
            .collect();
        let mut sink = BoolVecBitSink::with_capacity(as_bools.len());

        action(&mut sink);

        assert_eq!(as_bools, sink.get_bits());
    }

    pub fn test_u8_result(encoder: &dyn EncodingProtocol, value: u8, encoded: &str) {
        test_result(&mut |sink| encoder.write_u8(sink, value).unwrap(), encoded);
    }
}