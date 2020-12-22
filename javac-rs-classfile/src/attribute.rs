//! Structures related to attributes of a class.

use crate::annotation::{Annotation, ElementValue, TypeAnnotation};
use crate::class::ClassAccessFlags;
use crate::classfile_writable;
use crate::constpool::{ConstClassInfo, ConstNameAndTypeInfo, ConstPackageInfo, ConstPoolIndex, ConstUtf8Info, LoadableConstPoolEntryInfo, ConstPool, ConstValueInfo, ConstValue, ConstPoolStoreError};
use crate::frame::StackMapFrame;
use crate::method::MethodAccessFlags;
use crate::module::{
    ModuleExports, ModuleFlags, ModuleOpens, ModuleProvides, ModuleRequires, ModuleUses,
};
use crate::vec::{JvmVecU1, JvmVecU2, JvmVecU4, JvmVecStoreError};
use std::io::Write;
use crate::writer::ClassfileWritable;
use thiserror::Error;
use std::convert::TryFrom;

/// An error which may occur while creating a new attribute.
#[derive(Error, Debug)]
pub enum AttributeCreateError {
    #[error("Target const pool is out of space")]
    ConstPoolOutOfSpace(#[from] ConstPoolStoreError),
}

/// An error which may occur while adding a new attribute.
#[derive(Error, Debug)]
pub enum AttributeAddError {
    #[error("JVM vector of attributes is out of space")]
    OutOfSpace(#[from] JvmVecStoreError),
}

classfile_writable! {
    #[doc = "Named attribute."]
    #[derive(Eq, PartialEq, Debug)]
    pub struct NamedAttribute {
        name: ConstPoolIndex<ConstUtf8Info>,
        info: AttributeInfo,
    }
}

impl NamedAttribute {
    pub fn new(name: ConstPoolIndex<ConstUtf8Info>, info: AttributeInfo) -> Self {
        Self { name, info }
    }

    // Factories for creation of attributes

    pub fn new_const_value_attribute(const_pool: &mut ConstPool, value: ConstValue)
                                     -> Result<NamedAttribute, AttributeCreateError> {
        let name = const_pool.store_const_utf8_info(String::from("ConstantValue"))?;
        let value = const_pool.store_const_value_info(value)?;

        Ok(NamedAttribute { name, info: AttributeInfo::ConstantValue(ConstantValueAttribute { value }) })
    }

    pub fn new_custom_attribute(const_pool: &mut ConstPool, name: String, payload: JvmVecU4<u8>)
                                -> Result<NamedAttribute, AttributeCreateError> {
        Ok(NamedAttribute { name: const_pool.store_const_utf8_info(name)?, info: AttributeInfo::Custom(CustomAttribute { payload }) })
    }
}

/// A class member which may have attributes.
pub trait Attributable {
    fn add_attribute(&mut self, attribute: NamedAttribute) -> Result<(), AttributeAddError>;
}

///"Attribute of classfile member as specified by
/// [#4.7](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.7).
#[derive(Eq, PartialEq, Debug)]
pub enum AttributeInfo {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    Signature(SignatureAttribute),
    SourceFile(SourceFileAttribute),
    SourceDebugExtension(SourceDebugExtensionAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    Deprecated(DeprecatedAttribute),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotationsAttribute),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotationsAttribute),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsAttribute),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotationsAttribute),
    RuntimeInvisibleTypeAnnotations(RuntimeInvisibleTypeAnnotationsAttribute),
    AnnotationDefault(AnnotationDefaultAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    MethodParameters(MethodParametersAttribute),
    Module(ModuleAttribute),
    ModulePackages(ModulePackagesAttribute),
    ModuleMainClass(ModuleMainClassAttribute),
    NestHost(NestHostAttribute),
    NestMembers(NestMembersAttribute),
    Custom(CustomAttribute),
}

