use crate::*;

pub struct U32VecBitSink {
    ints: Vec<u32>,
    bit_index: u8
}

impl U32VecBitSink {

    pub fn new() -> Self {
        Self { ints: Vec::new(), bit_index: 0 }
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self { ints: Vec::with_capacity(initial_capacity), bit_index: 0 }
    }

    pub fn get_ints(&self) -> &Vec<u32> {
        &self.ints
    }

    pub fn get_bits(&self) -> Vec<bool> {
        let mut bools = Vec::with_capacity(self.get_num_bools() as usize);
        let first_bound = match self.bit_index == 0 {
            true => self.ints.len(), false => self.ints.len() - 1
        };
        for index in 0..first_bound {
            let current_int = self.ints[index];
            for bit_index in 0..32 {
                bools.push(current_int & (1 << bit_index) != 0);
            }
        }

        if self.ints.len() > 0 {
            let last_int = self.ints[self.ints.len() - 1];
            for bit_index in 0..self.bit_index {
                bools.push(last_int & (1 << bit_index) != 0);
            }
        }
        bools
    }
}

impl BitSink for U32VecBitSink {

    fn write(&mut self, bits: &[bool]) -> Result<(), WriteError> {
        // This may not be exact, but should be very accurate
        self.ints.reserve(bits.len() / 32);

        // If we ended with a partial byte previously, we should continue with it
        let mut current_int = match self.bit_index == 0 {
            true => 0, false => self.ints.pop().unwrap()
        };

        // Add all bits...
        for bit in bits {
            if *bit {
                current_int |= 1 << self.bit_index;
            }
            self.bit_index += 1;
            if self.bit_index == 32 {
                self.ints.push(current_int);
                self.bit_index = 0;
                current_int = 0;
            }
        }

        // If we end with a partial int, we should remember it
        if self.bit_index != 0 {
            self.ints.push(current_int);
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<(), WriteError> {
        self.ints.shrink_to_fit();
        Ok(())
    }

    fn get_num_bools(&self) -> u64 {
        if self.bit_index == 0 {
            self.ints.len() as u64 * 32
        } else {
            (self.ints.len() as u64 - 1) * 32 + self.bit_index as u64
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn digit_test() {
        let encoder = DigitIntEncodingProtocol::v1();
        let mut sink = U32VecBitSink::new();

        // Encode some numbers
        for counter in 0..100 {
            encoder.write_u8(&mut sink, counter).unwrap();
        }

        let as_bools = sink.get_bits();
        let mut source = BoolSliceBitSource::new(&as_bools);
        let decoder = DigitIntDecodingProtocol::v1();

        // Decode the same numbers
        for counter in 0..100 {
            assert_eq!(counter, decoder.read_u8(&mut source).unwrap());
        }
    }
}