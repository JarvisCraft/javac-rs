use crate::class::{ClassfileWritable, Tagged};
use crate::classfile_writable;
use crate::constpool::{
    ConstDoubleInfo, ConstFloatInfo, ConstIntegerInfo, ConstLongInfo, ConstPoolIndex,
    ConstUtf8Info, RawConstPoolIndex,
};
use crate::vec::{JvmVecU1, JvmVecU2};

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct Annotation {
        type_name: ConstPoolIndex<ConstUtf8Info>,
        elements: JvmVecU2<NamedElementValue>,
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct NamedElementValue { name: ConstPoolIndex<ConstUtf8Info>, value: ElementValue }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ElementValue {
    Byte(ConstIntegerInfo),
    Char(ConstIntegerInfo),
    Double(ConstDoubleInfo),
    Float(ConstFloatInfo),
    Int(ConstIntegerInfo),
    Long(ConstLongInfo),
    Short(ConstIntegerInfo),
    Boolean(ConstIntegerInfo),
    String(ConstUtf8Info),
    EnumType {
        type_name: ConstPoolIndex<ConstUtf8Info>,
        const_name: ConstPoolIndex<ConstUtf8Info>,
    },
    Class(ConstPoolIndex<ConstUtf8Info>),
    AnnotationType(Annotation),
    ArrayType(JvmVecU2<ElementValue>),
}

impl ClassfileWritable for ElementValue {
    fn write_to_classfile(&self, buffer: &mut Vec<u8>) {
        self.tag().write_to_classfile(buffer);
        match self {
            Self::Byte(index) => index.write_to_classfile(buffer),
            Self::Char(index) => index.write_to_classfile(buffer),
            Self::Double(index) => index.write_to_classfile(buffer),
            Self::Float(index) => index.write_to_classfile(buffer),
            Self::Int(index) => index.write_to_classfile(buffer),
            Self::Long(index) => index.write_to_classfile(buffer),
            Self::Short(index) => index.write_to_classfile(buffer),
            Self::Boolean(index) => index.write_to_classfile(buffer),
            Self::String(index) => index.write_to_classfile(buffer),
            Self::EnumType {
                type_name,
                const_name,
            } => {
                type_name.write_to_classfile(buffer);
                const_name.write_to_classfile(buffer);
            }
            Self::Class(index) => index.write_to_classfile(buffer),
            Self::AnnotationType(annotation) => annotation.write_to_classfile(buffer),
            Self::ArrayType(elements) => elements.write_to_classfile(buffer),
        }
    }
}

impl Tagged for ElementValue {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        (match self {
            Self::Byte(..) => 'B',
            Self::Char(..) => 'C',
            Self::Double(..) => 'D',
            Self::Float(..) => 'F',
            Self::Int(..) => 'I',
            Self::Long(..) => 'J',
            Self::Short(..) => 'S',
            Self::Boolean(..) => 'Z',
            Self::String(..) => 's',
            Self::EnumType { .. } => 'e',
            Self::Class(..) => 'c',
            Self::AnnotationType(..) => '@',
            Self::ArrayType(..) => '[',
        }) as u8
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypeAnnotation {
        target: TargetInfo,
        type_path: TypePath,
        annotation: Annotation,
    }
}

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypePath {
        entries: JvmVecU1<TypePath>,
    }
);

// TODO type-safe alternative
classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypePathEntry { type_path_kind: u8, type_argument_index: u8 }
}

// see: 4.7.20.1
#[derive(Eq, PartialEq, Debug)]
enum TargetInfo {
    TypeParameterTarget(TypeParameterTargetKind, TypeParameterTargetInfo),
    SupertypeTarget(SupertypeTargetKind, SupertypeTargetInfo),
    TypeParameterBoundTarget(TypeParameterBoundTargetKind, TypeParameterBoundTargetInfo),
    EmptyTarget(EmptyTargetKind, EmptyTargetInfo),
    FormalParameterTarget(FormalParameterTargetKind, FormalParameterTargetInfo),
    ThrowsTarget(ThrowsTargetKind, ThrowsTargetInfo),
    LocalvarTarget(LocalvarTargetKind, LocalVarTargetInfo),
    CatchTarget(CatchTargetKind, CatchTargetInfo),
    OffsetTarget(OffsetTargetKind, OffsetTargetInfo),
    TypeArgumentTarget(TypeArgumentTargetKind, TypeArgumentTargetInfo),
}

