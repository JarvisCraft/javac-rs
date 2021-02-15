//! Structures related to fields of a class.

use crate::attribute::{Attributable, AttributeAddError, NamedAttribute};
use crate::classfile_writable;
use crate::classfile_writable_mask_flags;
use crate::constpool::{ConstPoolIndex, ConstUtf8Info};
use crate::vec::JvmVecU2;

classfile_writable! {
    #[doc = "Field structure as specified by \
    [#4.5](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.5)"]
    #[derive(Eq, PartialEq, Debug)]
    pub struct FieldInfo {
        access_flags: FieldAccessFlags,
        name: ConstPoolIndex<ConstUtf8Info>,
        descriptor: ConstPoolIndex<ConstUtf8Info>,
        attributes: JvmVecU2<NamedAttribute>,
    }
}

impl FieldInfo {
    pub fn new(
        access_flags: FieldAccessFlags,
        name: ConstPoolIndex<ConstUtf8Info>,
        descriptor: ConstPoolIndex<ConstUtf8Info>,
    ) -> Self {
        Self {
            access_flags,
            name,
            descriptor,
            attributes: JvmVecU2::new(),
        }
    }
}

impl Attributable for FieldInfo {
    fn add_attribute(&mut self, attribute: NamedAttribute) -> Result<(), AttributeAddError> {
        self.attributes.push(attribute)?;
        Ok(())
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
