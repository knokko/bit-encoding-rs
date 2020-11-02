use crate::*;

/// An implementation of *BitSink* that uses a *Vec\<u8\>* to store the bools written
/// into it. Every *u8* will represent 8 *bool*s (except for the last one if the
/// number of bools is not a multiple of 8).
///
/// Since every *u8* takes 1 byte in memory and this implementation stores 1 *u8*
/// per 8 *bool*s, this implementation is very efficient in terms of memory usage.
/// However, packing all these bools into bytes makes this implementation a bit
/// slow. For instance, it looks like the *write* method of this implementation
/// is approximately twice as slow as that of *BoolVecBitSink* (but note that the
/// time to write bools into the *BitSink* is rarely the performance bottleneck
/// of the encoding process).
pub struct U8VecBitSink {
    bytes: Vec<u8>,
    bit_index: u8,
}

impl U8VecBitSink {
    /// Constructs a new and empty instance of *U8VecBitSink*
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            bit_index: 0,
        }
    }

    /// Constructs a new and empty instance of *U8VecBitSink* with an initial
    /// capacity of *initial_capacity* *u8*s.
    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(initial_capacity),
            bit_index: 0,
        }
    }

    /// Gets a reference to the *Vec\<u8\>* of this sink
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    /// Creates a *Vec* of bools that shows exactly which bools were written into
    /// this sink in which order: The first bool of the *Vec* will be the first
    /// bool that was written into this sink.
    ///
    /// # Example
    /// ```
    /// use bit_encoding::*;
    ///
    /// let mut sink = U8VecBitSink::new();
    /// sink.write(&[true, false, true]);
    ///
    /// assert_eq!(vec![true, false, true], sink.get_bools());
    /// ```
    pub fn get_bools(&self) -> Vec<bool> {
        let mut as_bools = bytes_to_bools(self.get_bytes());
        if self.bit_index > 0 {
            for _counter in 0..8 - self.bit_index {
                as_bools.pop();
            }
            as_bools.shrink_to_fit();
        }

        as_bools
    }
}

impl BitSink for U8VecBitSink {
    fn write(&mut self, bits: &[bool]) -> Result<(), WriteError> {
        // This may not be exact, but should be very accurate
        self.bytes.reserve(bits.len() / 8);

        // If we ended with a partial byte previously, we should continue with it
        let mut current_byte = match self.bit_index == 0 {
            true => 0,
            false => self.bytes.pop().unwrap(),
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

    fn get_num_bools(&self) -> u64 {
        if self.bit_index == 0 {
            self.bytes.len() as u64 * 8
        } else {
            (self.bytes.len() as u64 - 1) * 8 + self.bit_index as u64
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn digit_test() {
        let encoder = DigitIntEncodingProtocol::v1();
        let mut sink = U8VecBitSink::new();

        // Encode some numbers
        for counter in 0..100 {
            encoder.write_u8(&mut sink, counter).unwrap();
        }

        let as_bools = bytes_to_bools(&sink.bytes);
        let mut source = BoolSliceBitSource::new(&as_bools);
        let decoder = DigitIntDecodingProtocol::v1();

        // Decode the same numbers
        for counter in 0..100 {
            assert_eq!(counter, decoder.read_u8(&mut source).unwrap());
        }
    }
}