impl ClassfileWritable for AttributeInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        if let Self::Custom(custom) = self {
            custom.write_to_classfile(buffer);
        } else {
            // TODO alloc-free implementation
            let mut tmp_buffer = Vec::<u8>::new();
            match self {
                Self::ConstantValue(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Code(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::StackMapTable(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Exceptions(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::InnerClasses(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::EnclosingMethod(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Synthetic(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Signature(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::SourceFile(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::SourceDebugExtension(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::LineNumberTable(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::LocalVariableTable(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::LocalVariableTypeTable(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Deprecated(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeVisibleAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeInvisibleAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeVisibleParameterAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeInvisibleParameterAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeVisibleTypeAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::RuntimeInvisibleTypeAnnotations(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::AnnotationDefault(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::BootstrapMethods(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::MethodParameters(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Module(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::ModulePackages(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::ModuleMainClass(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::NestHost(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::NestMembers(v) => v.write_to_classfile(&mut tmp_buffer),
                Self::Custom(..) => unsafe { ::std::hint::unreachable_unchecked() },
            };
            // TODO get rid of unwrapping by using result for return type
            JvmVecU4::try_from(tmp_buffer).unwrap().write_to_classfile(buffer);
        }
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstantValueAttribute { value: ConstPoolIndex<ConstValueInfo> }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct CodeAttribute {
            max_stack: u16,
            max_locals: u16,
            code: JvmVecU4<u8>,
            exception_tables: JvmVecU2<ExceptionTable>,
            attributes: JvmVecU2<AttributeInfo>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct ExceptionTable {
        start_pc: u16,
        end_pc: u16,
        handler_prc: u16,
        catch_type: ConstPoolIndex<ConstClassInfo>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct StackMapTableAttribute {
        stack_map_frame: JvmVecU2<StackMapFrame>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct ExceptionsAttribute { exceptions: JvmVecU2<ConstPoolIndex<ConstClassInfo>> }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct InnerClassesAttribute { classes: JvmVecU2<InnerClassInfo> }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct InnerClassInfo {
        inner_class: ConstPoolIndex<ConstClassInfo>,
        outer_class: ConstPoolIndex<ConstClassInfo>,
        inner_name: ConstPoolIndex<ConstUtf8Info>,
        inner_class_access_flags: ClassAccessFlags,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct EnclosingMethodAttribute {
        class: ConstPoolIndex<ConstClassInfo>,
        method: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct SyntheticAttribute;
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct SignatureAttribute {
        signature: ConstPoolIndex<ConstUtf8Info>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct SourceFileAttribute {
        sourcefile: ConstPoolIndex<ConstUtf8Info>,
    }
}

// This is not implemented via `classfile_writable!` as its buffer
// is written without explicit length
#[derive(Eq, PartialEq, Debug)]
pub struct SourceDebugExtensionAttribute {
    debug_extension: Vec<u8>,
}

impl ClassfileWritable for SourceDebugExtensionAttribute {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        for entry in &self.debug_extension {
            entry.write_to_classfile(buffer);
        }
    }
}

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LineNumberTableAttribute {
        line_number_table: JvmVecU2<LineNumberTableEntry>,
    }
);
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LineNumberTableEntry {
        start_pc: u16,
        line_number: u16,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVariableTableAttribute {
        local_variable_table: JvmVecU2<LocalVariableTableEntry>,
    }
);
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVariableTableEntry {
        start_pc: u16,
        length: u16,
        name: ConstPoolIndex<ConstUtf8Info>,
        descriptor: ConstPoolIndex<ConstUtf8Info>,
        index: u16,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVariableTypeTableAttribute {
        local_variable_type_table: JvmVecU2<LocalVariableTypeTableEntry>,
    }
);
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVariableTypeTableEntry {
        start_pc: u16,
        length: u16,
        name: ConstPoolIndex<ConstUtf8Info>,
        signature: ConstPoolIndex<ConstUtf8Info>,
        index: u16,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct DeprecatedAttribute;
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct AnnotationEntry;
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeVisibleAnnotationsAttribute {
        annotations: JvmVecU2<Annotation>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeInvisibleAnnotationsAttribute {
        annotations: JvmVecU2<Annotation>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeVisibleParameterAnnotationsAttribute {
        annotations: JvmVecU1<JvmVecU2<Annotation>>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeInvisibleParameterAnnotationsAttribute {
        annotations: JvmVecU1<JvmVecU2<Annotation>>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeVisibleTypeAnnotationsAttribute {
        annotations: JvmVecU2<TypeAnnotation>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct RuntimeInvisibleTypeAnnotationsAttribute {
        annotations: JvmVecU2<TypeAnnotation>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct AnnotationDefaultAttribute {
        value: ElementValue,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct BootstrapMethodsAttribute {
        bootstrap_methods: JvmVecU2<BootstrapMethod>,
    }
);
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct BootstrapMethod {
        bootstrap_method: ConstPoolIndex<LoadableConstPoolEntryInfo>,
        bootstrap_arguments: JvmVecU2<ConstPoolIndex<LoadableConstPoolEntryInfo>>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct MethodParametersAttribute {
        method_parameters: JvmVecU1<MethodParameter>,
    }
);
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct MethodParameter {
        name: ConstPoolIndex<ConstUtf8Info>,
        access_flags: MethodAccessFlags,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleAttribute {
        name: ConstPoolIndex<ConstUtf8Info>,
        flags: ModuleFlags,
        version: ConstPoolIndex<ConstUtf8Info>,
        requires: JvmVecU2<ModuleRequires>,
        exports: JvmVecU2<ModuleExports>,
        opens: JvmVecU2<ModuleOpens>,
        uses: JvmVecU2<ModuleUses>,
        provides: JvmVecU2<ModuleProvides>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModulePackagesAttribute {
        packages: JvmVecU2<ConstPoolIndex<ConstPackageInfo>>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleMainClassAttribute {
        main_class: ConstPoolIndex<ConstClassInfo>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct NestHostAttribute {
        host_class: ConstPoolIndex<ConstClassInfo>,
    }
);

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct NestMembersAttribute {
        classes: JvmVecU2<ConstPoolIndex<ConstClassInfo>>,
    }
);

// note: this is not a real attribute, it's a *placeholder* for any custom named attribute
classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct CustomAttribute {
        payload: JvmVecU4<u8>,
    }
);
