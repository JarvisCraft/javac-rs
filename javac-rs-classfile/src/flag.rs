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

// TODO: add docs, preserve meta
#[macro_export]
macro_rules! mask_flags {
    ($visibility:vis $flag_name:ident as $number:ty=$default:expr, $flags_name:ident => {$(
        $key:ident=$value:expr,
    )*}) => {
        #[derive(PartialEq, Eq, Debug)]
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

        #[derive(PartialEq, Eq, Debug)]
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
    };
}
