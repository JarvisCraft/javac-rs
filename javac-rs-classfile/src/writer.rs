use crate::constpool::ConstPool;
use std::io::Write;

/// Writer responsible for writing data to class files.
pub struct ClassfileWriter<W: Write> {
    buffer: W,
    const_pool: ConstPool,
}

impl<W: Write> ClassfileWriter<W> {
    fn new(buffer: W) -> Self {
        Self {
            buffer,
            const_pool: ConstPool::new(),
        }
    }
}
