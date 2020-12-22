//! Implementation of classfile-specific logic as specified by
//! [#4](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html).

use crate::attribute::{Attributable, NamedAttribute, AttributeCreateError, AttributeAddError};
use crate::constpool::{ConstClassInfo, ConstPool, ConstPoolIndex, ConstPoolStoreError, ConstValue};
use crate::defs::CLASSFILE_HEADER;
use crate::field::{FieldInfo, FieldAccessFlags};
use crate::method::MethodInfo;
use crate::vec::{JvmVecU2, JvmVecStoreError, JvmVecU4};
use std::io::Write;
use thiserror::Error;
use crate::writer::ClassfileWritable;

pub trait Tagged {
    type TagType;

    fn tag(&self) -> Self::TagType;
}

// TODO rewrite via Derive
#[macro_export]
macro_rules! classfile_writable {
    (
        $(#$struct_attribute:tt)*
        $struct_visibility:vis struct $struct_name:ident{$(
            $(#$field_attribute:tt)*
            $field_visibility:vis $field:ident: $type:ty
        ),*$(,)?}
    ) => {
        $(#$struct_attribute)*
        $struct_visibility struct $struct_name {$(
            $(#$field_attribute)*
            $field_visibility $field: $type,
        )*}

        impl $crate::writer::ClassfileWritable for $struct_name {
            fn write_to_classfile<W: ::std::io::Write>(&self, buffer: &mut W) {
                $(self.$field.write_to_classfile(buffer);)*
            }
        }
    };
    (
        $(#$struct_attribute:tt)*
        $struct_visibility:vis struct $struct_name:ident;
    ) => {
        $(#$struct_attribute)*
        $struct_visibility struct $struct_name;

        impl $crate::writer::ClassfileWritable for $struct_name {
            fn write_to_classfile<W: ::std::io::Write>(&self, _: &mut W) {}
        }
    }
}

#[macro_export]
macro_rules! classfile_writable_mask_flags {
    (
        $(#$flag_attribute:tt)*
        $visibility:vis $flag_name:ident as $number:ty=$default:expr;

        $(#$flags_attribute:tt)*
        $flags_name:ident => {$(
            $key:ident=$value:expr,
        )*}
    ) => {
        $crate::mask_flags! {
            $(#$flag_attribute)*
            $visibility $flag_name as $number = $default;

            $(#$flags_attribute)*
            $flags_name => {$(
                $key = $value,
            )*}
        }

        impl $crate::writer::ClassfileWritable for $flags_name {
            fn write_to_classfile<W: ::std::io::Write>(&self, buffer: &mut W) {
                self.mask().write_to_classfile(buffer);
            }
        }
    }
}

/// Version of a classfile.
#[derive(Debug)]
pub struct ClassfileVersion {
    major_version: u16,
    minor_version: u16,
}

impl ClassfileVersion {
    pub fn of(major_version: u16, minor_version: u16) -> Self {
        Self {
            major_version,
            minor_version,
        }
    }

    pub fn of_major(major_version: u16) -> Self {
        Self::of(major_version, 0)
    }

    pub fn major(&self) -> u16 {
        self.major_version
    }

    pub fn minor(&self) -> u16 {
        self.minor_version
    }
}

impl Clone for ClassfileVersion {
    fn clone(&self) -> Self {
        Self::of(self.major_version, self.minor_version)
    }
}

// ClassfileVersion can be copied as it is a small structure of non-aligned size of 4 bytes
impl Copy for ClassfileVersion {}

impl ClassfileWritable for ClassfileVersion {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        // note: the order differs from the structure
        self.minor_version.write_to_classfile(buffer);
        self.major_version.write_to_classfile(buffer);
    }
}

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub ClassAccessFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    ClassAccessFlags => {
        Public = 0x0001,
        Final = 0x0010,
        Super = 0x20,
        Interface = 0x200,
        Abstract = 0x400,
        Synthetic = 0x1000,
        Annotation = 0x2000,
        Enum = 0x4000,
    }
}

impl ClassfileWritable for [u8] {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        buffer.write(self).unwrap();
    }
}

macro_rules! impl_primitive_classfile_writable {
    ($($numeric:ty)*) => {$(
        impl $crate::writer::ClassfileWritable for $numeric {
            fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
                buffer.write(self.to_be_bytes().as_ref()).unwrap();
            }
        }
    )*};
}

impl_primitive_classfile_writable!(u8 u16 u32 u64);

#[derive(Error, Debug)]
pub enum ClassUpdateError {
    #[error("Const pool of the class is out of space")]
    ConstPoolStore(#[from] ConstPoolStoreError),
    #[error("Const pool of the class is out of space")]
    VecStore(#[from] JvmVecStoreError),
    #[error("Attribute could not be created")]
    AttributeCreate(#[from] AttributeCreateError),
    #[error("Attribute could not be added")]
    AttributeAdd(#[from] AttributeAddError),
    #[error("The specified field index is incorrect")]
    InvalidFieldIndex,
    #[error("The specified method index is incorrect")]
    InvalidMethodIndex,
}

/// Classfile structure including all its nested members as specified in
/// [#4.1](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.1)
#[derive(Debug)]
pub struct Class {
    version: ClassfileVersion,
    const_pool: ConstPool,
    access_flags: ClassAccessFlags,
    this_class: ConstPoolIndex<ConstClassInfo>,
    super_class: ConstPoolIndex<ConstClassInfo>,
    interfaces: JvmVecU2<ConstPoolIndex<ConstClassInfo>>,
    fields: JvmVecU2<FieldInfo>,
    methods: JvmVecU2<MethodInfo>,
    attributes: JvmVecU2<NamedAttribute>,
}

/// Type of an index of a field in a class.
pub type ClassFieldIndex = u16;

/// Type of an index of a method in a class.
pub type ClassMethodIndex = u16;

impl Class {
    pub fn new(
        version: ClassfileVersion,
        access_flags: ClassAccessFlags,
        this_class: String,
        super_class: String,
    ) -> Self {
        let mut const_pool = ConstPool::new();
        // const-pool should contain minimal space thus unwraps are safe
        let this_class = const_pool.store_const_class_info(this_class).unwrap();
        let super_class = const_pool.store_const_class_info(super_class).unwrap();
        Self {
            version,
            const_pool,
            access_flags,
            this_class,
            super_class,
            interfaces: JvmVecU2::new(),
            fields: JvmVecU2::new(),
            methods: JvmVecU2::new(),
            attributes: JvmVecU2::new(),
        }
    }

    pub fn add_interface(&mut self, interface: String) -> Result<(), ClassUpdateError> {
        let interface = self.const_pool.store_const_class_info(interface)?;
        self.interfaces.push(interface)?;
        Ok(())
    }

    pub fn add_field(&mut self, access_flags: FieldAccessFlags, name: String, descriptor: String)
                     -> Result<ClassFieldIndex, ClassUpdateError> {
        let name = self.const_pool.store_const_utf8_info(name)?;
        let descriptor = self.const_pool.store_const_utf8_info(descriptor)?;
        self.fields.push(FieldInfo::new(access_flags, name, descriptor)).map_err(|e| e.into())
    }

    fn add_method(&mut self, _method: MethodInfo) -> Result<(), ClassUpdateError> {
        unimplemented!() // TODO
    }

    pub fn add_source_file_attribute(&mut self, filename: String) -> Result<(), ClassUpdateError> {
        let attribute = NamedAttribute::new_source_file_attribute(&mut self.const_pool, filename)?;
        self.add_attribute(attribute)?;
        Ok(())
    }

    pub fn add_synthetic_attribute(&mut self) -> Result<(), ClassUpdateError> {
        let attribute = NamedAttribute::new_synthetic_attribute(&mut self.const_pool)?;
        self.add_attribute(attribute)?;
        Ok(())
    }

    pub fn add_deprecated_attribute(&mut self) -> Result<(), ClassUpdateError> {
        let attribute = NamedAttribute::new_deprecated_attribute(&mut self.const_pool)?;
        self.add_attribute(attribute)?;
        Ok(())
    }

    pub fn add_signature_attribute(&mut self, signature: String) -> Result<(), ClassUpdateError> {
        let attribute = NamedAttribute::new_signature_attribute(&mut self.const_pool, signature)?;
        self.add_attribute(attribute)?;
        Ok(())
    }

    pub fn add_custom_attribute(&mut self, name: String, payload: JvmVecU4<u8>) -> Result<(), ClassUpdateError> {
        let attribute = NamedAttribute::new_custom_attribute(&mut self.const_pool, name, payload)?;
        self.add_attribute(attribute)?;
        Ok(())
    }

    // Field updaters

    pub fn field_add_const_value_attribute(&mut self, field_index: ClassFieldIndex, value: ConstValue)
                                           -> Result<(), ClassUpdateError> {
        let field = self.fields.get_mut(field_index).ok_or(ClassUpdateError::InvalidFieldIndex)?;

        let attribute = NamedAttribute::new_const_value_attribute(
            &mut self.const_pool, value,
        )?;
        field.add_attribute(attribute);
        Ok(())
    }


    pub fn field_add_synthetic_attribute(&mut self, field_index: ClassFieldIndex) -> Result<(), ClassUpdateError> {
        let field = self.fields.get_mut(field_index).ok_or(ClassUpdateError::InvalidFieldIndex)?;

        let attribute = NamedAttribute::new_synthetic_attribute(&mut self.const_pool)?;
        field.add_attribute(attribute)?;
        Ok(())
    }

    pub fn field_add_deprecated_attribute(&mut self, field_index: ClassFieldIndex) -> Result<(), ClassUpdateError> {
        let field = self.fields.get_mut(field_index).ok_or(ClassUpdateError::InvalidFieldIndex)?;

        let attribute = NamedAttribute::new_deprecated_attribute(&mut self.const_pool)?;
        field.add_attribute(attribute)?;
        Ok(())
    }

    pub fn field_add_signature_attribute(&mut self, field_index: ClassFieldIndex, signature: String) -> Result<(), ClassUpdateError> {
        let field = self.fields.get_mut(field_index).ok_or(ClassUpdateError::InvalidFieldIndex)?;

        let attribute = NamedAttribute::new_signature_attribute(&mut self.const_pool, signature)?;
        field.add_attribute(attribute)?;
        Ok(())
    }

    pub fn field_add_custom_attribute(&mut self, field_index: ClassFieldIndex, name: String, payload: JvmVecU4<u8>) -> Result<(), ClassUpdateError> {
        let field = self.fields.get_mut(field_index).ok_or(ClassUpdateError::InvalidFieldIndex)?;

        let attribute = NamedAttribute::new_custom_attribute(&mut self.const_pool, name, payload)?;
        field.add_attribute(attribute)?;
        Ok(())
    }
}

impl Attributable for Class {
    fn add_attribute(&mut self, attribute: NamedAttribute) -> Result<(), AttributeAddError> {
        self.attributes.push(attribute)?;
        Ok(())
    }
}

// Classfile itself is also classfile writable although it is meant to be a top-level node
impl ClassfileWritable for Class {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        CLASSFILE_HEADER.write_to_classfile(buffer);

        self.version.write_to_classfile(buffer);

        self.const_pool.write_to_classfile(buffer);

        self.access_flags.write_to_classfile(buffer);

        self.this_class.write_to_classfile(buffer);
        self.super_class.write_to_classfile(buffer);

        self.interfaces.write_to_classfile(buffer);
        self.fields.write_to_classfile(buffer);
        self.methods.write_to_classfile(buffer);
        self.attributes.write_to_classfile(buffer);
    }
}
