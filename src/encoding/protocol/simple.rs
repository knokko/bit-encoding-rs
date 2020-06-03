use crate::*;

pub struct SimpleEncodingProtocol {}

impl SimpleEncodingProtocol {
    pub const fn new() -> Self {
        SimpleEncodingProtocol {}
    }

    fn write_unsigned(
        &self,
        sink: &mut dyn BitSink,
        num_bits: usize,
        value: u128,
    ) -> Result<(), WriteError> {
        let mut bools = Vec::with_capacity(num_bits);
        for index in 0..num_bits {
            bools.push(value & 1 << index != 0);
        }
        sink.write(&bools)
    }

    fn write_signed(
        &self,
        sink: &mut dyn BitSink,
        num_bits: usize,
        mut value: i128,
    ) -> Result<(), WriteError> {
        if value < 0 {
            value += 1 << num_bits;
        }
        self.write_unsigned(sink, num_bits, value as u128)
    }
}

impl EncodingProtocol for SimpleEncodingProtocol {
    fn write_u8(&self, sink: &mut dyn BitSink, value: u8) -> Result<(), WriteError> {
        self.write_unsigned(sink, 8, value as u128)
    }

    fn write_i8(&self, sink: &mut dyn BitSink, value: i8) -> Result<(), WriteError> {
        self.write_signed(sink, 8, value as i128)
    }

    fn write_u16(&self, sink: &mut dyn BitSink, value: u16) -> Result<(), WriteError> {
        self.write_unsigned(sink, 16, value as u128)
    }

    fn write_i16(&self, sink: &mut dyn BitSink, value: i16) -> Result<(), WriteError> {
        self.write_signed(sink, 16, value as i128)
    }

    fn write_u32(&self, sink: &mut dyn BitSink, value: u32) -> Result<(), WriteError> {
        self.write_unsigned(sink, 32, value as u128)
    }

    fn write_i32(&self, sink: &mut dyn BitSink, value: i32) -> Result<(), WriteError> {
        self.write_signed(sink, 32, value as i128)
    }

    fn write_u64(&self, sink: &mut dyn BitSink, value: u64) -> Result<(), WriteError> {
        self.write_unsigned(sink, 64, value as u128)
    }

    fn write_i64(&self, sink: &mut dyn BitSink, value: i64) -> Result<(), WriteError> {
        self.write_signed(sink, 64, value as i128)
    }

    fn write_u128(&self, sink: &mut dyn BitSink, value: u128) -> Result<(), WriteError> {
        self.write_unsigned(sink, 128, value)
    }

    fn write_i128(&self, sink: &mut dyn BitSink, value: i128) -> Result<(), WriteError> {
        self.write_signed(sink, 128, value)
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    use crate::encoding::protocol::testing::*;

    const ENCODER: SimpleEncodingProtocol = SimpleEncodingProtocol::new();


    #[test]
    fn test_symmetry() {
        test_encoding_pair(
            &ENCODER,
            &SimpleDecodingProtocol::new(),
        );
    }

    #[test]
    fn test_u8() {
        test_u8_result(&ENCODER, 0, "0000 0000");
        test_u8_result(&ENCODER, 5, "1010 0000");
        test_u8_result(&ENCODER, 190, "0111 1101");
        test_u8_result(&ENCODER, 255, "1111 1111");
    }

    #[test]
    fn test_i8() {
        test_i8_result(&ENCODER, 0, "0000 0000");
        test_i8_result(&ENCODER, 13, "1011 0000");
        test_i8_result(&ENCODER, 127, "1111 1110");
        test_i8_result(&ENCODER, -128, "0000 0001");
        test_i8_result(&ENCODER, -1, "1111 1111");
    }
}
