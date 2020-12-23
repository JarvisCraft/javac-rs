//! Constant pool structures as specified by
//! [#4.4](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.4)

use crate::classfile_writable;

use crate::class::Tagged;
use crate::constpool::ConstPoolEntry::Empty;
use crate::vec::{JvmVecCreateError, JvmVecU2};
use std::convert::{TryFrom, TryInto};
use std::marker::PhantomData;
use std::ops::Deref;
use thiserror::Error;
use std::io::Write;
use crate::writer::ClassfileWritable;

#[derive(Error, Debug)]
pub enum ConstPoolStoreError {
    #[error("Const pool is out of space")]
    OutOfSpace,
    #[error("This const pool reference is already associated with an index")]
    AlreadyStored,
    #[error("Vector's value is too big")]
    VecValueTooBig(#[from] JvmVecCreateError),
}

/// Typeless index in const pool
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct RawConstPoolIndex(u16);

impl RawConstPoolIndex {
    fn as_typed<T: ConstPoolEntryInfo>(&self) -> ConstPoolIndex<T> {
        ConstPoolIndex(self.clone(), PhantomData)
    }
}

// Allow usage of index as simple `u16`
impl Deref for RawConstPoolIndex {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<usize> for RawConstPoolIndex {
    fn into(self) -> usize {
        self.0 as usize
    }
}

// `typed -> typeless` conversion
impl<T: ConstPoolEntryInfo> From<ConstPoolIndex<T>> for RawConstPoolIndex {
    fn from(typed: ConstPoolIndex<T>) -> Self {
        typed.as_raw()
    }
}

// Safe conversion as index is just a wrapper around `u16`
impl From<u16> for RawConstPoolIndex {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

// Unsafe conversion as `usize` value may not fit into wrapped `u16`
impl TryFrom<usize> for RawConstPoolIndex {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        u16::try_from(value)
            .map(|value| RawConstPoolIndex(value))
            .map_err(|_| ())
    }
}

// Simply write wrapped numeric index
impl ClassfileWritable for RawConstPoolIndex {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.0.write_to_classfile(buffer);
    }
}

/// Typed index of a structure in const pool
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ConstPoolIndex<T: ConstPoolEntryInfo>(RawConstPoolIndex, PhantomData<T>);

impl<T: ConstPoolEntryInfo> ConstPoolIndex<T> {
    /// Creates a [`RawConstPoolIndex`] from this one.
    fn as_raw(&self) -> RawConstPoolIndex {
        self.0
    }
}

// Allow usage of index as its raw equivalent
impl<T: ConstPoolEntryInfo> Deref for ConstPoolIndex<T> {
    type Target = RawConstPoolIndex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// `typeless -> typed` conversion
impl<T: ConstPoolEntryInfo> From<RawConstPoolIndex> for ConstPoolIndex<T> {
    fn from(raw: RawConstPoolIndex) -> Self {
        raw.as_typed()
    }
}

// Safe conversion as index is just a wrapper around `u16`
impl<T: ConstPoolEntryInfo> From<u16> for ConstPoolIndex<T> {
    fn from(value: u16) -> Self {
        Self(value.into(), PhantomData)
    }
}

// Unsafe conversion as `usize` value may not fit into wrapped `u16`
impl<T: ConstPoolEntryInfo> TryFrom<usize> for ConstPoolIndex<T> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        RawConstPoolIndex::try_from(value).map(|value| value.into())
    }
}

// Simple use internal write implementation of raw index
// as the index itself does not preserve any type information
impl<T: ConstPoolEntryInfo> ClassfileWritable for ConstPoolIndex<T> {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.0.write_to_classfile(buffer);
    }
}

/// Constant pool (aka const pool) of Java class as specified by
/// [#4.4](https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.4).
#[derive(Eq, PartialEq, Debug)]
pub struct ConstPool {
    /// Entries of this const pool
    entries: JvmVecU2<ConstPoolEntry>,
}

/* OLD
trait ConstPoolEntryInfoStorage<T: ConstPoolEntryInfo> {
    fn store_entry(&mut self, entry: ConstPoolEntry) -> Option<ConstPoolIndex<T>> {
        self.index_of(&entry)
            .or_else(move || self.force_store(entry))
    }
}
 */

