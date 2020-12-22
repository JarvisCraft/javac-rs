use crate::constpool::ConstPool;
use std::io::Write;

/// An object which can be written into classfile.
pub trait ClassfileWritable {
    /// Writes the bytes of a class into the given buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - classfile byte-buffer into which this object should be written
    /// * `const_pool` - const pool to be used for
    fn write_to_classfile<W: Write>(&self, buffer: &mut W);

    /// Creates a new [byte-vector](Vec) from this object.
    fn to_classfile_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.write_to_classfile(&mut buffer);

        buffer
    }
}
