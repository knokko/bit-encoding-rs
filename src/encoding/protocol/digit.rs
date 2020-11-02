use crate::*;

pub(crate) const fn get_num_digit_values(digit_size: u8) -> u128 {
    (1u128 << digit_size) - 1
}

const fn compute_num_digits(digit_size: u8, max_bits: u8) -> u8 {
    let mut value = match max_bits == 128 {
        true => u128::max_value(),
        false => 1 << max_bits,
    };
    let num_digit_values = get_num_digit_values(digit_size);
    let mut counter = 0;
    while value > 0 {
        value /= num_digit_values;
        counter += 1;
    }
    counter
}

const RELEVANT_NUM_DIGITS: [u8; 10] = [7, 8, 15, 16, 31, 32, 63, 64, 127, 128];

pub(crate) const fn compute_relevant_num_digits(digit_size: u8) -> [u8; 10] {
    let mut result = [0; RELEVANT_NUM_DIGITS.len()];

    // I would rather use a for loop, but that is forbidden in const functions
    let mut index = 0;
    while index < result.len() {
        result[index] = compute_num_digits(digit_size, RELEVANT_NUM_DIGITS[index]);
        index += 1;
    }
    result
}

/// An *IntEncodingProtocol* based on writing integers digit by digit that uses a
/// special digit value as terminator digit. This encoding protocol is suitable for
/// encoding values that are often small.
///
/// It is inspired by the (decimal) notation of integers. Take for instance the
/// integer 537. It needs only 4 digits (including the dot as 'terminator'), but
/// this notation can just as well be used to store numbers of many more digits.
/// Even if every digit would take an entire byte, this would already be equally
/// good as the simple 32-bit representation of 537.
///
/// But we don't need an entire byte to store 1 decimal digit: there are only 10
/// possible values plus 1 terminator, so only 11 from the 256 possibilities would
/// be used, so using half a byte per digit would work as well, and would be
/// twice as short. But even then, only 11 out of the 16 possible half-byte values
/// are used. That is a huge improvement, but still not optimal.
///
/// To avoid wasting possible digit values, the number of possible digits plus 1
/// (for the terminator) should be a power of 2. So if we would want to use half
/// a byte (4 bits) per digit, we should have 15 possible digit values instead of
/// only 10. Then we can often write the same number with lesser digits. For
/// instance, the 10 digit system needs 4 digits to encode 3000, while the 15
/// digit system only needs 3 digits to do so.
///
/// The 15 digit system explained above is quite good, but the idea mentioned
/// above can be generalized further. For instance, we could also use a 7 digit
/// system (3 bits per digit) or a 31 digit system (5 bits per digit). Which of
/// these systems works best, depends on the number to be encoded. This protocol
/// can support all such systems because its *digit_size* (the number of bits per
/// digit is configurable).
///
/// To come back at the example, I will show how this protocol would encode the
/// number 537 with a *digit_size* of 4 (so 15 digits and 1 terminator):
/// 537 can be written as
///
/// 12 * 1 + 5 * 15 + 2 * 15 * 15, which would be encoded as
///
/// 0011 1010 0100 1111. The binary encoding used for the digits is the reverse of
/// what you would probably expect (so 1 would be encoded as 1000 rather than 0001).
/// So 12 is encoded as 0011, 5 is encoded as 1010, and 2 is encoded as 0100.
/// And finally, the terminating digit is 15, which is encoded as 1111. So in this
/// example, we needed only 16 bits = 2 bytes to encode the number 537. Using this
/// encoding, we would even be able to write 15 + 15 * 15 + 15 * 15 * 15 = 3615
/// using only 2 bytes.
///
/// So far, I explained the general concept of this encoding, but there is 1 small
/// thing more to tell. In real applications, it is usually the case that a big
/// part of the features is only used by a small part of the users. When storing
/// data for these users, the integers 0 and 1 are very frequently used to encode
/// that these features are unused (for instance, when encoding a list by first
/// encoding its size, this size would usually be 0). If this is indeed the case,
/// it would be beneficial to give 0 and 1 a very short encoding (even shorter than
/// the 1 and 2 digits you would normally use).
///
/// This is implemented by letting the first bit of the number be a discriminator
/// bit: if its 0, the encoded number is at least 2, and if its 1, the encoded
/// number is 0 or 1. In the latter case, the second bit is then used to tell which
/// of the 2 values it is, which makes it possible to encode 0 and 1 using only 2
/// bits. In the former case, the 'regular' digit encoding of the number - 2 will
/// be written after the first bit. This means that the encoding size of all numbers
/// greater than 1 is increased by 1 bit. Because this is quite a waste if 0 and 1
/// aren't used frequently, this behavior is configurable: it will only be used if
/// *short_zero_and_one* is true.
pub struct DigitIntEncodingProtocol {
    digit_size: u8,
    short_zero_and_one: bool,

