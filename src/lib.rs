//! A crate for compactly encoding 'raw' data (bools, integers, strings...) into
//! sequences of bools or bytes. The primary goal is to eventually store these
//! compact sequences to disk or send them over the network, attempting to use as
//! little disk space or bandwidth as possible.

mod source;
mod sink;
mod decoding;

pub use source::*;
pub use sink::*;
pub use decoding::*;

/// The type to be used for encoding lengths of collections and strings. Note that
/// this type only indicates the size in memory and usually *not* the number of
/// bits used to store the length in the sequences because it will normally be
/// stored with a variable length.
pub type LengthType = u64;

/// Encodes a sequence of booleans (given as bool slice) as a sequence of bytes
/// (u8 Vec). The length of the byte vector will be the ceiling of bools.length / 8. 
/// 
/// To perform the encoding, the boolean sequence will be split into chunks
/// of 8 (except for the last chunk if the length of the boolean sequence is not
/// a multiple of 8). The first chunk will consist of the first 8 booleans in the
/// sequence and will be used to encode the first byte. The second chunk will
/// consist of the next 8 booleans in the sequence and will be used to encode the
/// second byte... 
/// 
/// To encode a chunk of bools into a u8, we will read the bool chunk as a binary
/// number where the first bool acts as least significant bit (1) and the last bool
/// acts as most significant bit (128). If the length of the boolean sequence is
/// not a multiple of 8, the last chunk of bools will have a shorter length than 8. 
/// To encode that chunk, all 'missing' bools will be treated as false.
/// 
/// # Examples
/// 
/// Simple case with an array of 8 bools
/// ```
/// use bit_encoding::bools_to_bytes;
/// let bools = [true, true, false, true, false, false, false, false];
/// assert_eq!(vec![11], bools_to_bytes(&bools));
/// ```
/// 
/// Incomplete case with an array of only 3 bools
/// ```
/// use bit_encoding::bools_to_bytes;
/// let bools = [true, false, true];
/// assert_eq!(vec![5], bools_to_bytes(&bools));
/// ```
pub fn bools_to_bytes(bools: &[bool]) -> Vec<u8> {

    let num_bytes = bools.len() / 8 + if bools.len() % 8 == 0 {0} else {1};
    let mut bytes = Vec::with_capacity(num_bytes);

    for bool_tuple in bools.chunks(8) {

        let mut current_byte = 0;
        for bit_index in 0..8 {
            if bool_tuple.len() > bit_index && bool_tuple[bit_index] {
                current_byte |= 1 << bit_index;
            }
        }

        bytes.push(current_byte);
    }

    bytes
}

#[cfg(test)]
mod tests {

    use crate::*;

    const F: bool = false;
    const T: bool = true;

    #[test]
    fn test_bools_to_bytes() {
        assert_eq!(Vec::<u8>::new(), bools_to_bytes(&[]));
        assert_eq!(vec![0], bools_to_bytes(&[false]));
        assert_eq!(vec![0], bools_to_bytes(&[false; 8]));
        assert_eq!(vec![0, 0], bools_to_bytes(&[false; 9]));

        assert_eq!(vec![39], bools_to_bytes(&[T, T, T, F, F, T]));
        assert_eq!(vec![39], bools_to_bytes(&[T, T, T, F, F, T, F, F]));

        assert_eq!(vec![255, 0], bools_to_bytes(&[T,T,T,T, T,T,T,T, F,F,F,F, F,F,F,F]));
        assert_eq!(vec![170, 85], bools_to_bytes(&[F,T,F,T, F,T,F,T, T,F,T,F, T,F,T,F]));
    }
}