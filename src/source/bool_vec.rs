use crate::*;

pub struct BoolSliceBitSource<'a> {
    slice: &'a [bool],
}

impl<'a> BoolSliceBitSource<'a> {
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
