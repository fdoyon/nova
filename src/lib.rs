#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
#[doc(hidden)]
pub extern crate alloc;

#[macro_export]
/// Create a clonable newtype for a given type and name.
macro_rules! newtype {
    (@__impl $ty:path => $name:ident) => {
        impl core::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> $ty {
                self.0
            }
        }
    };
    (@__prefix $($tokens:tt)+) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, core::hash::Hash)]
        #[repr(transparent)]
        $($tokens)*;
    };
    ($ty:path => pub $name:ident) => {
        $crate::newtype!(@__prefix pub struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    };
    ($ty:path => pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype!(@__prefix pub($($vis)+) struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    };
    ($ty:path => $name:ident) => {
        $crate::newtype!(@__prefix struct $name($ty));
        $crate::newtype!(@__impl $ty => $name);
    }
}

#[macro_export]
/// Create a copiable newtype for a given type and name.
macro_rules! newtype_copy {
    (@__impl $ty:path => $name:ident) => {
        impl core::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> $ty {
                self.0
            }
        }
    };
    (@__prefix $($tokens:tt)+) => {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, core::hash::Hash)]
        #[repr(transparent)]
        $($tokens)*;
    };
    ($ty:path => pub $name:ident) => {
        $crate::newtype_copy!(@__prefix pub struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    };
    ($ty:path => pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(@__prefix pub($($vis)+) struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    };
    ($ty:path => $name:ident) => {
        $crate::newtype_copy!(@__prefix struct $name($ty));
        $crate::newtype_copy!(@__impl $ty => $name);
    }
}

#[cfg(feature = "uuid")]
#[macro_export]
/// Creates a [`uuid::Uuid`][::uuid::Uuid] newtype.
macro_rules! uuid {
    (pub $name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::uuid::Uuid => $name);
    };
}

#[macro_export]
/// Creates a `u8` newtype.
macro_rules! u8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u8 => $name);
    };
}

#[macro_export]
/// Creates a `u16` newtype.
macro_rules! u16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u16 => $name);
    };
}

#[macro_export]
/// Creates a `u32` newtype.
macro_rules! u32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u32 => $name);
    };
}

#[macro_export]
/// Creates a `u64` newtype.
macro_rules! u64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u64 => $name);
    };
}

#[macro_export]
/// Creates a `u128` newtype.
macro_rules! u128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(u128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(u128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(u128 => $name);
    };
}

#[macro_export]
/// Creates a `usize` newtype.
macro_rules! usize {
    (pub $name:ident) => {
        $crate::newtype_copy!(usize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(usize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(usize => $name);
    };
}

#[macro_export]
/// Creates an `i8` newtype.
macro_rules! i8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i8 => $name);
    };
}

#[macro_export]
/// Creates an `i16` newtype.
macro_rules! i16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i16 => $name);
    };
}

#[macro_export]
/// Creates an `i32` newtype.
macro_rules! i32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i32 => $name);
    };
}

#[macro_export]
/// Creates an `i64` newtype.
macro_rules! i64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i64 => $name);
    };
}

#[macro_export]
/// Creates an `i128` newtype.
macro_rules! i128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(i128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(i128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(i128 => $name);
    };
}

#[macro_export]
/// Creates an `isize` newtype.
macro_rules! isize {
    (pub $name:ident) => {
        $crate::newtype_copy!(isize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(isize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(isize => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroU8` newtype.
macro_rules! nonzero_u8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU8 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroU16` newtype.
macro_rules! nonzero_u16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU16 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroU32` newtype.
macro_rules! nonzero_u32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU32 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroU64` newtype.
macro_rules! nonzero_u64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU64 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroU128` newtype.
macro_rules! nonzero_u128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroU128 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroUsize` newtype.
macro_rules! nonzero_usize {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroUsize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroUsize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroUsize => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroI8` newtype.
macro_rules! nonzero_i8 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI8 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI8 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI8 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroI16` newtype.
macro_rules! nonzero_i16 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI16 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI16 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI16 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroI32` newtype.
macro_rules! nonzero_i32 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI32 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI32 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI32 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroI64` newtype.
macro_rules! nonzero_i64 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI64 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI64 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI64 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroI128` newtype.
macro_rules! nonzero_i128 {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI128 => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI128 => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroI128 => $name);
    };
}

#[macro_export]
/// Creates a `NonZeroIsize` newtype.
macro_rules! nonzero_isize {
    (pub $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroIsize => pub $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroIsize => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype_copy!(::core::num::NonZeroIsize => $name);
    };
}

#[macro_export]
#[cfg(feature = "heapless")]
/// Creates a [`Vec<u8>`][Vec] newtype. With <span class="stab portability"><code>heapless</code></span>
/// feature enabled, creates a [`heapless::Vec<u8, N>`][::heapless::Vec] newtype.
///
/// ## Usage
///
/// With [`heapless::Vec`][::heapless::Vec], the size needs to be specified.
/// ```
/// nova::bytevec!(MyStringType, 128);
/// ```
///
/// With an ordinary vector, no requirement for a size.
/// ```
/// nova::bytevec!(MyStringType);
/// ```
macro_rules! bytevec {
    (pub $name:ident, $n:tt) => {
        $crate::newtype!(::heapless::Vec<u8, $n> => pub $name);
    };
    (pub($($vis:tt)+) $name:ident, $n:tt) => {
        $crate::newtype!(::heapless::Vec<u8, $n> => pub ($($vis)*) $name);
    };
    ($name:ident, $n:tt) => {
        $crate::newtype!(::heapless::Vec<u8, $n> => $name);
    };
}

#[macro_export]
#[cfg(all(feature = "alloc", not(feature = "heapless")))]
/// Creates a [`Vec<u8>`][Vec] newtype. With <span class="stab portability"><code>heapless</code></span>
/// feature enabled, creates a [`heapless::Vec<u8, N>`][::heapless::Vec] newtype.
///
/// ## Usage
///
/// With [`heapless::Vec`][::heapless::Vec], the size needs to be specified.
/// ```
/// nova::bytevec!(MyStringType, 128);
/// ```
///
/// With an ordinary vector, no requirement for a size.
/// ```
/// nova::bytevec!(MyStringType);
/// ```
macro_rules! bytevec {
    (pub $name:ident) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => pub $name);
    };
    (pub $name:ident, $n:tt) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => pub $name);
    };
    (pub($($vis:tt)+) $name:ident) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => pub ($($vis)*) $name);
    };
    (pub($($vis:tt)+) $name:ident, $n:tt) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => pub ($($vis)*) $name);
    };
    ($name:ident) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => $name);
    };
    ($name:ident, $n:tt) => {
        $crate::newtype!($crate::alloc::vec::Vec<u8> => $name);
    };
}
#[macro_export]
#[cfg(feature = "heapless")]
/// Creates a [`String`][String] newtype. With <span class="stab portability"><code>heapless</code></span>
/// feature enabled, creates a [`heapless::String<N>`][::heapless::String] newtype.
///
/// ## Usage
///
/// With [`heapless::String`][::heapless::String], the size needs to be specified.
/// ```
/// nova::string!(MyStringType, 128);
/// ```
///
/// With an ordinary string, no requirement for a size.
/// ```
/// nova::string!(MyStringType);
/// ```
macro_rules! string {
    (@__impl $name:ident, $n:tt) => {
        impl core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> ::heapless::String<$n> {
                self.0
            }
        }
    };
    (pub $name:ident, $n:tt) => {
        $crate::newtype!(@__prefix pub struct $name(::heapless::String<$n>));
        $crate::string!(@__impl $name, $n);
    };
    (pub ($($vis:tt)+) $name:ident, $n:tt) => {
        $crate::newtype!(@__prefix pub($($vis)+) struct $name(::heapless::String<$n>));
        $crate::string!(@__impl $name, $n);
    };
    ($name:ident, $n:tt) => {
        $crate::newtype!(@__prefix struct $name(::heapless::String<$n>));
        $crate::string!(@__impl $name, $n);
    }
}

