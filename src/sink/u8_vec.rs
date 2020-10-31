use crate::*;

pub struct U8VecBitSink {
    bytes: Vec<u8>,
    bit_index: u8
}

impl U8VecBitSink {

    pub fn new() -> Self {
        Self { bytes: Vec::new(), bit_index: 0 }
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self { bytes: Vec::with_capacity(initial_capacity), bit_index: 0 }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn get_bits(&self) -> Vec<bool> {
        // TODO Remove the redundant bits at the end
        bytes_to_bools(self.get_bytes())
    }
}

impl BitSink for U8VecBitSink {

    fn write(&mut self, bits: &[bool]) -> Result<(), WriteError> {
        // This may not be exact, but should be very accurate
        self.bytes.reserve(bits.len() / 8);

        // If we ended with a partial byte previously, we should continue with it
        let mut current_byte = match self.bit_index == 0 {
            true => 0, false => self.bytes.pop().unwrap()
        };

        // Add all bits...
        for bit in bits {
            if *bit {
                current_byte |= 1 << self.bit_index;
            }
            self.bit_index += 1;
            if self.bit_index == 8 {
                self.bytes.push(current_byte);
                self.bit_index = 0;
                current_byte = 0;
            }
        }

        // If we end with a partial byte, we should remember it
        if self.bit_index != 0 {
            self.bytes.push(current_byte);
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<(), WriteError> {
        self.bytes.shrink_to_fit();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn digit_test() {
        let encoder = DigitEncodingProtocol::v1();
        let mut sink = U8VecBitSink::new();

        // Encode some numbers
        for counter in 0..100 {
            encoder.write_u8(&mut sink, counter).unwrap();
        }

        let as_bools = bytes_to_bools(&sink.bytes);
        let mut source = BoolSliceBitSource::new(&as_bools);
        let decoder = DigitDecodingProtocol::v1();

        // Decode the same numbers
        for counter in 0..100 {
            assert_eq!(counter, decoder.read_u8(&mut source).unwrap());
        }
    }
}