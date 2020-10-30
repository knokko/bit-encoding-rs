use crate::*;

pub struct DigitEncodingProtocol {

    digit_size: u8,
    short_zero_and_one: bool
}

impl DigitEncodingProtocol {

    pub const fn new(digit_size: u8, short_zero_and_one: bool) -> Self {
        if digit_size < 2 || digit_size > 127 {
            // The commented line won't compile, at least for now
            //panic!("Invalid digit size: {}", digit_size);
            panic!("Invalid digit size");
        }
        Self {
            digit_size,
            short_zero_and_one
        }
    }

    pub const fn v1() -> Self {
        Self::new(4, true)
    }

    fn get_num_digit_values(&self) -> u128 {
        (1u128 << self.digit_size) - 1
    }

    fn write_digit_part(&self, sink: &mut dyn BitSink, mut value: u128, max_bits: u8) -> Result<(), WriteError> {
        let simple_encoder = SimpleEncodingProtocol::new(); 
        let num_digit_values = self.get_num_digit_values();
        println!("num digit values is {} and value is {}", num_digit_values, value);
        while value > 0 {
            let next_digit = value % num_digit_values;
            simple_encoder.write_unsigned(sink, self.digit_size as usize, next_digit)?;
            value /= num_digit_values;
        }

        let ones = vec![true; self.digit_size as usize];

        sink.write(&ones)
    }

    fn write_unsigned(&self, sink: &mut dyn BitSink, mut value: u128, max_bits: u8) -> Result<(), WriteError> {
        if self.short_zero_and_one {
            if value == 0 {
                return sink.write(&[true, false])
            } else if value == 1 {
                return sink.write(&[true, true])
            } else {
                sink.write(&[false])?;
                value -= 2;
            }
        }

        self.write_digit_part(sink, value, max_bits)
    }

    fn write_signed(&self, sink: &mut dyn BitSink, mut value: i128, max_bits: u8) -> Result<(), WriteError> {

        if self.short_zero_and_one {
            if value == 0 {
                return sink.write(&[true, false])
            } else if value == 1 {
                return sink.write(&[true, true])
            } else {
                sink.write(&[false])?;
                if value >= 0 {
                    value -= 2;
                }
            }
        }

        if value < 0 {
            sink.write(&[true])?;
            value += 1;
            value = -value;
        }

        self.write_digit_part(sink, value as u128, max_bits - 1)
    }
}

impl EncodingProtocol for DigitEncodingProtocol {

    fn write_u8(&self, sink: &mut dyn BitSink, value: u8) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, 8)
    }

    fn write_i8(&self, sink: &mut dyn BitSink, value: i8) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, 8)
    }

    fn write_u16(&self, sink: &mut dyn BitSink, value: u16) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, 16)
    }

    fn write_i16(&self, sink: &mut dyn BitSink, value: i16) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, 16)
    }

    fn write_u32(&self, sink: &mut dyn BitSink, value: u32) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, 32)
    }

    fn write_i32(&self, sink: &mut dyn BitSink, value: i32) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, 32)
    }

    fn write_u64(&self, sink: &mut dyn BitSink, value: u64) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, 64)
    }

    fn write_i64(&self, sink: &mut dyn BitSink, value: i64) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, 64)
    }

    fn write_u128(&self, sink: &mut dyn BitSink, value: u128) -> Result<(), WriteError> {
        self.write_unsigned(sink, value, 128)
    }

    fn write_i128(&self, sink: &mut dyn BitSink, value: i128) -> Result<(), WriteError> {
        self.write_signed(sink, value, 128)
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    use crate::encoding::protocol::testing::*;

    const ENCODER: DigitEncodingProtocol = DigitEncodingProtocol::v1();
    const DECODER: DigitDecodingProtocol = DigitDecodingProtocol::v1();

    #[test]
    fn test_symmetry() {
        test_encoding_pair(&ENCODER, &DECODER);
    }

    #[test]
    fn test_u8() {
        // Test the special cases 0 and 1
        test_u8_result(&ENCODER, &DECODER, 0, "1 0");
        test_u8_result(&ENCODER, &DECODER, 1, "1 1");

        // Test the small numbers
        test_u8_result(&ENCODER, &DECODER, 2, "0 1111");
        test_u8_result(&ENCODER, &DECODER, 3, "0 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 4, "0 0100 1111");
        test_u8_result(&ENCODER, &DECODER, 5, "0 1100 1111");
        test_u8_result(&ENCODER, &DECODER, 6, "0 0010 1111");
        test_u8_result(&ENCODER, &DECODER, 7, "0 1010 1111");

        // Now the numbers that need an extra digit
        test_u8_result(&ENCODER, &DECODER, 16, "0 0111 1111");
        test_u8_result(&ENCODER, &DECODER, 17, "0 0000 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 18, "0 1000 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 19, "0 0100 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 31, "0 0111 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 32, "0 0000 0100 1111");
        test_u8_result(&ENCODER, &DECODER, 226, "0 0111 0111 1111");

        // The biggest numbers
        test_u8_result(&ENCODER, &DECODER, 227, "0 0000 0000 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 228, "0 1000 0000 1000 1111");
        test_u8_result(&ENCODER, &DECODER, 255, "0 1011 1000 1000 1111");
    }

    #[test]
    fn test_i8() {
        //
    }
}