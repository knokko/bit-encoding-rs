use crate::*;

#[derive(Debug)]
pub enum DecodeError {

    BigVecLength(LengthExceeded),

    BigStringLength(LengthExceeded),

    VecLengthOverflow{ length: LengthType },

    StringLengthOverflow{ length: LengthType },
}

#[derive(Debug)]
pub struct LengthExceeded {

    max_length: LengthType,
    read_length: LengthType
}

impl LengthExceeded {

    pub fn new(max_length: LengthType, read_length: LengthType) -> Self {
        LengthExceeded { max_length, read_length }
    }

    pub fn get_max_length(&self) -> LengthType {
        self.max_length
    }

    pub fn get_read_length(&self) -> LengthType {
        self.read_length
    }
}