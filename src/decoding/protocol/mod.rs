use crate::*;

mod simple;

pub use simple::*;

pub trait DecodingProtocol {
    fn read_u8(&self, source: &mut dyn BitSource) -> Result<u8, DecodeError>;

    fn read_i8(&self, source: &mut dyn BitSource) -> Result<i8, DecodeError>;

    fn read_u16(&self, source: &mut dyn BitSource) -> Result<u16, DecodeError>;

    fn read_i16(&self, source: &mut dyn BitSource) -> Result<i16, DecodeError>;

    fn read_u32(&self, source: &mut dyn BitSource) -> Result<u32, DecodeError>;

    fn read_i32(&self, source: &mut dyn BitSource) -> Result<i32, DecodeError>;

    fn read_u64(&self, source: &mut dyn BitSource) -> Result<u64, DecodeError>;

    fn read_i64(&self, source: &mut dyn BitSource) -> Result<i64, DecodeError>;

    fn read_u128(&self, source: &mut dyn BitSource) -> Result<u128, DecodeError>;

    fn read_i128(&self, source: &mut dyn BitSource) -> Result<i128, DecodeError>;
}

pub mod testing {

    use crate::*;

    fn test_result<R: Eq + std::fmt::Debug>(
        action: &mut dyn FnMut(&mut dyn BitSource) -> Result<R,DecodeError>, value: R, encoded: &str) {
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

    pub fn test_u8_result(decoder: &dyn DecodingProtocol, value: u8, encoded: &str) {
        test_result(&mut |source| decoder.read_u8(source), value, encoded);
    }

    pub fn test_i8_result(decoder: &dyn DecodingProtocol, value: i8, encoded: &str) {
        test_result(&mut |source | decoder.read_i8(source), value, encoded);
    }

    pub fn test_u16_result(decoder: &dyn DecodingProtocol, value: u16, encoded: &str) {
        test_result(&mut |source| decoder.read_u16(source), value, encoded);
    }

    pub fn test_i16_result(decoder: &dyn DecodingProtocol, value: i16, encoded: &str) {
        test_result(&mut |source| decoder.read_i16(source), value, encoded);
    }

    pub fn test_u32_result(decoder: &dyn DecodingProtocol, value: u32, encoded: &str) {
        test_result(&mut |source| decoder.read_u32(source), value, encoded);
    }

    pub fn test_i32_result(decoder: &dyn DecodingProtocol, value: i32, encoded: &str) {
        test_result(&mut |source| decoder.read_i32(source), value, encoded);
    }

    pub fn test_u64_result(decoder: &dyn DecodingProtocol, value: u64, encoded: &str) {
        test_result(&mut |source| decoder.read_u64(source), value, encoded);
    }

    pub fn test_i64_result(decoder: &dyn DecodingProtocol, value: i64, encoded: &str) {
        test_result(&mut |source| decoder.read_i64(source), value, encoded);
    }

    pub fn test_u128_result(decoder: &dyn DecodingProtocol, value: u128, encoded: &str) {
        test_result(&mut |source| decoder.read_u128(source), value, encoded);
    }

    pub fn test_i128_result(decoder: &dyn DecodingProtocol, value: i128, encoded: &str) {
        test_result(&mut |source| decoder.read_i128(source), value, encoded);
    }
}