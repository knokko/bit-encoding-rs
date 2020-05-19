use crate::*;

/// An implementation of BitSink that forgets all data that is written to it.
/// Calls to write() and finish() will never return an error and will not panic.
pub struct VoidBitSink {}

impl VoidBitSink {

    /// Creates a new instance of VoidBitSink. 
    pub fn new() -> Self {
        Self {}
    }
}

impl BitSink for VoidBitSink {

    fn write(&mut self, _bits: &[bool]) -> Result<(),WriteError> {
        Ok(())
    }

    fn finish(&mut self) -> Result<(),WriteError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_write() {
        let mut void_sink = VoidBitSink::new();
        assert!(void_sink.write(&[true]).is_ok());
    }

    #[test]
    fn test_finish() {
        let mut void_sink = VoidBitSink::new();
        assert!(void_sink.finish().is_ok());
    }
}