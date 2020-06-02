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
