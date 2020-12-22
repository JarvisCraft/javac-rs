use crate::class::Tagged;
use crate::constpool::{ConstClassInfo, ConstPoolIndex};
use crate::vec::JvmVecU2;
use std::io::Write;
use crate::writer::ClassfileWritable;

#[derive(Eq, PartialEq, Debug)]
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
        /* = 0-63 */
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        /* = 64-127 */
        stack: [VerificationTypeInfo; 1], // For consistency with specification
    },
    SameLocals1StackItemFrameExtended {
        /* frame_type: u8 = 247 */
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1], // For consistency with specification
    },
    ChopFrame {
        frame_type: u8,
        /* = 248-250 */
        offset_delta: u16,
    },
    SameFrameExtended {
        /* frame_type: u8 = 251 */ offset_delta: u16,
    },
    AppendFrame252 {
        offset_delta: u16,
        locals: [VerificationTypeInfo; 1],
    },
    AppendFrame253 {
        offset_delta: u16,
        locals: [VerificationTypeInfo; 2],
    },
    AppendFrame254 {
        offset_delta: u16,
        locals: [VerificationTypeInfo; 3],
    },
    FullFrame {
        /* frame+type: u8 = 255 */
        offset_delta: u16,
        locals: JvmVecU2<VerificationTypeInfo>,
        stack: JvmVecU2<VerificationTypeInfo>,
    },
}

impl StackMapFrame {
    fn frame_type(&self) -> u8 {
        match self {
            Self::SameFrame { frame_type } => frame_type.clone(),
            Self::SameLocals1StackItemFrame { frame_type, .. } => frame_type.clone(),
            Self::SameLocals1StackItemFrameExtended { .. } => 247,
            Self::ChopFrame { frame_type, .. } => frame_type.clone(),
            Self::SameFrameExtended { .. } => 251,
            Self::AppendFrame252 { .. } => 252,
            Self::AppendFrame253 { .. } => 253,
            Self::AppendFrame254 { .. } => 254,
            Self::FullFrame { .. } => 255,
        }
    }
}

impl ClassfileWritable for StackMapFrame {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.frame_type().write_to_classfile(buffer);
        match self {
            Self::SameFrame { .. } => {}
            Self::SameLocals1StackItemFrame { stack, .. } => {
                for entry in stack {
                    entry.write_to_classfile(buffer);
                }
            }
            Self::SameLocals1StackItemFrameExtended {
                offset_delta,
                stack,
                ..
            } => {
                offset_delta.write_to_classfile(buffer);
                for entry in stack {
                    entry.write_to_classfile(buffer);
                }
            }
            Self::ChopFrame { offset_delta, .. } => {
                offset_delta.write_to_classfile(buffer);
            }
            Self::SameFrameExtended { offset_delta, .. } => {
                offset_delta.write_to_classfile(buffer);
            }
            Self::AppendFrame252 {
                offset_delta,
                locals,
            } => {
                offset_delta.write_to_classfile(buffer);
                for local in locals {
                    local.write_to_classfile(buffer);
                }
            }
            Self::AppendFrame253 {
                offset_delta,
                locals,
            } => {
                offset_delta.write_to_classfile(buffer);
                for local in locals {
                    local.write_to_classfile(buffer);
                }
            }
            Self::AppendFrame254 {
                offset_delta,
                locals,
            } => {
                offset_delta.write_to_classfile(buffer);
                for local in locals {
                    local.write_to_classfile(buffer);
                }
            }
            Self::FullFrame {
                offset_delta,
                locals,
                stack,
            } => {
                offset_delta.write_to_classfile(buffer);
                locals.write_to_classfile(buffer);
                stack.write_to_classfile(buffer);
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum VerificationTypeInfo {
    Top,
    Integer,
    Float,
    Null,
    UninitializedThis,
    Object {
        class: ConstPoolIndex<ConstClassInfo>,
    },
    UninitializedVariable {
        offset: u16,
    },
    Long,
    Double,
}

impl Tagged for VerificationTypeInfo {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            VerificationTypeInfo::Top => 0,
            VerificationTypeInfo::Integer => 1,
            VerificationTypeInfo::Float => 2,
            VerificationTypeInfo::Null => 5,
            VerificationTypeInfo::UninitializedThis => 6,
            VerificationTypeInfo::Object { .. } => 7,
            VerificationTypeInfo::UninitializedVariable { .. } => 8,
            VerificationTypeInfo::Long => 4,
            VerificationTypeInfo::Double => 3,
        }
    }
}

impl ClassfileWritable for VerificationTypeInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.tag().write_to_classfile(buffer);
        match self {
            VerificationTypeInfo::Top => {}
            VerificationTypeInfo::Integer => {}
            VerificationTypeInfo::Float => {}
            VerificationTypeInfo::Null => {}
            VerificationTypeInfo::UninitializedThis => {}
            VerificationTypeInfo::Object { class } => {
                class.write_to_classfile(buffer);
            }
            VerificationTypeInfo::UninitializedVariable { offset } => {
                offset.write_to_classfile(buffer);
            }
            VerificationTypeInfo::Long => {}
            VerificationTypeInfo::Double => {}
        }
    }
}
