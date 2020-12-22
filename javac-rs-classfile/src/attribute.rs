//! Structures related to attributes of a class.

use crate::annotation::{Annotation, ElementValue, TypeAnnotation};
use crate::class::ClassAccessFlags;
use crate::classfile_writable;
use crate::constpool::{
    ConstClassInfo, ConstNameAndTypeInfo, ConstPackageInfo, ConstPoolIndex, ConstUtf8Info,
    LoadableConstPoolEntryInfo,
};
use crate::frame::StackMapFrame;
use crate::method::MethodAccessFlags;
use crate::module::{
    ModuleExports, ModuleFlags, ModuleOpens, ModuleProvides, ModuleRequires, ModuleUses,
};
use crate::vec::{JvmVecU1, JvmVecU2, JvmVecU4};
use std::io::Write;
use crate::writer::ClassfileWritable;

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
        match self {
            Self::ConstantValue(v) => v.write_to_classfile(buffer),
            Self::Code(v) => v.write_to_classfile(buffer),
            Self::StackMapTable(v) => v.write_to_classfile(buffer),
            Self::Exceptions(v) => v.write_to_classfile(buffer),
            Self::InnerClasses(v) => v.write_to_classfile(buffer),
            Self::EnclosingMethod(v) => v.write_to_classfile(buffer),
            Self::Synthetic(v) => v.write_to_classfile(buffer),
            Self::Signature(v) => v.write_to_classfile(buffer),
            Self::SourceFile(v) => v.write_to_classfile(buffer),
            Self::SourceDebugExtension(v) => v.write_to_classfile(buffer),
            Self::LineNumberTable(v) => v.write_to_classfile(buffer),
            Self::LocalVariableTable(v) => v.write_to_classfile(buffer),
            Self::LocalVariableTypeTable(v) => v.write_to_classfile(buffer),
            Self::Deprecated(v) => v.write_to_classfile(buffer),
            Self::RuntimeVisibleAnnotations(v) => v.write_to_classfile(buffer),
            Self::RuntimeInvisibleAnnotations(v) => v.write_to_classfile(buffer),
            Self::RuntimeVisibleParameterAnnotations(v) => v.write_to_classfile(buffer),
            Self::RuntimeInvisibleParameterAnnotations(v) => v.write_to_classfile(buffer),
            Self::RuntimeVisibleTypeAnnotations(v) => v.write_to_classfile(buffer),
            Self::RuntimeInvisibleTypeAnnotations(v) => v.write_to_classfile(buffer),
            Self::AnnotationDefault(v) => v.write_to_classfile(buffer),
            Self::BootstrapMethods(v) => v.write_to_classfile(buffer),
            Self::MethodParameters(v) => v.write_to_classfile(buffer),
            Self::Module(v) => v.write_to_classfile(buffer),
            Self::ModulePackages(v) => v.write_to_classfile(buffer),
            Self::ModuleMainClass(v) => v.write_to_classfile(buffer),
            Self::NestHost(v) => v.write_to_classfile(buffer),
            Self::NestMembers(v) => v.write_to_classfile(buffer),
            Self::Custom(v) => v.write_to_classfile(buffer),
        }
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstantValueAttribute { value: ConstPoolIndex<ConstUtf8Info> }
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
        name: ConstPoolIndex<ConstUtf8Info>,
        classes: JvmVecU4<u8>,
    }
);
