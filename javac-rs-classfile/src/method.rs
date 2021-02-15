//! Structures related to methods of a class.

use crate::classfile_writable_mask_flags;

use crate::attribute::{Attributable, AttributeAddError, NamedAttribute};
use crate::classfile_writable;
use crate::constpool::{ConstPoolIndex, ConstUtf8Info};
use crate::vec::JvmVecU2;

classfile_writable! {
    #[doc = "Method structure as specified by \
    [#4.6](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.6)."]
    #[derive(Eq, PartialEq, Debug)]
    pub struct MethodInfo {
        access_flags: MethodAccessFlags,
        name: ConstPoolIndex<ConstUtf8Info>,
        descriptor: ConstPoolIndex<ConstUtf8Info>,
        attributes: JvmVecU2<NamedAttribute>,
    }
}

impl MethodInfo {
    pub fn new(
        access_flags: MethodAccessFlags,
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

impl Attributable for MethodInfo {
    fn add_attribute(&mut self, attribute: NamedAttribute) -> Result<(), AttributeAddError> {
        self.attributes.push(attribute)?;
        Ok(())
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