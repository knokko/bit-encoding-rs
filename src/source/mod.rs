mod bool_slice;
mod u8_slice;
mod errors;

pub use bool_slice::*;
pub use u8_slice::*;
pub use errors::*;

/// A type from which bools can be read.
///
/// Typical implementations would read bools from a Vec of bools or binary
/// integers, but other implementations might read them from a file or socket
/// connection.
///
/// Every BitDecoder will have a BitSource from which it will read the data that
/// it decodes.
pub trait BitSource {
    /// Attempts to read bools from this source and put them into *dest*.
    ///
    /// The first bool read will be put in `dest[0]`, the second bool will be
    /// put into `dest[1]`, the third bool will be put into `dest[2]`...
    /// That will continue until *dest* has been filled completely.
    ///
    /// If enough bools could be read to fill *dest* completely, this method
    /// will return `Ok`. If all bools were read before *dest* was filled or
    /// if another error occurred while reading, a `ReadError` will be
    /// returned.
    fn read(&mut self, dest: &mut [bool]) -> Result<(), ReadError>;
}

#[cfg(test)]
pub(crate) mod test_helper {

    use crate::*;
    use rand::*;

    fn test_slice<S: BitSource>(slice: &[bool], source_generator: &impl Fn(&[bool]) -> S) {
        let mut source = source_generator(slice);
        
        // Test reading 1 by 1
        for index in 0 .. slice.len() {
            let mut target = [false];
            source.read(&mut target).unwrap();
            assert_eq!(slice[index], target[0]);
        }
        source.read(&mut [false]).unwrap_err();

        let mut source = source_generator(slice);

        // Test reading in groups of 3
        for index in 0 .. slice.len() / 3 {
            let mut target = [false; 3];
            source.read(&mut target).unwrap();
            assert_eq!(&slice[3 * index .. 3 * (index + 1)], &target);
        }
        source.read(&mut [false; 3]).unwrap_err();

        let mut source = source_generator(slice);

        // Test reading all bools at once
        let mut target = vec![false; slice.len()];
        source.read(&mut target).unwrap();
        assert_eq!(slice, target.as_slice());
        source.read(&mut [false]).unwrap_err();
    }

    pub fn basic_tests<S: BitSource>(source_generator: &impl Fn(&[bool]) -> S) {
        test_slice(&[false], source_generator);
        test_slice(&[true], source_generator);
        test_slice(&[true, true, false], source_generator);
        test_slice(&[false, true, false, true], source_generator);
        test_slice(&[true, true, false, false, true, false, false, true], source_generator);
        test_slice(&[true, true, false, false, true, false, false, true, true], source_generator);
    }

    pub fn random_tests<S: BitSource>(source_generator: &impl Fn(&[bool]) -> S) {
        let mut rng = rand::thread_rng();
        for _counter in 0 .. 100 {
            let random_length_part: u8 = rng.gen();
            let length = 1 + random_length_part as usize;
            let mut bools = vec![false; length];
            for index in 0 .. length {
                bools[index] = rng.gen();
            }
            test_slice(&bools, source_generator);
        }
    }
}