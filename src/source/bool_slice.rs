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
