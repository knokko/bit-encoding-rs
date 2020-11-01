use crate::*;

/// The simple implementation of *IntEncodingProtocol*. This implementation will
/// simply decode integers back from their binary representation (but always the
/// least significant bits first).
/// 
/// This implementation is ideal when every possible integer has an equal chance
/// to be stored, but not so great when some integers (for instance the small
/// integers) are much more common than the other integers.
/// 
/// The corresponding encoding protocol is *SimpleIntEncodingProtocol*.
pub struct SimpleIntDecodingProtocol {}

impl SimpleIntDecodingProtocol {
    pub const fn new() -> Self {
        SimpleIntDecodingProtocol {}
    }

    pub fn read_unsigned(
        &self,
        source: &mut impl BitSource,
        num_bits: usize,
    ) -> Result<u128, DecodeError> {
        let mut bits = vec![false; num_bits];
        source
            .read(&mut bits)
            .map_err(|read| DecodeError::Reading(read))?;

        let mut result = 0;
        for index in 0..num_bits {
            if bits[index] {
                result += 1 << index;
            }
        }

        Ok(result)
    }

    pub fn read_signed(
        &self,
        source: &mut impl BitSource,
        num_bits: usize,
    ) -> Result<i128, DecodeError> {
        let unsigned = self.read_unsigned(source, num_bits)?;
        if num_bits < 128 && unsigned >= 1 << num_bits {
            Ok((unsigned - (1 << num_bits)) as i128)
        } else {
            Ok(unsigned as i128)
        }
    }
}

impl IntDecodingProtocol for SimpleIntDecodingProtocol {
    fn read_u8(&self, source: &mut impl BitSource) -> Result<u8, DecodeError> {
        self.read_unsigned(source, 8).map(|x| x as u8)
    }

    fn read_i8(&self, source: &mut impl BitSource) -> Result<i8, DecodeError> {
        self.read_signed(source, 8).map(|x| x as i8)
    }

    fn read_u16(&self, source: &mut impl BitSource) -> Result<u16, DecodeError> {
        self.read_unsigned(source, 16).map(|x| x as u16)
    }

    fn read_i16(&self, source: &mut impl BitSource) -> Result<i16, DecodeError> {
        self.read_signed(source, 16).map(|x| x as i16)
    }

    fn read_u32(&self, source: &mut impl BitSource) -> Result<u32, DecodeError> {
        self.read_unsigned(source, 32).map(|x| x as u32)
    }

    fn read_i32(&self, source: &mut impl BitSource) -> Result<i32, DecodeError> {
        self.read_signed(source, 32).map(|x| x as i32)
    }

    fn read_u64(&self, source: &mut impl BitSource) -> Result<u64, DecodeError> {
        self.read_unsigned(source, 64).map(|x| x as u64)
    }

    fn read_i64(&self, source: &mut impl BitSource) -> Result<i64, DecodeError> {
        self.read_signed(source, 64).map(|x| x as i64)
    }

    fn read_u128(&self, source: &mut impl BitSource) -> Result<u128, DecodeError> {
        self.read_unsigned(source, 128).map(|x| x as u128)
    }

    fn read_i128(&self, source: &mut impl BitSource) -> Result<i128, DecodeError> {
        self.read_signed(source, 128).map(|x| x as i128)
    }
}

// This implementation doesn't have its own unit tests, but is instead tested
// alongside SimpleEncodingProtocol for more code reuse in tests.
