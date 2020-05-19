mod errors;

pub use errors::*;

pub trait BitSource {

    fn read(&self, dest: &[bool]);
}