#[derive(Eq, PartialEq, Debug)]
enum TypeParameterTargetKind {
    GenericClass,
    GenericInterface,
    GenericMethod,
    GenericConstructor,
}

impl Tagged for TypeParameterTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::GenericClass | Self::GenericInterface => 0x00,
            Self::GenericMethod | Self::GenericConstructor => 0x01,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum SupertypeTargetKind {
    ExtendedType,
    /* including interface extension */
    ImplementedType,
}

impl Tagged for SupertypeTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        0x10
    }
}

#[derive(Eq, PartialEq, Debug)]
enum TypeParameterBoundTargetKind {
    GenericClass,
    GenericInterface,
    GenericMethod,
    GenericConstructor,
}

impl Tagged for TypeParameterBoundTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::GenericClass | Self::GenericInterface => 0x11,
            Self::GenericMethod | Self::GenericConstructor => 0x12,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum EmptyTargetKind {
    Field,
    MethodReturn,
    ConstructorReturn,
    MethodReceiver,
    ConstructorReceiver,
}

impl Tagged for EmptyTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::Field => 0x13,
            Self::MethodReturn | Self::ConstructorReturn => 0x14,
            Self::MethodReceiver | Self::ConstructorReceiver => 0x15,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum FormalParameterTargetKind {
    MethodFormalParameter,
    ConstructorFormalParameter,
    LambdaExpressionFormalParameter,
}

impl Tagged for FormalParameterTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        0x16
    }
}

#[derive(Eq, PartialEq, Debug)]
enum ThrowsTargetKind {
    MethodThrowsClause,
    ConstructorThrowsClause,
}

impl Tagged for ThrowsTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        0x17
    }
}

#[derive(Eq, PartialEq, Debug)]
enum LocalvarTargetKind {
    LocalVariableDeclaration,
    ResourceVariableDeclaration,
}

impl Tagged for LocalvarTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::LocalVariableDeclaration => 0x40,
            Self::ResourceVariableDeclaration => 0x41,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum CatchTargetKind {
    ExceptionParameterDeclaration,
}

impl Tagged for CatchTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        0x42
    }
}

#[derive(Eq, PartialEq, Debug)]
enum OffsetTargetKind {
    InstanceofExpression,
    NewExpression,
    ConstructorReferenceExpression,
    MethodReferenceException,
}

impl Tagged for OffsetTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::InstanceofExpression => 0x44,
            Self::NewExpression => 0x44,
            Self::ConstructorReferenceExpression => 0x45,
            Self::MethodReferenceException => 0x46,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum TypeArgumentTargetKind {
    CastExpression,
    NewExpressionGenericConstructor,
    ConstructorInvocationStatement,
    MethodInvocationExpression,
    ConstructorReferenceExpression,
    MethodReferenceException,
}

impl Tagged for TypeArgumentTargetKind {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::CastExpression => 0x47,
            Self::NewExpressionGenericConstructor | Self::ConstructorInvocationStatement => 0x48,
            Self::MethodInvocationExpression => 0x49,
            Self::ConstructorReferenceExpression => 0x4A,
            Self::MethodReferenceException => 0x4B,
        }
    }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypeParameterTargetInfo { type_parameter_index: u8 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct SupertypeTargetInfo { supertype: RawConstPoolIndex /* interface or 65535 */ }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypeParameterBoundTargetInfo { type_parameter_index: u8, bound_index: u8 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct EmptyTargetInfo;
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct FormalParameterTargetInfo { formal_parameter_index: u8 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct ThrowsTargetInfo { throws_type_index: u16 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVarTargetInfo { table: JvmVecU2<LocalVarTableEntry> }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct LocalVarTableEntry { start_pc: u16, length: u16, index: u16 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct CatchTargetInfo { exception_table_index: u16 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct OffsetTargetInfo { offset: u16 }
}

classfile_writable! {
    #[derive(Eq, PartialEq, Debug)]
    pub struct TypeArgumentTargetInfo { offset: u16, type_argument_index: u8 }
}

impl ClassfileWritable for TargetInfo {
    fn write_to_classfile(&self, buffer: &mut Vec<u8>) {
        match self {
            Self::TypeParameterTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::SupertypeTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::TypeParameterBoundTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::EmptyTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::FormalParameterTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::ThrowsTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::LocalvarTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::CatchTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::OffsetTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
            Self::TypeArgumentTarget(kind, info) => {
                kind.tag().write_to_classfile(buffer);
                info.write_to_classfile(buffer);
            }
        }
    }
}
