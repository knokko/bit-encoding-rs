use crate::*;

pub struct DigitDecodingProtocol {
    digit_size: u8,
    short_zero_and_one: bool,

    max_num_digits: [u8; 10],
}

impl DigitDecodingProtocol {
    pub const fn new(digit_size: u8, short_zero_and_one: bool) -> Self {
        if digit_size < 2 || digit_size > 127 {
            // The commented line won't compile currently
            //panic!("Invalid digit size: {}", digit_size);
            panic!("Invalid digit size");
        }
        Self {
            digit_size,
            short_zero_and_one,
            max_num_digits: compute_relevant_num_digits(digit_size),
        }
    }

    pub const fn v1() -> Self {
        Self::new(3, true)
    }

    fn read_digit_part(
        &self,
        source: &mut dyn BitSource,
        max_num_digits: u8,
    ) -> Result<u128, DecodeError> {
        let simple_decoder = SimpleDecodingProtocol::new();
        let num_digit_values = get_num_digit_values(self.digit_size);

        let mut current_factor = 1;
        let mut current_result = 0;
        for current_digit in 1..=max_num_digits {
            let next_digit = simple_decoder.read_unsigned(source, self.digit_size as usize)?;

            // The maximum value indicates that the end of the number has been reached
            if next_digit == num_digit_values {
                break;
            }

            current_result += current_factor * next_digit;
            if current_digit < max_num_digits {
                current_factor *= num_digit_values;
            }
        }
        Ok(current_result)
    }

    fn read_unsigned(
        &self,
        source: &mut dyn BitSource,
        max_num_digits: u8,
    ) -> Result<u128, DecodeError> {
        if self.short_zero_and_one {
            let mut first_bit = [false];
            source.read(&mut first_bit)?;
            if first_bit[0] {
                let mut second_bit = [false];
                source.read(&mut second_bit)?;
                return match second_bit {
                    [false] => Ok(0),
                    [true] => Ok(1),
                };
            }
        }

        let result = self.read_digit_part(source, max_num_digits)?;
        match self.short_zero_and_one {
            false => Ok(result),
            true => Ok(result + 2),
        }
    }

    fn read_signed(
        &self,
        source: &mut dyn BitSource,
        max_num_digits: u8,
    ) -> Result<i128, DecodeError> {
        if self.short_zero_and_one {
            let mut first_bit = [false];
            source.read(&mut first_bit)?;
            if first_bit[0] {
                let mut second_bit = [false];
                source.read(&mut second_bit)?;
                return match second_bit {
                    [false] => Ok(0),
                    [true] => Ok(1),
                };
            }
        }

        let mut sign_bit = [false];
        source.read(&mut sign_bit)?;

        let unsigned_result = self.read_digit_part(source, max_num_digits)? as i128;
        match [sign_bit[0], self.short_zero_and_one] {
            [false, false] => Ok(unsigned_result),
            [false, true] => Ok(unsigned_result + 2),
            [true, _] => Ok(-unsigned_result - 1),
        }
    }
}

impl DecodingProtocol for DigitDecodingProtocol {
    fn read_u8(&self, source: &mut impl BitSource) -> Result<u8, DecodeError> {
        self.read_unsigned(source, self.max_num_digits[1])
            .map(|x| x as u8)
    }

    fn read_i8(&self, source: &mut impl BitSource) -> Result<i8, DecodeError> {
        self.read_signed(source, self.max_num_digits[0])
            .map(|x| x as i8)
    }

    fn read_u16(&self, source: &mut impl BitSource) -> Result<u16, DecodeError> {
        self.read_unsigned(source, self.max_num_digits[3])
            .map(|x| x as u16)
    }

    fn read_i16(&self, source: &mut impl BitSource) -> Result<i16, DecodeError> {
        self.read_signed(source, self.max_num_digits[2])
            .map(|x| x as i16)
    }

    fn read_u32(&self, source: &mut impl BitSource) -> Result<u32, DecodeError> {
        self.read_unsigned(source, self.max_num_digits[5])
            .map(|x| x as u32)
    }

    fn read_i32(&self, source: &mut impl BitSource) -> Result<i32, DecodeError> {
        self.read_signed(source, self.max_num_digits[4])
            .map(|x| x as i32)
    }

    fn read_u64(&self, source: &mut impl BitSource) -> Result<u64, DecodeError> {
        self.read_unsigned(source, self.max_num_digits[7])
            .map(|x| x as u64)
    }

    fn read_i64(&self, source: &mut impl BitSource) -> Result<i64, DecodeError> {
        self.read_signed(source, self.max_num_digits[6])
            .map(|x| x as i64)
    }

    fn read_u128(&self, source: &mut impl BitSource) -> Result<u128, DecodeError> {
        self.read_unsigned(source, self.max_num_digits[9])
    }

    fn read_i128(&self, source: &mut impl BitSource) -> Result<i128, DecodeError> {
        self.read_signed(source, self.max_num_digits[8])
    }
}
