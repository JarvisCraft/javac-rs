//! Specific [`Vec`] facades to work with classfile-specific length limitations

use thiserror::Error;

// see: https://github.com/rust-lang/rust/issues/52607
macro_rules! with_doc {
    ($(
        #[doc = $content:expr]
        $target:item
    )*) => {$(
        #[doc = $content]
        $target
    )*};
}

#[derive(Error, Debug)]
pub enum JvmVecCreationError {
    #[error("Source value is too big")]
    SourceTooBig,
}

// TODO: add docs
// TODO: intra_rustdoc_links
macro_rules! impl_size_limited_vec {
    ($($name:ident($size_name:ident = $size_type:ident))*) => {$(
        with_doc! {
            #[doc = ::std::concat!(
                "Type representing size of [`", ::std::stringify!($name), "`]."
            )]
            pub type $size_name = $size_type;
        }

        with_doc! {
            #[doc = ::std::concat!(
                "Wrapper around [`Vec`] limited in its size to bounds of [`",
                ::std::stringify!($size_type), "`] aliased as [`",
                ::std::stringify!($size_name), "`]."
            )]

            #[derive(::std::cmp::Eq, ::std::cmp::PartialEq, ::std::fmt::Debug)]
            pub struct $name<T>(::std::vec::Vec<T>);
        }

        impl<T> $name<T> {
            #[doc = "Maximal size of this vector."]
            pub const MAX_SIZE: $size_type = $size_type::MAX;

            pub fn new() -> Self { Self(::std::vec![]) }

            pub fn iter(&self) -> ::std::slice::Iter<T> { self.0.iter() }

            pub fn get(&self, index: $size_type) -> ::std::option::Option<&T> {
                self.0.get(index as usize)
            }

            #[inline(always)]
            pub fn len(&self) -> $size_type { self.0.len() as $size_type }

            pub fn remaining_space(&self) -> $size_type {
                Self::MAX_SIZE - self.len()
            }

            pub fn has_space_for(&self, required_space: $size_type) -> bool {
                self.remaining_space() >= required_space
            }

            pub fn has_space(&self) -> bool { self.remaining_space() > 0 }

            pub fn push(&mut self, element: T) -> ::std::option::Option<$size_type> {
                let length = self.len();
                if length < Self::MAX_SIZE {
                    self.0.push(element);
                    Some(length)
                } else { None }
            }
        }

        impl<T> ::std::default::Default for $name<T> {
            fn default() -> Self { Self::new() }
        }

        impl<T> ::std::convert::TryFrom<Vec<T>> for $name<T> {
            type Error = $crate::vec::JvmVecCreationError;

            fn try_from(source: Vec<T>) -> Result<Self, Self::Error> {
                if source.len() > Self::MAX_SIZE as usize {
                    ::std::result::Result::Err($crate::vec::JvmVecCreationError::SourceTooBig)
                } else { ::std::result::Result::Ok(Self(source)) }
            }
        }

        impl<T: $crate::class::ClassfileWritable> $crate::class::ClassfileWritable for $name<T> {
            fn write_to_classfile<W: ::std::io::Write>(&self, buffer: &mut W) {
                self.len().write_to_classfile(buffer);
                for element in &self.0 { element.write_to_classfile(buffer); }
            }
        }
    )*};
}

impl_size_limited_vec! {
    JvmVecU1(JvmVecU1Size = u8)
    JvmVecU2(JvmVecU2Size = u16)
    JvmVecU4(JvmVecU4Size = u32)
}
