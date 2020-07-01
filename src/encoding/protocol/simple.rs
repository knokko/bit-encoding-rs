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
    const DECODER: SimpleDecodingProtocol = SimpleDecodingProtocol::new();

    #[test]
    fn test_symmetry() {
        test_encoding_pair(
            &ENCODER,
            &SimpleDecodingProtocol::new(),
        );
    }

    #[test]
    fn test_u8() {
        test_u8_result(&ENCODER, &DECODER, 0, "0000 0000");
        test_u8_result(&ENCODER, &DECODER,5, "1010 0000");
        test_u8_result(&ENCODER, &DECODER,190, "0111 1101");
        test_u8_result(&ENCODER, &DECODER,255, "1111 1111");
    }

    #[test]
    fn test_i8() {
        test_i8_result(&ENCODER, &DECODER,0, "0000 0000");
        test_i8_result(&ENCODER, &DECODER,13, "1011 0000");
        test_i8_result(&ENCODER, &DECODER,127, "1111 1110");
        test_i8_result(&ENCODER, &DECODER,-128, "0000 0001");
        test_i8_result(&ENCODER, &DECODER,-1, "1111 1111");
        test_i8_result(&ENCODER, &DECODER,-75, "1010 1101")
    }

    #[test]
    fn test_u16() {
        test_u16_result(&ENCODER, &DECODER,0, "0000 0000 0000 0000");
        test_u16_result(&ENCODER, &DECODER,9, "1001 0000 0000 0000");
        test_u16_result(&ENCODER, &DECODER,65_535, "1111 1111 1111 1111");
    }

    #[test]
    fn test_i16() {
        test_i16_result(&ENCODER, &DECODER,0, "0000 0000 0000 0000");
        test_i16_result(&ENCODER, &DECODER,5, "1010 0000 0000 0000");
        test_i16_result(&ENCODER, &DECODER,32_767, "1111 1111 1111 1110");
        test_i16_result(&ENCODER, &DECODER,-32_768, "0000 0000 0000 0001");
        test_i16_result(&ENCODER, &DECODER,-3, "1011 1111 1111 1111");
        test_i16_result(&ENCODER, &DECODER,-1, "1111 1111 1111 1111");
    }

    #[test]
    fn test_u32() {
        test_u32_result(&ENCODER, &DECODER,0, "0000 0000 0000 0000  0000 0000 0000 0000");
        test_u32_result(&ENCODER, &DECODER,11, "1101 0000 0000 0000  0000 0000 0000 0000");
        test_u32_result(&ENCODER, &DECODER,4_294_967_295, "1111 1111 1111 1111  1111 1111 1111 1111")
    }

    #[test]
    fn test_i32() {
        test_i32_result(&ENCODER, &DECODER,0, "0000 0000 0000 0000  0000 0000 0000 0000");
        test_i32_result(&ENCODER, &DECODER,6, "0110 0000 0000 0000  0000 0000 0000 0000");
        test_i32_result(&ENCODER, &DECODER,2_147_483_647, "1111 1111 1111 1111  1111 1111 1111 1110");
        test_i32_result(&ENCODER, &DECODER,-2_147_483_648, "0000 0000 0000 0000  0000 0000 0000 0001");
        test_i32_result(&ENCODER, &DECODER,-4, "0011 1111 1111 1111  1111 1111 1111 1111");
        test_i32_result(&ENCODER, &DECODER,-1, "1111 1111 1111 1111  1111 1111 1111 1111");
    }

    // TODO Perhaps unit tests for iu64 and iu128 as well, but these strings are long...
}
