//! Utilities for generating bytecode

use crate::attribute::{
    Attributable, AttributeAddError, AttributeCreateError, ExceptionTableInfo, NamedAttribute,
    TryIntoNamedAttribute,
};
use crate::{
    ConstClassInfo, ConstFieldRefInfo, ConstInterfaceMethodRefInfo, ConstInvokeDynamicInfo,
    ConstMethodRefInfo, ConstPool, ConstPoolEntryInfo, ConstPoolIndex, JvmVecStoreError, JvmVecU2,
    JvmVecU4, TypeDescriptor,
};
use std::cmp::max;

use std::num::NonZeroU8;
use thiserror::Error;

#[derive(Debug)]
pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_prc: u16,
    /// Name of a catched type
    catch_type: String,
}

impl ExceptionTable {
    pub fn into_attribute_analog(
        self,
        const_pool: &mut ConstPool,
    ) -> Result<ExceptionTableInfo, AttributeCreateError> {
        ExceptionTableInfo::new(
            const_pool,
            self.start_pc,
            self.end_pc,
            self.handler_prc,
            self.catch_type,
        )
    }
}

#[derive(Debug)]
pub struct Bytecode {
    max_stack: u16,
    max_locals: u16,
    code: JvmVecU4<u8>,
    exception_tables: JvmVecU2<ExceptionTable>,
    attributes: JvmVecU2<NamedAttribute>,
    // state
    stack: u16,
}

/// An error which may occur while updating bytecode.
#[derive(Error, Debug)]
pub enum BytecodeUpdateError {
    #[error("Bytecode is out of space")]
    OutOfSpace(#[from] JvmVecStoreError),
    #[error("Stack is corrupted")]
    CorruptedStack,
    #[error("Index of the local variable is out of bounds")]
    LocalIndexOutOfBounds,
    #[error("Too much method parameters were passed to a method invocation instruction")]
    TooMuchMethodParameters,
    #[error("Given type cannot be used in the given context")]
    InvalidType,
}

impl Bytecode {
    const ZERO_BYTE_ARRAY_1: [u8; 2] = [0, 0];
    const ZERO_BYTE_ARRAY_2: [u8; 2] = [0, 0];

    pub fn new(max_locals: u16) -> Self {
        Self {
            max_stack: 0,
            max_locals,
            code: JvmVecU4::new(),
            exception_tables: JvmVecU2::new(),
            attributes: JvmVecU2::new(),
            stack: 0,
        }
    }

    fn check_local_index(&self, local_index: u16) -> Result<(), BytecodeUpdateError> {
        if local_index >= self.max_locals {
            Err(BytecodeUpdateError::LocalIndexOutOfBounds)
        } else {
            Ok(())
        }
    }

    fn stack_update(&mut self, dec: u16, inc: u16) -> Result<(), BytecodeUpdateError> {
        match (dec, inc) {
            (0, 0) => {} // no need to verify stack as no changes happen
            (dec, 0) => {
                self.stack = self
                    .stack
                    .checked_sub(dec)
                    .ok_or(BytecodeUpdateError::CorruptedStack)?
            }
            (0, inc) => {
                self.stack = self
                    .stack
                    .checked_add(inc)
                    .ok_or(BytecodeUpdateError::CorruptedStack)?;
                self.max_stack = max(self.max_stack, self.stack);
            }
            (inc, dec) => {
                self.stack = self
                    .stack
                    .checked_sub(dec)
                    .and_then(|stack| stack.checked_add(inc))
                    .ok_or(BytecodeUpdateError::CorruptedStack)?;
                self.max_stack = max(self.max_stack, self.stack);
            }
        };
        Ok(())
    }

    fn push_instr(&mut self, opcode: u8) -> Result<BytecodeOffset, BytecodeUpdateError> {
        Ok(self.code.push(opcode)?)
    }

    fn push_ops(&mut self, operands: &[u8]) -> Result<BytecodeOffset, BytecodeUpdateError> {
        Ok(self.code.extend_from_slice(operands)?)
    }

