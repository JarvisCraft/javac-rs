//! Structures related to methods of a class.

use crate::classfile_writable_mask_flags;

use crate::attribute::AttributeInfo;
use crate::writer::ClassfileWritable;
use crate::constpool::{ConstPoolIndex, ConstUtf8Info};
use std::io::Write;

/// Type of field attributes' count
type MethodAttributeCount = u16;

/// Method structure as specified by
/// [#4.6](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.6).
#[derive(Debug)]
pub struct MethodInfo {
    access_flags: MethodAccessFlags,
    name: ConstPoolIndex<ConstUtf8Info>,
    descriptor: ConstPoolIndex<ConstUtf8Info>,
    attributes: Vec<AttributeInfo>,
}

impl ClassfileWritable for MethodInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.access_flags.write_to_classfile(buffer);
        self.name.write_to_classfile(buffer);
        self.descriptor.write_to_classfile(buffer);
        (self.attributes.len() as MethodAttributeCount).write_to_classfile(buffer);
        for attribute in &self.attributes {
            attribute.write_to_classfile(buffer);
        }
    }
}

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub MethodAccessFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    MethodAccessFlags => {
       Public = 0x1,
       Private = 0x2,
       Protected = 0x4,
       Static = 0x8,
       Final = 0x10,
       Synchronized = 0x20,
       Bridge = 0x40,
       Varargs = 0x80,
       Native = 0x100,
       Abstract = 0x400,
       Strict = 0x800,
       Synthetic = 0x1000,
    }
}
