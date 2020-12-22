//! Structures related to fields of a class.

use crate::attribute::AttributeInfo;
use crate::writer::ClassfileWritable;
use crate::classfile_writable_mask_flags;
use crate::constpool::{ConstPoolIndex, ConstUtf8Info};
use std::io::Write;

/// Type of field attributes' count
type FieldAttributeCount = u16;

/// Field structure as specified by
/// [#4.5](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.5).
#[derive(Debug)]
pub struct FieldInfo {
    access_flags: FieldAccessFlags,
    name: ConstPoolIndex<ConstUtf8Info>,
    descriptor: ConstPoolIndex<ConstUtf8Info>,
    attributes: Vec<AttributeInfo>,
}

impl ClassfileWritable for FieldInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.access_flags.write_to_classfile(buffer);
        self.name.write_to_classfile(buffer);
        self.descriptor.write_to_classfile(buffer);
        (self.attributes.len() as FieldAttributeCount).write_to_classfile(buffer);
        for attribute in &self.attributes {
            attribute.write_to_classfile(buffer);
        }
    }
}

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub FieldAccessFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    FieldAccessFlags => {
        Public = 0x1,
        Private = 0x2,
        Protected = 0x4,
        Static = 0x8,
        Final = 0x10,
        Volatile = 0x40,
        Transient = 0x80,
        Synthetic = 0x1000,
        Enum = 0x4000,
    }
}