    const fn slot_size(fat: bool) -> u16 {
        if fat {
            2
        } else {
            1
        }
    }

    #[inline(always)] // used only privately to reduce code duplication
    fn access_local(
        &mut self,
        generic_opcode: u8,
        specific_0_opcode: u8,
        index: u16,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        match index {
            0..=3 => self.push_instr(specific_0_opcode + index as u8),
            index if index < u8::MAX as u16 => self
                .stack_update(0, 1)
                .and(self.push_instr(generic_opcode))
                .and(self.push_ops(&[index as u8])),
            _ => self
                .push_instr(0xc4)
                .and(self.push_instr(generic_opcode))
                .and(self.push_ops(&index.to_be_bytes())),
        }
    }

    #[inline(always)] // used only privately to reduce code duplication
    fn instr_load(
        &mut self,
        generic_opcode: u8,
        specific_0_opcode: u8,
        index: u16,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.check_local_index(index)
            .and(self.stack_update(0, Self::slot_size(fat)))
            .and(self.access_local(generic_opcode, specific_0_opcode, index))
    }

    #[inline(always)] // used only privately to reduce code duplication
    fn instr_store(
        &mut self,
        generic_opcode: u8,
        specific_0_opcode: u8,
        index: u16,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.check_local_index(index)
            .and(self.stack_update(Self::slot_size(fat), 0))
            .and(self.access_local(generic_opcode, specific_0_opcode, index))
    }
}

pub type BytecodeOffset = u32;

/// Implementation of bytecode modifying methods.
/// Current API is experimental and is likely to change to a safer one.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use javac_rs_classfile::Bytecode;
/// // Let the signature be `(Ljava/lang/String;)Ljava/lang/String;` and the method return `null`
/// let mut bytecode = Bytecode::new(1);
/// bytecode.instr_aconst_null().unwrap(); // put `null` onto the operand stack
/// bytecode.instr_areturn().unwrap(); // return the value
/// ```
impl Bytecode {
    pub fn instr_aaload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x32))
    }

    pub fn instr_aastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x53))
    }

    pub fn instr_aconst_null(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x1))
    }

    pub fn instr_aload(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_load(0x19, 0x2a, index, false)
    }

    pub fn instr_anewarray(
        &mut self,
        component_type: ConstPoolIndex<ConstClassInfo>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1)
            .and(self.push_instr(0x19))
            .and(self.push_ops(&component_type.as_bytes()))
    }

    pub fn instr_areturn(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0xb0))
    }

    pub fn instr_arraylength(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0xbe))
    }

    pub fn instr_astore(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_store(0x3a, 0x4b, index, false)
    }

    pub fn instr_athrow(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        // note: operand stack may be freed from other entries if the exception do not get handled
        self.stack_update(1, 1).and(self.push_instr(0xbf))
    }

    pub fn instr_baload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x33))
    }

    pub fn instr_bastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x54))
    }

    pub fn instr_bipush(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x10))
    }

    pub fn instr_caload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x34))
    }

    pub fn instr_castore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x55))
    }

    pub fn instr_checkcast(
        &mut self,
        target_type: ConstPoolIndex<ConstClassInfo>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0)
            .and(self.push_instr(0xc0))
            .and(self.push_ops(&target_type.as_bytes()))
    }

    pub fn instr_d2f(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x90))
    }

    pub fn instr_d2i(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x8e))
    }

    pub fn instr_d2l(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x8f))
    }

    pub fn instr_dadd(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x63))
    }

    pub fn instr_daload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x31))
    }

    pub fn instr_dastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 0).and(self.push_instr(0x52))
    }

    pub fn instr_dcmpg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 1).and(self.push_instr(0x98))
    }

    pub fn instr_dcmpl(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 1).and(self.push_instr(0x97))
    }

    pub fn instr_dconst_0(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 2).and(self.push_instr(0xe))
    }

    pub fn instr_dconst_1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 2).and(self.push_instr(0xf))
    }

    pub fn instr_ddiv(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x6f))
    }

    pub fn instr_dload(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_load(0x18, 0x26, index, true)
    }

    pub fn instr_dmul(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x6b))
    }

    pub fn instr_dneg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x77))
    }

    pub fn instr_drem(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x73))
    }

    pub fn instr_dreturn(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0).and(self.push_instr(0xaf))
    }

    pub fn instr_dstore(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_store(0x39, 0x47, index, true)
    }

    pub fn instr_dsub(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x67))
    }

    pub fn instr_dup(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 2).and(self.push_instr(0x59))
    }

    pub fn instr_dup_x1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 3).and(self.push_instr(0x5a))
    }

    pub fn instr_dup_x2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        // note: while specification mentions both 3 -> 4 and 2 -> 3
        // the latter just has longs/doubles on both sides making it equal to the former
        self.stack_update(3, 4).and(self.push_instr(0x5b))
    }

    pub fn instr_dup2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 4).and(self.push_instr(0x5c))
    }

    pub fn instr_dup2_x1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        // note: while specification mentions both 3 -> 5 and 2 -> 3
        // the latter just has longs/doubles on both sides making it equal to the former
        self.stack_update(3, 5).and(self.push_instr(0x5d))
    }

    pub fn instr_dup2_x2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        // note: while specification mentions multiple variants
        // the real stack diagram is 4 -> 6 as others are just using longs/doubles
        self.stack_update(4, 6).and(self.push_instr(0x5d))
    }

    pub fn instr_f2d(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 2).and(self.push_instr(0x8d))
    }

    pub fn instr_f2i(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x8b))
    }

    pub fn instr_f2l(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 2).and(self.push_instr(0x8c))
    }

    pub fn instr_fadd(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x62))
    }

    pub fn instr_faload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x30))
    }

    pub fn instr_fastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x51))
    }

    pub fn instr_fcmpg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x96))
    }

    pub fn instr_fcmpl(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x95))
    }

    pub fn instr_fconst_0(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0xb))
    }

    pub fn instr_fconst_1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0xc))
    }

    pub fn instr_fconst_2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0xd))
    }

    pub fn instr_fdiv(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x6e))
    }

    pub fn instr_fload(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_load(0x17, 0x22, index, false)
    }

    pub fn instr_fmul(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x6a))
    }

    pub fn instr_fneg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x76))
    }

    pub fn instr_frem(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x72))
    }

    pub fn instr_fstore(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_store(0x38, 0x43, index, false)
    }

    pub fn instr_freturn(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0xae))
    }

    pub fn instr_fsub(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x66))
    }

    pub fn instr_getfield(
        &mut self,
        field: ConstPoolIndex<ConstFieldRefInfo>,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, Self::slot_size(fat))
            .and(self.push_instr(0xb4))
            .and(self.push_ops(&field.as_bytes()))
    }

    pub fn instr_getstatic(
        &mut self,
        field: ConstPoolIndex<ConstFieldRefInfo>,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, Self::slot_size(fat))
            .and(self.push_instr(0xb2))
            .and(self.push_ops(&field.as_bytes()))
    }

    pub fn instr_goto(
        &mut self,
        offset: BytecodeOffset,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        if offset <= u16::MAX as u32 {
            // goto
            self.stack_update(1, 1)
                .and(self.push_instr(0xa7))
                .and(self.push_ops(&(offset as u16).to_be_bytes()))
        } else {
            // goto_w
            self.stack_update(1, 1)
                .and(self.push_instr(0xc8))
                .and(self.push_ops(&offset.to_be_bytes()))
        }
    }

    pub fn instr_i2b(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x91))
    }

    pub fn instr_i2c(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x92))
    }

    pub fn instr_i2d(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 2).and(self.push_instr(0x87))
    }

    pub fn instr_i2f(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x86))
    }

    pub fn instr_i2l(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 2).and(self.push_instr(0x85))
    }

    pub fn instr_i2s(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x93))
    }

    pub fn instr_iadd(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x60))
    }

    pub fn instr_iaload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x2e))
    }

    pub fn instr_iand(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x7e))
    }

    pub fn instr_iastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x4f))
    }

    // TODO general-purpose Xconst method using const pool

    pub fn instr_iconst_m1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x2))
    }

    pub fn instr_iconst_0(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x3))
    }

    pub fn instr_iconst_1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x4))
    }

    pub fn instr_iconst_2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x5))
    }

    pub fn instr_iconst_3(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x6))
    }

    pub fn instr_iconst_4(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x7))
    }

    pub fn instr_iconst_5(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1).and(self.push_instr(0x8))
    }

    pub fn instr_idiv(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x7e))
    }

    pub fn instr_if_acmpeq(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa5))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_acmpne(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa6))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmpeq(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0x9f))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmpne(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa0))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmplt(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa1))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmpge(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa2))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmpgt(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa3))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_if_icmple(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0)
            .and(self.push_instr(0xa4))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifeq(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x99))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifne(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x9a))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_iflt(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x9b))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifge(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x9c))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifgt(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x9d))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifle(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0x9e))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifnonnull(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0xc7))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_ifnull(
        &mut self,
        branch: u16, /* not sure why wide is not used here */
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0)
            .and(self.push_instr(0xc6))
            .and(self.push_ops(&branch.to_be_bytes()))
    }

    pub fn instr_iinc(
        &mut self,
        local_variable_index: u16,
        delta: i16,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        if delta >= i8::MIN as i16
            && delta <= i8::MAX as i16
            && local_variable_index <= u8::MAX as u16
        {
            self.push_instr(0x84)
                .and(self.push_ops(&[local_variable_index as u8, delta as u8]))
        } else {
            self.push_instr(0xc4)
                .and(self.push_instr(0x84))
                .and(self.push_ops(&local_variable_index.to_be_bytes()))
                .and(self.push_ops(&delta.to_be_bytes()))
        }
    }

    pub fn instr_iload(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_load(0x15, 0x1a, index, false)
    }

    pub fn instr_imul(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x68))
    }

    pub fn instr_ineg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1).and(self.push_instr(0x74))
    }

    pub fn instr_instanceof(
        &mut self,
        checked_type: ConstPoolIndex<ConstClassInfo>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 1)
            .and(self.push_instr(0xc1))
            .and(self.push_ops(&checked_type.to_be_bytes()))
    }

    pub fn instr_invokedynamic(
        &mut self,
        bootstrap_method: ConstPoolIndex<ConstInvokeDynamicInfo>,
        arguments_count: u8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(arguments_count as u16, 0)
            .and(self.push_instr(0xba))
            .and(self.push_ops(&bootstrap_method.to_be_bytes()))
            .and(self.push_ops(&Self::ZERO_BYTE_ARRAY_2))
    }

    pub fn instr_invokeinterface(
        &mut self,
        method: ConstPoolIndex<ConstInterfaceMethodRefInfo>,
        arguments_count: u8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        let arguments_count = arguments_count
            .checked_add(1)
            .ok_or(BytecodeUpdateError::TooMuchMethodParameters)?;
        self.stack_update(arguments_count as u16, 0)
            .and(self.push_instr(0xb9))
            .and(self.push_ops(&method.to_be_bytes()))
            .and(self.push_ops(&[arguments_count, 0]))
    }

    // TODO AnyMethodRefInfo or similar to this and similar methods
    pub fn instr_invokespecial(
        &mut self,
        method: ConstPoolIndex<ConstInterfaceMethodRefInfo>,
        arguments_count: u8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(
            arguments_count
                .checked_add(1)
                .ok_or(BytecodeUpdateError::TooMuchMethodParameters)? as u16,
            0,
        )
        .and(self.push_instr(0xb7))
        .and(self.push_ops(&method.to_be_bytes()))
    }

    pub fn instr_invokestatic(
        &mut self,
        method: ConstPoolIndex<ConstInterfaceMethodRefInfo>,
        arguments_count: u8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(arguments_count as u16, 0)
            .and(self.push_instr(0xb8))
            .and(self.push_ops(&method.to_be_bytes()))
    }

    pub fn instr_invokevirtual(
        &mut self,
        method: ConstPoolIndex<ConstMethodRefInfo>,
        arguments_count: u8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        println!("1) self = {:?}", self);
        let arguments_count = arguments_count
            .checked_add(1)
            .ok_or(BytecodeUpdateError::TooMuchMethodParameters)?;
        println!("2) self = {:?}", self);
        self.stack_update(arguments_count as u16, 0)
            .and(self.push_instr(0xb6))
            .and(self.push_ops(&method.to_be_bytes()))
    }

    pub fn instr_ior(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x80))
    }

    pub fn instr_irem(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x70))
    }

    pub fn instr_ireturn(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0xac))
    }

    pub fn instr_ishl(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x78))
    }

    pub fn instr_istore(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_store(0x36, 0x3b, index, false)
    }

    pub fn instr_isub(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x64))
    }

    pub fn instr_iushr(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x7c))
    }

    pub fn instr_ixor(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x82))
    }

    pub fn instr_jsr(
        &mut self,
        offset: BytecodeOffset,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        if offset <= u16::MAX as u32 {
            // jsr
            self.stack_update(1, 1)
                .and(self.push_instr(0xa8))
                .and(self.push_ops(&(offset as u16).to_be_bytes()))
        } else {
            // jsr_w
            self.stack_update(1, 1)
                .and(self.push_instr(0xc9))
                .and(self.push_ops(&offset.to_be_bytes()))
        }
    }

    pub fn instr_l2d(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x8a))
    }

    pub fn instr_l2f(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x89))
    }

    pub fn instr_l2i(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x88))
    }

    pub fn instr_ladd(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x61))
    }

    pub fn instr_laload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x2f))
    }

    pub fn instr_land(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x7f))
    }

    pub fn instr_lastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 0).and(self.push_instr(0x50))
    }

    pub fn instr_lcmp(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 1).and(self.push_instr(0x94))
    }

    pub fn instr_lconst_0(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 2).and(self.push_instr(0x9))
    }

    pub fn instr_lconst_1(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 2).and(self.push_instr(0xa))
    }

    // TODO type-safety
    pub fn instr_ldc<E: ConstPoolEntryInfo>(
        &mut self,
        value: ConstPoolIndex<E>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1)
            .and(if value.as_u16() <= u8::MAX as u16 {
                self.push_instr(0x12) // ldc
                    .and(self.push_ops(&(value.as_u16() as u8).to_le_bytes()))
            } else {
                self.push_instr(0x13) // ldc_w
                    .and(self.push_ops(&value.to_le_bytes()))
            })
    }

    pub fn instr_ldc2_w<E: ConstPoolEntryInfo>(
        &mut self,
        value: ConstPoolIndex<E>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 2)
            .and(self.push_instr(0x14))
            .and(self.push_ops(&value.to_le_bytes()))
    }

    pub fn instr_ldiv(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x6d))
    }

    pub fn instr_lload(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_load(0x16, 0x1e, index, true)
    }

    pub fn instr_lmul(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x69))
    }

    pub fn instr_lneg(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x75))
    }

    // TODO instr_lookupswitch

    pub fn instr_lor(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x81))
    }

    pub fn instr_lrem(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x71))
    }

    pub fn instr_lreturn(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0).and(self.push_instr(0xad))
    }

    pub fn instr_lshl(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x79))
    }

    pub fn instr_lshr(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x7b))
    }

    pub fn instr_lstore(&mut self, index: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.instr_store(0x37, 0x3f, index, true)
    }

    pub fn instr_lsub(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x65))
    }

    pub fn instr_lushr(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x7d))
    }

    pub fn instr_lxor(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(4, 2).and(self.push_instr(0x83))
    }

    pub fn instr_monitorenter(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0xc2))
    }

    pub fn instr_monitorexit(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0xc2))
    }

    pub fn instr_multinewarray(
        &mut self,
        component_type: ConstPoolIndex<ConstClassInfo>,
        dimensions: NonZeroU8,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        let dimensions = dimensions.get();
        self.stack_update(dimensions as u16, 1)
            .and(self.push_instr(0x19))
            .and(self.push_ops(&component_type.as_bytes()))
            .and(self.push_ops(&[dimensions]))
    }

    pub fn instr_new(
        &mut self,
        component_type: ConstPoolIndex<ConstClassInfo>,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1)
            .and(self.push_instr(0xbb))
            .and(self.push_ops(&component_type.as_bytes()))
    }

    pub fn instr_newarray(
        &mut self,
        component_type: TypeDescriptor,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        let component_type = match component_type {
            TypeDescriptor::Boolean => 4,
            TypeDescriptor::Char => 5,
            TypeDescriptor::Float => 6,
            TypeDescriptor::Double => 7,
            TypeDescriptor::Byte => 8,
            TypeDescriptor::Short => 9,
            TypeDescriptor::Int => 10,
            TypeDescriptor::Long => 1,
            TypeDescriptor::Class(..) | TypeDescriptor::Array(..) => {
                return Err(BytecodeUpdateError::InvalidType);
            }
        };
        self.stack_update(0, 1)
            .and(self.push_instr(0xbc))
            .and(self.push_ops(&[component_type]))
    }

    pub fn instr_nop(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.push_instr(0x0)
    }

    pub fn instr_pop(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1, 0).and(self.push_instr(0x57))
    }

    pub fn instr_pop2(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 0).and(self.push_instr(0xb5))
    }

    pub fn instr_putfield(
        &mut self,
        field: ConstPoolIndex<ConstFieldRefInfo>,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(1 + Self::slot_size(fat), 0)
            .and(self.push_instr(0xb5))
            .and(self.push_ops(&field.as_bytes()))
    }

    pub fn instr_putstatic(
        &mut self,
        field: ConstPoolIndex<ConstFieldRefInfo>,
        fat: bool,
    ) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(Self::slot_size(fat), 0)
            .and(self.push_instr(0xb3))
            .and(self.push_ops(&field.as_bytes()))
    }

    pub fn instr_ret(&mut self, index: u8) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.push_instr(0xa9).and(self.push_ops(&[index]))
    }

    pub fn instr_return(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.push_instr(0xb1)
    }

    pub fn instr_saload(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 1).and(self.push_instr(0x35))
    }

    pub fn instr_sastore(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(3, 0).and(self.push_instr(0x56))
    }

    pub fn instr_sipush(&mut self, value: u16) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(0, 1)
            .and(self.push_instr(0x56))
            .and(self.push_ops(&value.to_be_bytes()))
    }

    pub fn instr_swap(&mut self) -> Result<BytecodeOffset, BytecodeUpdateError> {
        self.stack_update(2, 2).and(self.push_instr(0x5f))
    }

    // TODO instr_tableswitch
}

impl TryIntoNamedAttribute for Bytecode {
    fn try_into_named_attribute(
        self,
        const_pool: &mut ConstPool,
    ) -> Result<NamedAttribute, AttributeCreateError> {
        NamedAttribute::new_code_attribute(
            const_pool,
            self.max_stack,
            self.max_locals,
            self.code,
            self.exception_tables,
            self.attributes,
        )
    }
}

impl Attributable for Bytecode {
    fn add_attribute(&mut self, attribute: NamedAttribute) -> Result<(), AttributeAddError> {
        self.attributes.push(attribute)?;
        Ok(())
    }
}
