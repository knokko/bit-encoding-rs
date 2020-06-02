use crate::*;

/// One of the simplest implementations of BitSink.
/// This is a struct with only a Vec<bool> and it implements BitSink
/// by simply pushing each bool written to it onto its bool vector.
///
/// The write and finish method of this type will never return Err
/// because this implementation doesn't have any complex operations
/// that could fail in normal circumstances. However, it may panic if
/// for instance so many bools are pushed that the process runs out of
/// memory.
///
/// Note that this implementation is not very efficient in terms of memory usage
/// because Rust will always use 1 byte to store 1 bool rather than using 1 byte
/// to store 8 bools.
pub struct BoolVecBitSink {
    vec: Vec<bool>,
}

impl BoolVecBitSink {
    /// Constructs a new BoolVecBitSink backed by an empty Vec.
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Constructs a new BoolVecBitSink backed by an empty Vec with an
    /// initial capacity of *capacity*.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Gets a slice over the bools that were written to this BoolVecBitSink
    /// (in the right order).
    pub fn get_bits(&self) -> &[bool] {
        &self.vec
    }

    /// Encodes the bools that were written to this BoolVecBitSink as bytes,
    /// using the bools_to_bytes function of this crate.
    ///
    /// Note that this will create a new Vec<u8> each time this method is called,
    /// so don't call this repeatedly if you don't need to.
    pub fn to_bytes(&self) -> Vec<u8> {
        bools_to_bytes(&self.vec)
    }
}

impl BitSink for BoolVecBitSink {
    fn write(&mut self, bits: &[bool]) -> Result<(), WriteError> {
        self.vec.extend_from_slice(bits);

        // No errors should occur here
        Ok(())
    }

    fn finish(&mut self) -> Result<(), WriteError> {
        // Let's try to release some memory
        self.vec.shrink_to_fit();

        // No errors should occur here
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bool_vec_bit_sink_new() {
        let mut sink = BoolVecBitSink::new();
        assert_eq!(&[false; 0], sink.get_bits());
        sink.write(&[false, true, false, true, false, true])
            .unwrap();
        assert_eq!(&[false, true, false, true, false, true], sink.get_bits());
    }

    #[test]
    fn test_bool_vec_bit_sink_low_capacity() {
        let mut sink = BoolVecBitSink::with_capacity(2);
        assert_eq!(&[false; 0], sink.get_bits());
        sink.write(&[true, false, true]).unwrap();
        assert_eq!(&[true, false, true], sink.get_bits());
    }

    #[test]
    fn test_bool_vec_bit_sink_high_capacity() {
        let mut sink = BoolVecBitSink::with_capacity(2);
        sink.write(&[true]).unwrap();
        assert_eq!(&[true], sink.get_bits());
    }

    #[test]
    fn test_bool_vec_bit_sink_write() {
        let mut sink = BoolVecBitSink::new();
        sink.write(&[false, false]).unwrap();
        assert_eq!(&[false, false], sink.get_bits());
        sink.write(&[true, false, true]).unwrap();
        assert_eq!(&[false, false, true, false, true], sink.get_bits());
    }

    #[test]
    fn test_bool_vec_bit_sink_bytes() {
        let mut sink = BoolVecBitSink::new();
        assert_eq!(&[0u8; 0], &sink.to_bytes()[..]);
        sink.write(&[false, false, true, true, false, false, true, true, true])
            .unwrap();
        assert_eq!(&[204, 1], &sink.to_bytes()[..]);
    }
}