impl ConstPool {
    pub fn new() -> Self {
        let mut entries = JvmVecU2::<ConstPoolEntry>::new();
        entries.push(ConstPoolEntry::Empty);
        ConstPool { entries }
    }

    /// Gets the index of the specified raw const pool entry if it is present in this const pool.
    ///
    /// # Arguments
    ///
    /// * `entry` - entry whose index should be resolved
    fn raw_index_of(&self, entry: &ConstPoolEntry) -> Option<RawConstPoolIndex> {
        self.entries
            .iter()
            .position(|checked_entry| checked_entry == entry)
            .and_then(|index| RawConstPoolIndex::try_from(index).ok())
    }

    /// Gets the index of the specified const pool entry if it is present in the const pool.
    ///
    /// # Arguments
    ///
    /// * `entry` - entry whose index should be resolved
    pub fn index_of<T: ConstPoolEntryInfo>(
        &self,
        entry: &ConstPoolEntry,
    ) -> Option<ConstPoolIndex<T>> {
        self.raw_index_of(entry).map(|raw_index| raw_index.into())
    }

    /// Gets the entry at the given index if it is not of this const pool's bounds.
    ///
    /// # Arguments
    ///
    /// * `index` - index at which to try to get the entry
    fn entry_at<T: ConstPoolEntryInfo>(&self, index: ConstPoolIndex<T>) -> Option<&ConstPoolEntry> {
        self.entries.get(index.0.0)
    }

    /// Gets the size of this const pool as `u16` which corresponds to its VM-type.
    pub fn size(&self) -> u16 {
        self.entries.len()
    }

    fn force_store_raw(
        &mut self,
        entry: ConstPoolEntry,
    ) -> Result<RawConstPoolIndex, ConstPoolStoreError> {
        self.entries
            .push(entry)
            .map(|index| index.into())
            .map_err(|_| ConstPoolStoreError::OutOfSpace)
    }

    fn store_raw(
        &mut self,
        entry: ConstPoolEntry,
    ) -> Result<RawConstPoolIndex, ConstPoolStoreError> {
        match self.raw_index_of(&entry) {
            Some(index) => Ok(index),
            None => self.force_store_raw(entry),
        }
    }

    // FIXME check types of entries
    fn force_store<T: ConstPoolEntryInfo>(
        &mut self,
        entry: ConstPoolEntry,
    ) -> Result<ConstPoolIndex<T>, ConstPoolStoreError> {
        self.entries
            .push(entry)
            .map(|index| index.into())
            .map_err(|_| ConstPoolStoreError::OutOfSpace)
    }

    fn store<T: ConstPoolEntryInfo>(
        &mut self,
        entry: ConstPoolEntry,
    ) -> Result<ConstPoolIndex<T>, ConstPoolStoreError> {
        match self.index_of(&entry) {
            Some(index) => Ok(index),
            None => self.force_store(entry),
        }
    }

    // Specializations for entries' creation

    pub fn store_const_class_info(
        &mut self,
        name: String,
    ) -> Result<ConstPoolIndex<ConstClassInfo>, ConstPoolStoreError> {
        let name = self.store_const_utf8_info(name)?;
        self.store_entry_info(ConstClassInfo { name })
    }

    pub fn store_const_field_ref_info(
        &mut self,
        class_name: String,
        name: String,
        descriptor: String,
    ) -> Result<ConstPoolIndex<ConstFieldRefInfo>, ConstPoolStoreError> {
        let class = self.store_const_class_info(class_name)?;
        let name_and_type = self.store_const_name_and_type_info(name, descriptor)?;
        self.store_entry_info(ConstFieldRefInfo {
            class,
            name_and_type,
        })
    }

    pub fn store_const_method_ref_info(
        &mut self,
        class_name: String,
        name: String,
        descriptor: String,
    ) -> Result<ConstPoolIndex<ConstMethodRefInfo>, ConstPoolStoreError> {
        let class = self.store_const_class_info(class_name)?;
        let name_and_type = self.store_const_name_and_type_info(name, descriptor)?;
        self.store_entry_info(ConstMethodRefInfo {
            class,
            name_and_type,
        })
    }

