use crate::*;

/// An implementation of *BitSource* that simply reads *bool*s from a slice of
/// *bool*s. The *bool*s that occur first in the slice will be read first.
///
/// # Example
/// ```
/// use bit_encoding::*;
///
/// let bool_array = [true, false, true];
/// let mut source = BoolSliceBitSource::new(&bool_array);
///
/// // The initial value doesn't matter, but we have to give something
/// let mut array1 = [false];
/// source.read(&mut array1);
/// assert_eq!([true], array1);
///
/// // Initial values again do not matter, but have to be chosen
/// let mut array2 = [false, false];
/// source.read(&mut array2);
/// assert_eq!([false, true], array2);
/// ```
/// 
/// This implementation has excellent performance because minimal work is needed for
/// calls to *read*, but its memory usage is not so great because Rust uses an
/// entire byte to store 1 *bool*.
pub struct BoolSliceBitSource<'a> {
    slice: &'a [bool],
}

impl<'a> BoolSliceBitSource<'a> {
    /// Constructs a new instance of *BoolSliceBitSource* that will read its
    /// values from the given slice of *bool*s. See the documentation of this
    /// struct for an example.
    pub fn new(slice: &'a [bool]) -> Self {
        Self { slice }
    }
}

impl<'a> BitSource for BoolSliceBitSource<'a> {
    fn read(&mut self, dest: &mut [bool]) -> Result<(), ReadError> {
        if dest.len() > self.slice.len() {
            for index in 0..self.slice.len() {
                dest[index] = self.slice[index];
            }

            let old_length = self.slice.len();
            self.slice = &[];

            return Err(ReadError::ReachedEnd {
                read_bools: old_length,
            });
        }

        for index in 0..dest.len() {
            dest[index] = self.slice[index];
        }

        self.slice = &self.slice[dest.len()..self.slice.len()];

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn basic_tests() {
        super::test_helper::basic_tests(&|slice| {
            let mut as_vec = vec![false; slice.len()];
            as_vec.copy_from_slice(slice);

            // Not so neat, but it's only a unit test anyway
            let leaked_vec = Box::leak(Box::new(as_vec));
            let source = BoolSliceBitSource::new(leaked_vec);
            source
        });
    }

    #[test]
    fn random_tests() {
        super::test_helper::random_tests(&|slice| {
            let mut as_vec = vec![false; slice.len()];
            as_vec.copy_from_slice(slice);

            // Not so neat, but it's only a unit test anyway
            let leaked_vec = Box::leak(Box::new(as_vec));
            let source = BoolSliceBitSource::new(leaked_vec);
            source
        });
    }
}