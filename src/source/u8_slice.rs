use crate::*;

/// An implementation of *BitSource* that reads its *bool*s from a slice of
/// *u8*s by using bitwise operators. The least significant bit of the first
/// *u8* will be read first, then the second-least significant bit of the
/// first *u8*...
/// 
/// If you use a slice from the result of *bools_to_bytes(x)* to read from,
/// this implementation will read *bools* in the same order as they appeared in
/// *x*.
/// 
/// # Example
/// ```
/// use bit_encoding::*;
/// 
/// let some_bools = [true, true, false, false, true];
/// 
/// // We have only 5 bools, which will be converted to 1 u8.
/// // However, 1 u8 can hold data for 8 bools, which is misleading since we only have 5
/// let num_padding_bools = 8 - some_bools.len();
/// let as_bytes = bools_to_bytes(&some_bools);
/// let mut source = U8SliceBitSource::new(&as_bytes, num_padding_bools as u8);
/// 
/// let mut dest = [false; 5];
/// source.read(&mut dest).expect("Reading 5 bools should be possible");
/// assert_eq!(some_bools, dest);
/// source.read(&mut [false]).expect_err("No bools left to read");
/// ```
/// 
/// This implementation will be slower than BoolSliceBitSource because it has to
/// convert a *u8* in its slice to *bool*s each time it reads. However, it should
/// consume 8 times as less memory since it stores 8 *bool*s in 1 *u8*, and Rust
/// uses 8 bits to store 1 bool.
pub struct U8SliceBitSource<'a> {
    slice: &'a [u8],
    bit_offset: u8,
    num_padding_bits: u8
}

impl<'a> U8SliceBitSource<'a> {
    /// Constructs a new *U8SliceBitSource* that will read its data from *slice*
    /// and has the given number of padding bits. The last *num_padding_bits* bits
    /// of *slice* will be ignored by the *read* method: It will return
    /// *ReadError::ReachedEnd* when an attempt is made to read these bits. 
    /// 
    /// This property is useful because the number of bools that should be written
    /// is usually not a multiple of 8, but a slice of *u8*s can only store a
    /// multiple of 8 number of bools, so there are usually a couple of fake values
    /// at the end, that were never written, and thus should not be read.
    pub fn new(slice: &'a [u8], num_padding_bits: u8) -> Self {
        Self { slice, bit_offset: 0, num_padding_bits }
    }
}

impl<'a> BitSource for U8SliceBitSource<'a> {
    fn read(&mut self, dest: &mut [bool]) -> Result<(), ReadError> {
        let remaining_bits = self.slice.len() * 8 - self.bit_offset as usize - self.num_padding_bits as usize;
        let num_bits_to_write = usize::min(dest.len(), remaining_bits);
        for dest_index in 0..num_bits_to_write {
            let own_byte = self.slice[0];
            let own_bit = own_byte & (1 << self.bit_offset) != 0;
            dest[dest_index] = own_bit;
            if self.bit_offset == 7 {
                self.bit_offset = 0;
                self.slice = &self.slice[1..self.slice.len()];
            } else {
                self.bit_offset += 1;
            }
        }

        if num_bits_to_write < dest.len() {
            Err(ReadError::ReachedEnd { read_bools: num_bits_to_write })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn basic_tests() {
        super::test_helper::basic_tests(&|slice| {
            let bytes = bools_to_bytes(slice);

            // Not such a nice solution, but this is just for unit testing anyway
            let byte_slice = Box::leak(Box::new(bytes));
            let source = U8SliceBitSource::new(byte_slice, (8 * byte_slice.len() - slice.len()) as u8);
            source
        });
    }

    #[test]
    fn random_tests() {
        super::test_helper::random_tests(&|slice| {
            let bytes = bools_to_bytes(slice);

            // Not such a nice solution, but this is just for unit testing anyway
            let byte_slice = Box::leak(Box::new(bytes));
            let source = U8SliceBitSource::new(byte_slice, (8 * byte_slice.len() - slice.len()) as u8);
            source
        });
    }
}