    max_num_digits: [u8; 10],
}

impl DigitIntEncodingProtocol {
    /// Constructs a new instance of *DigitIntEncodingProtocol* using the given
    /// *digit_size* and *short_zero_and_one* configuration. As explained in the
    /// documentation of this struct, *digit_size* is the number of bits used to
    /// encode a single digit, and *short_zero_and_one* should be true if you
    /// want the encoder to use extra short encodings for 0 and 1 at the expense
    /// of a slightly longer encoding of the rest of the numbers.
    pub const fn new(digit_size: u8, short_zero_and_one: bool) -> Self {
        if digit_size < 2 || digit_size > 127 {
            // The commented line won't compile, at least for now
            //panic!("Invalid digit size: {}", digit_size);
            panic!("Invalid digit size");
        }
        DigitIntEncodingProtocol {
            digit_size,
            short_zero_and_one,
            max_num_digits: compute_relevant_num_digits(digit_size),
        }
    }

    /// Constructs a new instance of *DigitIntEncodingProtocol* that uses the first
    /// configuration (constructor parameters) that I found to perform well. The
    /// *v1* function of *DigitIntDecodingProtocol* returns a corresponding
    /// decoder for this encoder.
    ///
    /// This configuration uses *short_zero_and_one* and has a *digit_size* of 3.
    pub const fn v1() -> Self {
        Self::new(3, true)
    }

    fn get_num_digit_values(&self) -> u128 {
        (1u128 << self.digit_size) - 1
    }

    fn write_digit_part(
        &self,
        sink: &mut impl BitSink,
        mut value: u128,
        max_num_digits: u8,
    ) -> Result<(), WriteError> {
        let simple_encoder = SimpleIntEncodingProtocol::new();
        let num_digit_values = self.get_num_digit_values();
        let mut num_digits = 0;
        while value > 0 {
            let next_digit = value % num_digit_values;
            simple_encoder.write_unsigned(sink, self.digit_size as usize, next_digit)?;
            value /= num_digit_values;
            num_digits += 1;
        }

        if num_digits < max_num_digits {
            let ones = vec![true; self.digit_size as usize];
            sink.write(&ones)
        } else {
            Ok(())
        }
    }

    fn write_unsigned(
        &self,
        sink: &mut impl BitSink,
        mut value: u128,
        max_num_digits: u8,
    ) -> Result<(), WriteError> {
        if self.short_zero_and_one {
            if value == 0 {
                return sink.write(&[true, false]);
            } else if value == 1 {
                return sink.write(&[true, true]);
            } else {
                sink.write(&[false])?;
                value -= 2;
            }
        }

        self.write_digit_part(sink, value, max_num_digits)
    }