    pub fn store_const_interface_method_ref_info(
        &mut self,
        class_name: String,
        name: String,
        descriptor: String,
    ) -> Result<ConstPoolIndex<ConstInterfaceMethodRefInfo>, ConstPoolStoreError> {
        let class = self.store_const_class_info(class_name)?;
        let name_and_type = self.store_const_name_and_type_info(name, descriptor)?;
        self.store_entry_info(ConstInterfaceMethodRefInfo {
            class,
            name_and_type,
        })
    }

    pub fn store_const_string_info(
        &mut self,
        value: String,
    ) -> Result<ConstPoolIndex<ConstStringInfo>, ConstPoolStoreError> {
        let value = self.store_const_utf8_info(value)?;
        self.store_entry_info(ConstStringInfo { value })
    }

    pub fn store_const_integer_info(
        &mut self,
        value: u32,
    ) -> Result<ConstPoolIndex<ConstIntegerInfo>, ConstPoolStoreError> {
        self.store_entry_info(ConstIntegerInfo { value })
    }

    pub fn store_const_float_info(
        &mut self,
        value: f32,
    ) -> Result<ConstPoolIndex<ConstFloatInfo>, ConstPoolStoreError> {
        self.store_entry_info(ConstFloatInfo::from(value))
    }

    pub fn store_const_long_info(
        &mut self,
        value: u64,
    ) -> Result<ConstPoolIndex<ConstLongInfo>, ConstPoolStoreError> {
        self.store_entry_info(ConstLongInfo { value })
    }

    pub fn store_const_double_info(
        &mut self,
        value: f64,
    ) -> Result<ConstPoolIndex<ConstDoubleInfo>, ConstPoolStoreError> {
        self.store_entry_info(ConstDoubleInfo::from(value))
    }

    pub fn store_const_name_and_type_info(
        &mut self,
        name: String,
        descriptor: String,
    ) -> Result<ConstPoolIndex<ConstNameAndTypeInfo>, ConstPoolStoreError> {
        let a = self.store_const_utf8_info(name)?;
        let b = self.store_const_utf8_info(descriptor)?;
        self.store_entry_info(ConstNameAndTypeInfo {
            name: a,
            descriptor: b,
        })
    }

    pub fn store_const_utf8_info(
        &mut self,
        value: String,
    ) -> Result<ConstPoolIndex<ConstUtf8Info>, ConstPoolStoreError> {
        self.store_entry_info(ConstUtf8Info {
            bytes: value.into_bytes().try_into()?,
        })
    }

    pub fn store_const_method_type_info(
        &mut self,
        descriptor: String,
    ) -> Result<ConstPoolIndex<ConstMethodTypeInfo>, ConstPoolStoreError> {
        let descriptor = self.store_const_utf8_info(descriptor)?;
        self.store_entry_info(ConstMethodTypeInfo { descriptor })
    }

    // TODO type-safe implementation for MethodHandle and InvokeDynamic elements

    // helpers

    pub fn store_const_value_info(
        &mut self,
        value: ConstValue,
    ) -> Result<ConstPoolIndex<ConstValueInfo>, ConstPoolStoreError> {
        match value {
            ConstValue::Integer(value) => self.store_const_integer_info(value).map(|index| index.as_typed()),
            ConstValue::Float(value) => self.store_const_float_info(value).map(|index| index.as_typed()),
            ConstValue::Long(value) => self.store_const_long_info(value).map(|index| index.as_typed()),
            ConstValue::Double(value) => self.store_const_double_info(value).map(|index| index.as_typed()),
            ConstValue::String(value) => self.store_const_string_info(value).map(|index| index.as_typed()),
        }
    }
}

impl Default for ConstPool {
    fn default() -> Self {
        Self::new()
    }
}

// Simply write internal limited Vec which corresponds to VM representation
impl ClassfileWritable for ConstPool {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        // JvmVecU2<_>::write_to_classfile() implementation cannot be used
        // as the specification requires slot 0 to be empty
        // TODO: check for bounds with respect to this limitation
        // (self.entries.len() + 1).write_to_classfile(buffer);
        // for element in self.entries.iter() { element.write_to_classfile(buffer); }
        self.entries.write_to_classfile(buffer);
    }
}

