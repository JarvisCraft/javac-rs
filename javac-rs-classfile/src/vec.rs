//! Specific Vec facades to make with classfile-specific length limitations

// TODO: add docs
macro_rules! impl_size_limited_vec {
    ($($name:ident($size_name:ident=$size_type:ident))*) => {$(
        pub type $size_name = $size_type;

        pub struct $name<T>(::std::vec::Vec<T>);

        impl<T> ::std::ops::Deref for $name<T> {
            type Target = ::std::vec::Vec<T>;

            fn deref(&self) -> &Self::Target { &self.0 }
        }
   )*};
}

impl_size_limited_vec! {
    JvmVecU2(JvmVecU2Size = u16)
    JvmVecU4(JvmVecU4Size = u32)
}