    fn write_signed(
        &self,
        sink: &mut impl BitSink,
        mut value: i128,
        max_num_digits: u8,
    ) -> Result<(), WriteError> {
        if self.short_zero_and_one {
            if value == 0 {
                return sink.write(&[true, false]);
            } else if value == 1 {
                return sink.write(&[true, true]);
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
        } else {
            sink.write(&[false])?;
        }

        self.write_digit_part(sink, value as u128, max_num_digits)
    }
}

impl IntEncodingProtocol for DigitIntEncodingProtocol {
    fn write_u8(&self, sink: &mut impl BitSink, value: u8) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, self.max_num_digits[1])
    }

    fn write_i8(&self, sink: &mut impl BitSink, value: i8) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, self.max_num_digits[0])
    }

    fn write_u16(&self, sink: &mut impl BitSink, value: u16) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, self.max_num_digits[3])
    }

    fn write_i16(&self, sink: &mut impl BitSink, value: i16) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, self.max_num_digits[2])
    }

    fn write_u32(&self, sink: &mut impl BitSink, value: u32) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, self.max_num_digits[5])
    }

    fn write_i32(&self, sink: &mut impl BitSink, value: i32) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, self.max_num_digits[4])
    }

    fn write_u64(&self, sink: &mut impl BitSink, value: u64) -> Result<(), WriteError> {
        self.write_unsigned(sink, value as u128, self.max_num_digits[7])
    }

    fn write_i64(&self, sink: &mut impl BitSink, value: i64) -> Result<(), WriteError> {
        self.write_signed(sink, value as i128, self.max_num_digits[6])
    }

    fn write_u128(&self, sink: &mut impl BitSink, value: u128) -> Result<(), WriteError> {
        self.write_unsigned(sink, value, self.max_num_digits[9])
    }

    fn write_i128(&self, sink: &mut impl BitSink, value: i128) -> Result<(), WriteError> {
        self.write_signed(sink, value, self.max_num_digits[8])
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    use crate::encoding::protocol::testing::*;

    const ENCODER: DigitIntEncodingProtocol = DigitIntEncodingProtocol::new(4, true);
    const DECODER: DigitIntDecodingProtocol = DigitIntDecodingProtocol::new(4, true);

    #[test]
    fn test_symmetry() {
        test_encoding_pair(&ENCODER, &DECODER);
        // Test a few other combinations as well
        test_encoding_pair(
            &DigitIntEncodingProtocol::new(2, false),
            &DigitIntDecodingProtocol::new(2, false),
        );
        test_encoding_pair(
            &DigitIntEncodingProtocol::new(3, true),
            &DigitIntDecodingProtocol::new(3, true),
        );
        test_encoding_pair(
            &DigitIntEncodingProtocol::new(7, false),
            &DigitIntDecodingProtocol::new(7, false),
        );
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
        test_u8_result(&ENCODER, &DECODER, 227, "0 0000 0000 1000");
        test_u8_result(&ENCODER, &DECODER, 228, "0 1000 0000 1000");
        test_u8_result(&ENCODER, &DECODER, 255, "0 1011 1000 1000");
    }

    #[test]
    fn test_i8() {
        // The first tests are ripped from test_u8
        test_i8_result(&ENCODER, &DECODER, 0, "1 0");
        test_i8_result(&ENCODER, &DECODER, 1, "1 1");

        // Test the small numbers
        test_i8_result(&ENCODER, &DECODER, 2, "00 1111");
        test_i8_result(&ENCODER, &DECODER, 3, "00 1000 1111");
        test_i8_result(&ENCODER, &DECODER, 4, "00 0100 1111");
        test_i8_result(&ENCODER, &DECODER, 5, "00 1100 1111");
        test_i8_result(&ENCODER, &DECODER, 6, "00 0010 1111");
        test_i8_result(&ENCODER, &DECODER, 7, "00 1010 1111");

        // Now the numbers that need an extra digit
        test_i8_result(&ENCODER, &DECODER, 16, "00 0111 1111");
        test_i8_result(&ENCODER, &DECODER, 17, "00 0000 1000");
        test_i8_result(&ENCODER, &DECODER, 18, "00 1000 1000");
        test_i8_result(&ENCODER, &DECODER, 19, "00 0100 1000");
        test_i8_result(&ENCODER, &DECODER, 31, "00 0111 1000");
        test_i8_result(&ENCODER, &DECODER, 32, "00 0000 0100");

        // Lets finally test some negative numbers as well
        test_i8_result(&ENCODER, &DECODER, -1, "01 1111");
        test_i8_result(&ENCODER, &DECODER, -2, "01 1000 1111");
        test_i8_result(&ENCODER, &DECODER, -3, "01 0100 1111");
        test_i8_result(&ENCODER, &DECODER, -4, "01 1100 1111");
        test_i8_result(&ENCODER, &DECODER, -5, "01 0010 1111");
        test_i8_result(&ENCODER, &DECODER, -6, "01 1010 1111");

        // Now the numbers that need an extra digit
        test_i8_result(&ENCODER, &DECODER, -15, "01 0111 1111");
        test_i8_result(&ENCODER, &DECODER, -16, "01 0000 1000");
        test_i8_result(&ENCODER, &DECODER, -17, "01 1000 1000");
        test_i8_result(&ENCODER, &DECODER, -18, "01 0100 1000");
        test_i8_result(&ENCODER, &DECODER, -30, "01 0111 1000");
        test_i8_result(&ENCODER, &DECODER, -31, "01 0000 0100");

        // The biggest values for i8
        test_i8_result(&ENCODER, &DECODER, -128, "01 1110 0001");
        test_i8_result(&ENCODER, &DECODER, 127, "00 1010 0001");
    }

    // The 16-bit version of the unit tests for u8 and i8
    #[test]
    fn test_u16() {
        // Test the special cases 0 and 1
        test_u16_result(&ENCODER, &DECODER, 0, "1 0");
        test_u16_result(&ENCODER, &DECODER, 1, "1 1");

        // Test the small numbers
        test_u16_result(&ENCODER, &DECODER, 2, "0 1111");
        test_u16_result(&ENCODER, &DECODER, 3, "0 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 4, "0 0100 1111");
        test_u16_result(&ENCODER, &DECODER, 5, "0 1100 1111");
        test_u16_result(&ENCODER, &DECODER, 6, "0 0010 1111");
        test_u16_result(&ENCODER, &DECODER, 7, "0 1010 1111");

        // Now the numbers that need an extra digit
        test_u16_result(&ENCODER, &DECODER, 16, "0 0111 1111");
        test_u16_result(&ENCODER, &DECODER, 17, "0 0000 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 18, "0 1000 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 19, "0 0100 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 31, "0 0111 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 32, "0 0000 0100 1111");
        test_u16_result(&ENCODER, &DECODER, 226, "0 0111 0111 1111");

        // The biggest numbers
        test_u16_result(&ENCODER, &DECODER, 227, "0 0000 0000 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 228, "0 1000 0000 1000 1111");
        test_u16_result(&ENCODER, &DECODER, 255, "0 1011 1000 1000 1111");
    }

    #[test]
    fn test_i16() {
        // The first tests are ripped from test_u8
        test_i16_result(&ENCODER, &DECODER, 0, "1 0");
        test_i16_result(&ENCODER, &DECODER, 1, "1 1");

        // Test the small numbers
        test_i16_result(&ENCODER, &DECODER, 2, "00 1111");
        test_i16_result(&ENCODER, &DECODER, 3, "00 1000 1111");
        test_i16_result(&ENCODER, &DECODER, 4, "00 0100 1111");
        test_i16_result(&ENCODER, &DECODER, 5, "00 1100 1111");
        test_i16_result(&ENCODER, &DECODER, 6, "00 0010 1111");
        test_i16_result(&ENCODER, &DECODER, 7, "00 1010 1111");

        // Now the numbers that need an extra digit
        test_i16_result(&ENCODER, &DECODER, 16, "00 0111 1111");
        test_i16_result(&ENCODER, &DECODER, 17, "00 0000 1000 1111");
        test_i16_result(&ENCODER, &DECODER, 18, "00 1000 1000 1111");
        test_i16_result(&ENCODER, &DECODER, 19, "00 0100 1000 1111");
        test_i16_result(&ENCODER, &DECODER, 31, "00 0111 1000 1111");
        test_i16_result(&ENCODER, &DECODER, 32, "00 0000 0100 1111");

        // Lets finally test some negative numbers as well
        test_i16_result(&ENCODER, &DECODER, -1, "01 1111");
        test_i16_result(&ENCODER, &DECODER, -2, "01 1000 1111");
        test_i16_result(&ENCODER, &DECODER, -3, "01 0100 1111");
        test_i16_result(&ENCODER, &DECODER, -4, "01 1100 1111");
        test_i16_result(&ENCODER, &DECODER, -5, "01 0010 1111");
        test_i16_result(&ENCODER, &DECODER, -6, "01 1010 1111");

        // Now the numbers that need an extra digit
        test_i16_result(&ENCODER, &DECODER, -15, "01 0111 1111");
        test_i16_result(&ENCODER, &DECODER, -16, "01 0000 1000 1111");
        test_i16_result(&ENCODER, &DECODER, -17, "01 1000 1000 1111");
        test_i16_result(&ENCODER, &DECODER, -18, "01 0100 1000 1111");
        test_i16_result(&ENCODER, &DECODER, -30, "01 0111 1000 1111");
        test_i16_result(&ENCODER, &DECODER, -31, "01 0000 0100 1111");

        // The biggest values for i8
        test_i16_result(&ENCODER, &DECODER, -128, "01 1110 0001 1111");
        test_i16_result(&ENCODER, &DECODER, 127, "00 1010 0001 1111");
    }

    #[test]
    fn test_zero_and_one() {
        let regular_encoder = DigitIntEncodingProtocol::new(3, false);
        let regular_decoder = DigitIntDecodingProtocol::new(3, false);
        let special_encoder = DigitIntEncodingProtocol::new(3, true);
        let special_decoder = DigitIntDecodingProtocol::new(3, true);

        test_u8_result(&regular_encoder, &regular_decoder, 0, "111");
        test_u8_result(&regular_encoder, &regular_decoder, 1, "100 111");
        test_u8_result(&special_encoder, &special_decoder, 0, "1 0");
        test_u8_result(&special_encoder, &special_decoder, 1, "1 1");
    }
}