trait ConstPoolEntryInfoStorage<T: ConstPoolEntryInfo> {
    fn store_entry_info(&mut self, entry: T) -> Result<ConstPoolIndex<T>, ConstPoolStoreError>;
}

/// Typed entry of a [const pool](ConstPool)
#[derive(Eq, PartialEq, Debug)]
pub enum ConstPoolEntry {
    /// Empty value of a constant pool index.
    /// This should only be present at slot 0 as it does not have a tag.
    Empty,
    Class(ConstClassInfo),
    FieldRef(ConstFieldRefInfo),
    MethodRef(ConstMethodRefInfo),
    InterfaceMethodRef(ConstInterfaceMethodRefInfo),
    String(ConstStringInfo),
    Integer(ConstIntegerInfo),
    Float(ConstFloatInfo),
    Long(ConstLongInfo),
    Double(ConstDoubleInfo),
    NameAndType(ConstNameAndTypeInfo),
    Utf8(ConstUtf8Info),
    MethodHandle(ConstMethodHandleInfo),
    MethodType(ConstMethodTypeInfo),
    Dynamic(ConstDynamicInfo),
    InvokeDynamic(ConstInvokeDynamicInfo),
    Module(ConstModuleInfo),
    Package(ConstPackageInfo),
}

impl Tagged for ConstPoolEntry {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            // TODO find a better alternative
            Self::Empty => panic!("Empty ConstPoolEntry cannot have a tag"),
            Self::Class(..) => 7,
            Self::FieldRef(..) => 9,
            Self::MethodRef(..) => 10,
            Self::InterfaceMethodRef(..) => 11,
            Self::String(..) => 8,
            Self::Integer(..) => 3,
            Self::Float(..) => 4,
            Self::Long(..) => 5,
            Self::Double(..) => 6,
            Self::NameAndType(..) => 12,
            Self::Utf8(..) => 1,
            Self::MethodHandle(..) => 15,
            Self::MethodType(..) => 16,
            Self::Dynamic(..) => 17,
            Self::InvokeDynamic(..) => 18,
            Self::Module(..) => 19,
            Self::Package(..) => 20,
        }
    }
}

pub trait ConstPoolEntryInfo: PartialEq + Eq {
    /* fn is_valid_type(entry: &ConstPoolEntry) -> bool; */
}

macro_rules! impl_traits_for_const_pool_entry {
    ($($source_identifier:ident => $target_type:ty,)*) => {$(
        impl ConstPoolEntryInfoStorage<$target_type> for ConstPool {
            fn store_entry_info(&mut self, value: $target_type)
                                -> Result<ConstPoolIndex<$target_type>, ConstPoolStoreError> {
                self.store(ConstPoolEntry::$source_identifier(value))
            }
        }

        impl ::std::convert::TryInto<$target_type> for ConstPoolEntry {
            type Error = ();

            fn try_into(self) -> Result<$target_type, Self::Error> {
                if let Self::$source_identifier(info) = self{ Ok(info) } else { Err(()) }
            }
        }

        impl ::std::cmp::PartialEq<$target_type> for ConstPoolEntry {
            fn eq(&self, info: &$target_type) -> bool {
                if let Self::$source_identifier(self_info) = self { self_info == info }
                else { false }
            }
        }
    )*};
}
impl_traits_for_const_pool_entry! {
    Class => ConstClassInfo,
    FieldRef => ConstFieldRefInfo,
    MethodRef => ConstMethodRefInfo,
    InterfaceMethodRef => ConstInterfaceMethodRefInfo,
    String => ConstStringInfo,
    Integer => ConstIntegerInfo,
    Float => ConstFloatInfo,
    Long => ConstLongInfo,
    Double => ConstDoubleInfo,
    NameAndType => ConstNameAndTypeInfo,
    Utf8 => ConstUtf8Info,
    MethodHandle => ConstMethodHandleInfo,
    MethodType => ConstMethodTypeInfo,
    Dynamic => ConstDynamicInfo,
    InvokeDynamic => ConstInvokeDynamicInfo,
    Module => ConstModuleInfo,
    Package => ConstPackageInfo,
}

impl ClassfileWritable for ConstPoolEntry {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        if *self == Empty {
            return;
        }

