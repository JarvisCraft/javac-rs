use std::collections::hash_set;
use std::collections::HashSet;
use std::hash::Hash;

/// A simple data-container consisting of flags which may be set to `true` or `false`.
pub trait Flags<T> {
    /// Creates a new flags object with all flags set to `false`.
    fn none() -> Self;

    /// Sets the given flag to `true`.
    ///
    /// # Arguments
    ///
    /// * `flag` - flag to set to `true`
    fn set(&mut self, flag: T);

    /// Sets the given flag to `false`.
    ///
    /// # Arguments
    ///
    /// * `flag` - flag to set to `false`
    fn unset(&mut self, flag: &T);

    /// Checks the value of the given flag returning `true`
    /// if it is set to `true` and `false` otherwise.
    fn is_set(&self, flag: &T) -> bool;
}

/// Implementation of [flags](Flags) based on internal [hash-set](HashSet).
pub struct HashSetFlags<T: Eq + Hash>(HashSet<T>);

impl<T: Eq + Hash> HashSetFlags<T> {
    fn iter(&self) -> hash_set::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T: Eq + Hash> Flags<T> for HashSetFlags<T> {
    fn none() -> Self {
        Self(HashSet::new())
    }

    fn set(&mut self, flag: T) {
        self.0.insert(flag);
    }

    fn unset(&mut self, flag: &T) {
        self.0.remove(flag);
    }

    fn is_set(&self, flag: &T) -> bool {
        self.0.contains(flag)
    }
}

#[macro_export]
macro_rules! mask_flags {
    (
        $(#$flag_attribute:tt)*
        $visibility:vis $flag_name:ident as $number:ty=$default:expr;

        $(#$flags_attribute:tt)*
        $flags_name:ident => {$(
            $key:ident=$value:expr,
        )*}
    ) => {
        $(#$flag_attribute)*
        $visibility enum $flag_name {$(
            $key,
        )*}

        impl $flag_name {
            fn mask(&self) -> $number {
                match self {$(
                    Self::$key => { $value },
                )*}
            }
        }

        $(#$flags_attribute)*
        $visibility struct $flags_name($number);

        impl $flags_name {
            fn mask(&self) -> $number { self.0 }
        }

        impl $crate::flag::Flags<$flag_name> for $flags_name {
            fn none() -> Self { Self($default) }

            fn set(&mut self, flag: $flag_name) { self.0 |= flag.mask(); }

            fn unset(&mut self, flag: &$flag_name) { self.0 &= !flag.mask(); }

            fn is_set(&self, flag: &$flag_name) -> bool { self.0 & flag.mask() != $default }
        }

        impl ::std::ops::BitOr for $flag_name {
            type Output = $flags_name;

            fn bitor(self, rhs: Self) -> Self::Output {
                $flags_name(self.mask() | rhs.mask())
            }
        }

        impl ::std::ops::BitAnd for $flag_name {
            type Output = $flags_name;

            fn bitand(self, rhs: Self) -> Self::Output {
                $flags_name(self.mask() & rhs.mask())
            }
        }

        impl ::std::ops::BitOr<$flags_name> for $flag_name {
            type Output = $flags_name;

            fn bitor(self, rhs: $flags_name) -> Self::Output {
                $flags_name(self.mask() | rhs.mask())
            }
        }

        impl ::std::ops::BitAnd<$flags_name> for $flag_name {
            type Output = $flags_name;

            fn bitand(self, rhs: $flags_name) -> Self::Output {
                $flags_name(self.mask() & rhs.mask())
            }
        }

        impl ::std::ops::BitOr for $flags_name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                $flags_name(self.0 | rhs.0)
            }
        }

        impl ::std::ops::BitAnd for $flags_name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                $flags_name(self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitOr<$flag_name> for $flags_name {
            type Output = Self;

            fn bitor(self, rhs: $flag_name) -> Self::Output {
                $flags_name(self.0 | rhs.mask())
            }
        }

        impl ::std::ops::BitAnd<$flag_name> for $flags_name {
            type Output = Self;

            fn bitand(self, rhs: $flag_name) -> Self::Output {
                $flags_name(self.0 & rhs.mask())
            }
        }
    };
}