#[macro_export]
#[cfg(all(feature = "alloc", not(feature = "heapless")))]
/// Creates a [`String`][String] newtype. With <span class="stab portability"><code>heapless</code></span>
/// feature enabled, creates a [`heapless::String<N>`][::heapless::String] newtype.
///
/// ## Usage
///
/// With [`heapless::String`][::heapless::String], the size needs to be specified.
/// ```
/// nova::string!(MyStringType, 128);
/// ```
///
/// With an ordinary string, no requirement for a size.
/// ```
/// nova::string!(MyStringType);
/// ```
macro_rules! string {
    (@__impl $name:ident) => {
        impl core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn into_inner(self) -> $crate::alloc::string::String {
                self.0
            }
        }
    };
    (pub $name:ident) => {
        $crate::newtype!(@__prefix pub struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    };
    (pub ($($vis:tt)+) $name:ident) => {
        $crate::newtype!(@__prefix pub($($vis)+) struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    };
    ($name:ident) => {
        $crate::newtype!(@__prefix struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    };
    (pub $name:ident, $n:tt) => {
        $crate::newtype!(@__prefix pub struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    };
    (pub ($($vis:tt)+) $name:ident, $n:tt) => {
        $crate::newtype!(@__prefix pub($($vis)+) struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    };
    ($name:ident, $n:tt) => {
        $crate::newtype!(@__prefix struct $name($crate::alloc::string::String));
        $crate::string!(@__impl $name);
    }
}

#[cfg(test)]
mod test {
    use crate as nova;

    #[test]
    #[cfg(feature = "heapless")]
    fn heapless_bytevec() {
        nova::bytevec!(TestVec, 40);
    }

    #[test]
    #[cfg(all(feature = "alloc", not(feature = "heapless")))]
    fn create_string() {
        nova::string!(pub(crate) QuietScreaming);

        QuietScreaming("wow".into());
    }

    #[test]
    #[cfg(feature = "heapless")]
    fn create_string() {
        nova::string!(pub(crate) QuietScreaming, 20);

        QuietScreaming("wow".into());
    }

    #[test]
    fn newtype_u32() {
        nova::newtype_copy!(u32 => MyU32);

        MyU32(32);
    }

    #[test]
    fn newtypes() {
        nova::u8!(pub(crate) A);
        nova::u16!(pub B);
        nova::u32!(pub C);
        nova::u64!(pub D);
        nova::u128!(pub E);
        nova::usize!(pub F);
        nova::i8!(pub A2);
        nova::i16!(pub B2);
        nova::i32!(pub C2);
        nova::i64!(pub D2);
        nova::i128!(pub E2);
        nova::isize!(pub F2);

        #[cfg(all(feature = "alloc", not(feature = "heapless")))]
        nova::string!(pub G);

        #[cfg(feature = "heapless")]
        nova::string!(pub G, 42);

        #[cfg(all(feature = "alloc", not(feature = "heapless")))]
        nova::bytevec!(pub(in super) H);

        #[cfg(feature = "heapless")]
        nova::bytevec!(pub(in super) H, 42);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn create_uuid() {
        nova::uuid!(pub QuietScreaming);

        QuietScreaming(uuid::Uuid::new_v4());
    }
}