        self.tag().write_to_classfile(buffer);
        match self {
            Self::Class(value) => value.write_to_classfile(buffer),
            Self::FieldRef(value) => value.write_to_classfile(buffer),
            Self::MethodRef(value) => value.write_to_classfile(buffer),
            Self::InterfaceMethodRef(value) => value.write_to_classfile(buffer),
            Self::String(value) => value.write_to_classfile(buffer),
            Self::Integer(value) => value.write_to_classfile(buffer),
            Self::Float(value) => value.write_to_classfile(buffer),
            Self::Long(value) => value.write_to_classfile(buffer),
            Self::Double(value) => value.write_to_classfile(buffer),
            Self::NameAndType(value) => value.write_to_classfile(buffer),
            Self::Utf8(value) => value.write_to_classfile(buffer),
            Self::MethodHandle(value) => value.write_to_classfile(buffer),
            Self::MethodType(value) => value.write_to_classfile(buffer),
            Self::Dynamic(value) => value.write_to_classfile(buffer),
            Self::InvokeDynamic(value) => value.write_to_classfile(buffer),
            Self::Module(value) => value.write_to_classfile(buffer),
            Self::Package(value) => value.write_to_classfile(buffer),
            Self::Empty => unsafe { ::std::hint::unreachable_unchecked() },
        }
    }
}

/* enum LoadableConstPoolEntry {
    Integer(ConstPoolEntry),
    Float(ConstPoolEntry),
    Long(ConstPoolEntry),
    Double(ConstPoolEntry),
    Class(ConstPoolEntry),
    String(ConstPoolEntry),
    MethodHandle(ConstPoolEntry),
    MethodType(ConstPoolEntry),
    Dynamic(ConstPoolEntry),
}

impl Deref for LoadableConstPoolEntry {
    type Target = ConstPoolEntry;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Integer(entry) => { entry }
            Self::Float(entry) => { entry }
            Self::Long(entry) => { entry }
            Self::Double(entry) => { entry }
            Self::Class(entry) => { entry }
            Self::String(entry) => { entry }
            Self::MethodHandle(entry) => { entry }
            Self::MethodType(entry) => { entry }
            Self::Dynamic(entry) => { entry }
        }
    }
}*/

#[derive(Eq, PartialEq, Debug)]
pub enum LoadableConstPoolEntryInfo {
    Integer(ConstIntegerInfo),
    Float(ConstFloatInfo),
    Long(ConstLongInfo),
    Double(ConstDoubleInfo),
    Class(ConstClassInfo),
    String(ConstStringInfo),
    MethodHandle(ConstMethodHandleInfo),
    MethodType(ConstMethodTypeInfo),
    Dynamic(ConstDynamicInfo),
}

macro_rules! impl_try_into_for_loadable_const_pool_entry_info {
    ($($source_identifier:ident => $target_type:ty,)*) => {$(
        impl TryInto<$target_type> for LoadableConstPoolEntryInfo {
            type Error = ();

            fn try_into(self) -> Result<$target_type, Self::Error> {
                if let Self::$source_identifier(info) = self { Ok(info) } else { Err(()) }
            }
        }
    )*};
}
impl_try_into_for_loadable_const_pool_entry_info! {
    Integer => ConstIntegerInfo,
    Float => ConstFloatInfo,
    Long => ConstLongInfo,
    Double => ConstDoubleInfo,
    Class => ConstClassInfo,
    String => ConstStringInfo,
    MethodHandle => ConstMethodHandleInfo,
    MethodType => ConstMethodTypeInfo,
    Dynamic => ConstDynamicInfo,
}
impl ClassfileWritable for LoadableConstPoolEntryInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        match self {
            Self::Integer(info) => info.write_to_classfile(buffer),
            Self::Float(info) => info.write_to_classfile(buffer),
            Self::Long(info) => info.write_to_classfile(buffer),
            Self::Double(info) => info.write_to_classfile(buffer),
            Self::Class(info) => info.write_to_classfile(buffer),
            Self::String(info) => info.write_to_classfile(buffer),
            Self::MethodHandle(info) => info.write_to_classfile(buffer),
            Self::MethodType(info) => info.write_to_classfile(buffer),
            Self::Dynamic(info) => info.write_to_classfile(buffer),
        }
    }
}

impl ConstPoolEntryInfo for LoadableConstPoolEntryInfo {}

