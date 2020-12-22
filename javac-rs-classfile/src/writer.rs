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

    /// Writes the bytes of a class into a newly created buffer.
    fn to_classfile_bytes<W: Write + Default>(&self) -> W {
        let mut buffer = W::default();
        self.write_to_classfile(&mut buffer);

        buffer
    }
}