/* FIXME
impl TryInto<ConstStringInfo> for LoadableConstPoolEntry {
    type Error = ();

    fn try_into(self) -> Result<ConstStringInfo, Self::Error> {
        if matches!(self, Self::String(_)) self.
    }
}

macro_rules! impl_try_into_for_loadable_const_pool_entry {
    ($($name:ty)*) => {$(
        impl TryInto
    )*};
}
*/

macro_rules! impl_const_pool_entry_info {
    (
        $entry_matcher:pat,
        $(#$struct_attribute:tt)*
        $struct_visibility:vis struct $struct_name:ident{$(
            $(#$field_attribute:tt)*
            $field_visibility:vis $field:ident: $type:ty
        ),*$(,)?}
    ) => {
        classfile_writable!(
            $(#$struct_attribute)*
            $struct_visibility struct $struct_name{$(
                $(#$field_attribute)*
                $field_visibility $field: $type,
            )*}
        );

        impl ConstPoolEntryInfo for $struct_name {
            /*
            fn is_valid_type(entry: &ConstPoolEntry) -> bool { matches!(entry, $entry_matcher) }
            */
        }
    };
    (
        $entry_matcher:pat,
        $(#$struct_attribute:tt)*
        $struct_visibility:vis struct $struct_name:ident;
    ) => {
        classfile_writable!(
            $(#$struct_attribute)*
            $struct_visibility struct $struct_name;
        );
        impl ConstPoolEntryInfo for $struct_name {
            /*
            fn is_valid_type(entry: &ConstPoolEntry) -> bool { matches!(entry, $entry_matcher) }
            */
        }
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Class(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstClassInfo {
        name: ConstPoolIndex<ConstUtf8Info>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::FieldRef(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstFieldRefInfo {
        class: ConstPoolIndex<ConstClassInfo>,
        name_and_type: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::MethodRef(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstMethodRefInfo {
        class: ConstPoolIndex<ConstClassInfo>,
        name_and_type: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::InterfaceMethodRef(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstInterfaceMethodRefInfo {
        class: ConstPoolIndex<ConstClassInfo>,
        name_and_type: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::String(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstStringInfo {
        value: ConstPoolIndex<ConstUtf8Info>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Integer(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstIntegerInfo { value: u32 }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Float(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstFloatInfo { value: u32 }
}

impl ConstFloatInfo {
    pub fn from(value: f32) -> Self { Self { value: value.to_bits() } }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Long(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstLongInfo { value: u64 }
}

impl ConstLongInfo {
    pub fn high(&self) -> u32 {
        (self.value >> 32) as u32
    }

    pub fn low(&self) -> u32 {
        (self.value & 0xFFFF) as u32
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Double(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstDoubleInfo { value: u64 }
}

impl ConstDoubleInfo {
    pub fn from(value: f64) -> Self { Self { value: value.to_bits() } }

    pub fn high(&self) -> u32 {
        (self.value >> 32) as u32
    }

    pub fn low(&self) -> u32 {
        (self.value & 0xFFFF) as u32
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::NameAndType(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstNameAndTypeInfo {
        name: ConstPoolIndex<ConstUtf8Info>,
        descriptor: ConstPoolIndex<ConstUtf8Info>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Utf8(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstUtf8Info {
        bytes: JvmVecU2<u8>,
    }
}

/// Either [`ConstMethodRefInfo`] or [`ConstInterfaceMethodRefInfo`].
#[derive(Eq, PartialEq, Debug)]
pub enum AnyMethodRefInfo {
    Method(ConstMethodRefInfo),
    InterfaceMethod(ConstInterfaceMethodRefInfo),
}

impl ConstPoolEntryInfo for AnyMethodRefInfo {}

#[derive(Eq, PartialEq, Debug)]
pub enum ConstMethodHandleInfo {
    GetField(ConstPoolIndex<ConstFieldRefInfo>),
    GetStatic(ConstPoolIndex<ConstFieldRefInfo>),
    PutField(ConstPoolIndex<ConstFieldRefInfo>),
    PutStatic(ConstPoolIndex<ConstFieldRefInfo>),
    InvokeVirtual(ConstPoolIndex<ConstMethodRefInfo>),
    InvokeStatic(ConstPoolIndex<AnyMethodRefInfo>),
    InvokeSpecial(ConstPoolIndex<AnyMethodRefInfo>),
    NewInvokeSpecial(ConstPoolIndex<ConstMethodRefInfo>),
    InvokeInterface(ConstPoolIndex<ConstInterfaceMethodRefInfo>),
}

impl Tagged for ConstMethodHandleInfo {
    type TagType = u8;

    fn tag(&self) -> Self::TagType {
        match self {
            Self::GetField(..) => 1,
            Self::GetStatic(..) => 2,
            Self::PutField(..) => 3,
            Self::PutStatic(..) => 4,
            Self::InvokeVirtual(..) => 5,
            Self::InvokeStatic(..) => 6,
            Self::InvokeSpecial(..) => 7,
            Self::NewInvokeSpecial(..) => 8,
            Self::InvokeInterface(..) => 9,
        }
    }
}

impl ConstPoolEntryInfo for ConstMethodHandleInfo {
    /*
    fn is_valid_type(entry: &ConstPoolEntry) -> bool {
        matches!(entry, ConstPoolEntry::MethodHandle(..))
    }
    */
}

impl ClassfileWritable for ConstMethodHandleInfo {
    fn write_to_classfile<W: Write>(&self, buffer: &mut W) {
        self.tag().write_to_classfile(buffer);
        match self {
            Self::GetField(index) => {
                index.write_to_classfile(buffer);
            }
            Self::GetStatic(index) => {
                index.write_to_classfile(buffer);
            }
            Self::PutField(index) => {
                index.write_to_classfile(buffer);
            }
            Self::PutStatic(index) => {
                index.write_to_classfile(buffer);
            }
            Self::InvokeVirtual(index) => {
                index.write_to_classfile(buffer);
            }
            Self::InvokeStatic(index) => {
                index.write_to_classfile(buffer);
            }
            Self::InvokeSpecial(index) => {
                index.write_to_classfile(buffer);
            }
            Self::NewInvokeSpecial(index) => {
                index.write_to_classfile(buffer);
            }
            Self::InvokeInterface(index) => {
                index.write_to_classfile(buffer);
            }
        }
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::MethodType(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstMethodTypeInfo {
        descriptor: ConstPoolIndex<ConstUtf8Info>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Dynamic(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstDynamicInfo {
        bootstrap_method_attribute_index: u16,
        name_and_type: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::InvokeDynamic(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstInvokeDynamicInfo {
        bootstrap_method_attribute_index: u16,
        name_and_type: ConstPoolIndex<ConstNameAndTypeInfo>,
    }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Module(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstModuleInfo { name: ConstPoolIndex<ConstUtf8Info> }
}

impl_const_pool_entry_info! {
    ConstPoolEntry::Package(..),
    #[derive(Eq, PartialEq, Debug)]
    pub struct ConstPackageInfo { name: ConstPoolIndex<ConstUtf8Info> }
}

#[cfg(test)]
mod tests {
    use crate::constpool::{ConstIntegerInfo, ConstPool, ConstPoolEntryInfoStorage};

    #[test]
    fn test_const_pool_insert() {
        let mut const_pool = ConstPool::new();
        assert_eq!(
            const_pool.store_entry_info(ConstIntegerInfo { value: 123 }).unwrap().0.0, 0u16
        );
        assert_eq!(
            const_pool.store_entry_info(ConstIntegerInfo { value: 123 }).unwrap().0.0, 0u16
        );
        assert_eq!(
            const_pool.store_entry_info(ConstIntegerInfo { value: 456 }).unwrap().0.0, 1u16
        );
    }
}

pub enum ConstValue {
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    String(String),
}

/// Either [`ConstIntegerInfo`], [`ConstFloatInfo`], [`ConstLongInfo`],
/// [`ConstDoubleInfo`] or [`ConstStringInfo`].
#[derive(Eq, PartialEq, Debug)]
pub enum ConstValueInfo {
    Integer(ConstIntegerInfo),
    Float(ConstFloatInfo),
    Long(ConstLongInfo),
    Double(ConstDoubleInfo),
    String(ConstStringInfo),
}

impl ConstPoolEntryInfo for ConstValueInfo {}
