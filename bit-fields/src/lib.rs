//! [`ExampleBitField`] is generated from:
//! ```ignore
//! #[rustfmt::skip]
//! bitfield!(ExampleBitField,u32,[
//!     RANGE1, 0..1,
//!     SSE, 2,
//!     SSE1, 3,
//!     RANGE2, 4..6,
//!     SSE2, 9,
//!     SSE3, 10,
//!     RANGE3, 12..15,
//!     SSE4, 18
//! ]);
//! ```
//! When `#![feature(const_mut_refs)]` is stabilized all functions under `impl Type { ... }` can
//! become `const fn`s.
#![warn(clippy::pedantic)]
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::marker::PhantomData;

pub use bit_fields_macros::*;

pub trait BitIndex<T, const P: u8> {
    fn bit(&self) -> &Bit<T, P>;
}
pub trait BitIndexMut<T, const P: u8> {
    fn bit_mut(&mut self) -> &mut Bit<T, P>;
}

/// A type interface for a range of bits.
#[derive(Debug, Clone, Copy)]
pub struct BitRange<T, const START: u8, const END: u8>(pub PhantomData<T>);

// Display impl
impl<const START: u8, const END: u8> fmt::Display for BitRange<u128, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u128 = self.into();
        write!(f, "{}", a)
    }
}
impl<const START: u8, const END: u8> fmt::Display for BitRange<u64, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u64 = self.into();
        write!(f, "{}", a)
    }
}
impl<const START: u8, const END: u8> fmt::Display for BitRange<u32, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u32 = self.into();
        write!(f, "{}", a)
    }
}
impl<const START: u8, const END: u8> fmt::Display for BitRange<u16, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u16 = self.into();
        write!(f, "{}", a)
    }
}
impl<const START: u8, const END: u8> fmt::Display for BitRange<u8, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u8 = self.into();
        write!(f, "{}", a)
    }
}

/// Checks add error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckedAddAssignErr {
    /// Operation would result in overflow of bit range.
    Overflow,
    /// Given value is more than maximum value storable in bit range.
    OutOfRange,
}
/// Checks add error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckedSubAssignErr {
    /// Operation would result in underflow of bit range.
    Underflow,
    /// Given value is more than maximum value storable in bit range.
    OutOfRange,
}
#[derive(Debug)]
pub struct CheckedAssignErr;
impl fmt::Display for CheckedAssignErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given value is greater than maximum storable value in bit range"
        )
    }
}

// Struct impl
impl<const START: u8, const END: u8> BitRange<u128, START, END> {
    const MASK: u128 = mask_u128(START, END);
    /// The maximum value this range can store plus one.
    const MAX: u128 = 2u128.pow((END - START) as u32);

    fn data(&self) -> *const u128 {
        let a = self as *const Self;
        a.cast::<u128>()
    }

    fn data_mut(&mut self) -> *mut u128 {
        let a = self as *mut Self;
        a.cast::<u128>()
    }

    /// Adds `x` to the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When adding `x` to the value of the bit range would overflow.
    pub fn checked_add_assign(&mut self, x: u128) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x < Self::MAX - cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() += shift;
                }
                Ok(())
            } else {
                Err(CheckedAddAssignErr::Overflow)
            }
        } else {
            Err(CheckedAddAssignErr::OutOfRange)
        }
    }

    /// Subtract `x` from the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When subtracting `x` from the value of the bit range would underflow.
    pub fn checked_sub_assign(&mut self, x: u128) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x <= cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() -= shift;
                }
                Ok(())
            } else {
                Err(CheckedSubAssignErr::Underflow)
            }
        } else {
            Err(CheckedSubAssignErr::OutOfRange)
        }
    }

    /// Sets the bit range returning `Err(())` when the given `x` is not storable in the range.
    ///
    /// # Errors
    ///
    /// When `x` is greater than the maximum storable value in `self`.
    pub fn checked_assign(&mut self, x: u128) -> Result<(), CheckedAssignErr> {
        if x < Self::MAX {
            let shift = x << START;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(CheckedAssignErr)
        }
    }
}
impl<const START: u8, const END: u8> BitRange<u64, START, END> {
    const MASK: u64 = mask_u64(START, END);
    /// The maximum value this range can store plus one.
    const MAX: u64 = 2u64.pow((END - START) as u32);

    fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a.cast::<u64>()
    }

    fn data_mut(&mut self) -> *mut u64 {
        let a = self as *mut Self;
        a.cast::<u64>()
    }

    /// Adds `x` to the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When adding `x` to the value of the bit range would overflow.
    pub fn checked_add_assign(&mut self, x: u64) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x < Self::MAX - cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() += shift;
                }
                Ok(())
            } else {
                Err(CheckedAddAssignErr::Overflow)
            }
        } else {
            Err(CheckedAddAssignErr::OutOfRange)
        }
    }

    /// Subtract `x` from the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When subtracting `x` from the value of the bit range would underflow.
    pub fn checked_sub_assign(&mut self, x: u64) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x <= cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() -= shift;
                }
                Ok(())
            } else {
                Err(CheckedSubAssignErr::Underflow)
            }
        } else {
            Err(CheckedSubAssignErr::OutOfRange)
        }
    }

    /// Sets the bit range returning `Err(())` when the given `x` is not storable in the range.
    ///
    /// # Errors
    ///
    /// When `x` is greater than the maximum storable value in `self`.
    pub fn checked_assign(&mut self, x: u64) -> Result<(), CheckedAssignErr> {
        if x < Self::MAX {
            let shift = x << START;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(CheckedAssignErr)
        }
    }
}
impl<const START: u8, const END: u8> BitRange<u32, START, END> {
    const MASK: u32 = mask_u32(START, END);
    /// The maximum value this range can store plus one.
    const MAX: u32 = 2u32.pow((END - START) as u32);

    fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a.cast::<u32>()
    }

    fn data_mut(&mut self) -> *mut u32 {
        let a = self as *mut Self;
        a.cast::<u32>()
    }

    /// Adds `x` to the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When adding `x` to the value of the bit range would overflow.
    pub fn checked_add_assign(&mut self, x: u32) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x < Self::MAX - cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() += shift;
                }
                Ok(())
            } else {
                Err(CheckedAddAssignErr::Overflow)
            }
        } else {
            Err(CheckedAddAssignErr::OutOfRange)
        }
    }

    /// Subtract `x` from the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When subtracting `x` from the value of the bit range would underflow.
    pub fn checked_sub_assign(&mut self, x: u32) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x <= cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() -= shift;
                }
                Ok(())
            } else {
                Err(CheckedSubAssignErr::Underflow)
            }
        } else {
            Err(CheckedSubAssignErr::OutOfRange)
        }
    }

    /// Sets the bit range returning `Err(())` when the given `x` is not storable in the range.
    ///
    /// # Errors
    ///
    /// When `x` is greater than the maximum storable value in `self`.
    pub fn checked_assign(&mut self, x: u32) -> Result<(), CheckedAssignErr> {
        if x < Self::MAX {
            let shift = x << START;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(CheckedAssignErr)
        }
    }
}
impl<const START: u8, const END: u8> BitRange<u16, START, END> {
    const MASK: u16 = mask_u16(START, END);
    /// The maximum value this range can store plus one.
    const MAX: u16 = 2u16.pow((END - START) as u32);

    fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a.cast::<u16>()
    }

    fn data_mut(&mut self) -> *mut u16 {
        let a = self as *mut Self;
        a.cast::<u16>()
    }

    /// Adds `x` to the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When adding `x` to the value of the bit range would overflow.
    pub fn checked_add_assign(&mut self, x: u16) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x < Self::MAX - cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() += shift;
                }
                Ok(())
            } else {
                Err(CheckedAddAssignErr::Overflow)
            }
        } else {
            Err(CheckedAddAssignErr::OutOfRange)
        }
    }

    /// Subtract `x` from the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When subtracting `x` from the value of the bit range would underflow.
    pub fn checked_sub_assign(&mut self, x: u16) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x <= cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() -= shift;
                }
                Ok(())
            } else {
                Err(CheckedSubAssignErr::Underflow)
            }
        } else {
            Err(CheckedSubAssignErr::OutOfRange)
        }
    }

    /// Sets the bit range returning `Err(())` when the given `x` is not storable in the range.
    ///
    /// # Errors
    ///
    /// When `x` is greater than the maximum storable value in `self`.
    pub fn checked_assign(&mut self, x: u16) -> Result<(), CheckedAssignErr> {
        if x < Self::MAX {
            let shift = x << START;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(CheckedAssignErr)
        }
    }
}
impl<const START: u8, const END: u8> BitRange<u8, START, END> {
    const MASK: u8 = mask_u8(START, END);
    /// The maximum value this range can store plus one.
    const MAX: u8 = 2u8.pow((END - START) as u32);

    fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a.cast::<u8>()
    }

    fn data_mut(&mut self) -> *mut u8 {
        let a = self as *mut Self;
        a.cast::<u8>()
    }

    /// Adds `x` to the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When adding `x` to the value of the bit range would overflow.
    pub fn checked_add_assign(&mut self, x: u8) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x < Self::MAX - cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() += shift;
                }
                Ok(())
            } else {
                Err(CheckedAddAssignErr::Overflow)
            }
        } else {
            Err(CheckedAddAssignErr::OutOfRange)
        }
    }

    /// Subtract `x` from the value of the bit range.
    ///
    /// # Errors
    ///
    /// 1. When `x` is greater than the maximum value storable in the bit range.
    /// 2. When subtracting `x` from the value of the bit range would underflow.
    pub fn checked_sub_assign(&mut self, x: u8) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> START;
            if x <= cur {
                let shift = x << START;
                unsafe {
                    *self.data_mut() -= shift;
                }
                Ok(())
            } else {
                Err(CheckedSubAssignErr::Underflow)
            }
        } else {
            Err(CheckedSubAssignErr::OutOfRange)
        }
    }

    /// Sets the value of the bit range to `x`.
    ///
    /// # Errors
    ///
    /// When `x` is greater than the maximum storable value in `self`.
    pub fn checked_assign(&mut self, x: u8) -> Result<(), CheckedAssignErr> {
        if x < Self::MAX {
            let shift = x << START;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(CheckedAssignErr)
        }
    }
}

// impl<const START: u8, const END: u8> AddAssign<u32> for BitRange<u32, START,END> {
//     fn add_assign(&mut self, x: u32) {
//         let a = x << START;
//         let b = a + unsafe { *self.data() };
//         unsafe { *self.data_mut() = b; }
//     }
// }

// Into<uint> impl
impl<const START: u8, const END: u8> From<&BitRange<u128, START, END>> for u128 {
    fn from(this: &BitRange<u128, START, END>) -> Self {
        let a = BitRange::<u128, START, END>::MASK & unsafe { *this.data() };
        a >> START
    }
}
impl<const START: u8, const END: u8> From<&BitRange<u64, START, END>> for u64 {
    fn from(this: &BitRange<u64, START, END>) -> Self {
        let a = BitRange::<u64, START, END>::MASK & unsafe { *this.data() };
        a >> START
    }
}
impl<const START: u8, const END: u8> From<&BitRange<u32, START, END>> for u32 {
    fn from(this: &BitRange<u32, START, END>) -> Self {
        let a = BitRange::<u32, START, END>::MASK & unsafe { *this.data() };
        a >> START
    }
}
impl<const START: u8, const END: u8> From<&BitRange<u16, START, END>> for u16 {
    fn from(this: &BitRange<u16, START, END>) -> Self {
        let a = BitRange::<u16, START, END>::MASK & unsafe { *this.data() };
        a >> START
    }
}
impl<const START: u8, const END: u8> From<&BitRange<u8, START, END>> for u8 {
    fn from(this: &BitRange<u8, START, END>) -> Self {
        let a = BitRange::<u8, START, END>::MASK & unsafe { *this.data() };
        a >> START
    }
}

// Eq impl

impl<const START: u8, const END: u8> PartialEq<u128> for BitRange<u128, START, END> {
    fn eq(&self, other: &u128) -> bool {
        let a = u128::from(self);
        a == *other
    }
}
impl<const START: u8, const END: u8> PartialEq for BitRange<u128, START, END> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (u128, u128) = (self.into(), other.into());
        a == b
    }
}
impl<const START: u8, const END: u8> PartialEq<u64> for BitRange<u64, START, END> {
    fn eq(&self, other: &u64) -> bool {
        let a = u64::from(self);
        a == *other
    }
}
impl<const START: u8, const END: u8> Eq for BitRange<u128, START, END> {}
impl<const START: u8, const END: u8> PartialEq for BitRange<u64, START, END> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (u64, u64) = (self.into(), other.into());
        a == b
    }
}
impl<const START: u8, const END: u8> PartialEq<u32> for BitRange<u32, START, END> {
    fn eq(&self, other: &u32) -> bool {
        let a = u32::from(self);
        a == *other
    }
}
impl<const START: u8, const END: u8> Eq for BitRange<u64, START, END> {}
impl<const START: u8, const END: u8> PartialEq for BitRange<u32, START, END> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (u32, u32) = (self.into(), other.into());
        a == b
    }
}
impl<const START: u8, const END: u8> PartialEq<u16> for BitRange<u16, START, END> {
    fn eq(&self, other: &u16) -> bool {
        let a = u16::from(self);
        a == *other
    }
}
impl<const START: u8, const END: u8> Eq for BitRange<u32, START, END> {}
impl<const START: u8, const END: u8> PartialEq for BitRange<u16, START, END> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (u16, u16) = (self.into(), other.into());
        a == b
    }
}
impl<const START: u8, const END: u8> PartialEq<u8> for BitRange<u8, START, END> {
    fn eq(&self, other: &u8) -> bool {
        let a = u8::from(self);
        a == *other
    }
}
impl<const START: u8, const END: u8> Eq for BitRange<u16, START, END> {}
impl<const START: u8, const END: u8> PartialEq for BitRange<u8, START, END> {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (u8, u8) = (self.into(), other.into());
        a == b
    }
}
impl<const START: u8, const END: u8> Eq for BitRange<u8, START, END> {}

// Ord impl
impl<const START: u8, const END: u8> PartialOrd<u128> for BitRange<u128, START, END> {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        let a = u128::from(self);
        Some(a.cmp(other))
    }
}
impl<const START: u8, const END: u8> PartialOrd for BitRange<u128, START, END> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b): (u128, u128) = (self.into(), other.into());
        Some(a.cmp(&b))
    }
}
impl<const START: u8, const END: u8> Ord for BitRange<u128, START, END> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<const START: u8, const END: u8> PartialOrd<u64> for BitRange<u64, START, END> {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        let a = u64::from(self);
        Some(a.cmp(other))
    }
}
impl<const START: u8, const END: u8> PartialOrd for BitRange<u64, START, END> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b): (u64, u64) = (self.into(), other.into());
        Some(a.cmp(&b))
    }
}
impl<const START: u8, const END: u8> Ord for BitRange<u64, START, END> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<const START: u8, const END: u8> PartialOrd<u32> for BitRange<u32, START, END> {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        let a = u32::from(self);
        Some(a.cmp(other))
    }
}
impl<const START: u8, const END: u8> PartialOrd for BitRange<u32, START, END> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b): (u32, u32) = (self.into(), other.into());
        Some(a.cmp(&b))
    }
}
impl<const START: u8, const END: u8> Ord for BitRange<u32, START, END> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<const START: u8, const END: u8> PartialOrd<u16> for BitRange<u16, START, END> {
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        let a = u16::from(self);
        Some(a.cmp(other))
    }
}
impl<const START: u8, const END: u8> PartialOrd for BitRange<u16, START, END> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b): (u16, u16) = (self.into(), other.into());
        Some(a.cmp(&b))
    }
}
impl<const START: u8, const END: u8> Ord for BitRange<u16, START, END> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<const START: u8, const END: u8> PartialOrd<u8> for BitRange<u8, START, END> {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        let a = u8::from(self);
        Some(a.cmp(other))
    }
}
impl<const START: u8, const END: u8> PartialOrd for BitRange<u8, START, END> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b): (u8, u8) = (self.into(), other.into());
        Some(a.cmp(&b))
    }
}
impl<const START: u8, const END: u8> Ord for BitRange<u8, START, END> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// A type interface for a single bit.
#[derive(Debug, Clone, Copy)]
pub struct Bit<T, const P: u8>(pub PhantomData<T>);

// Display impl
impl<const P: u8> fmt::Display for Bit<u128, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: u8> fmt::Display for Bit<u64, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: u8> fmt::Display for Bit<u32, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: u8> fmt::Display for Bit<u16, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: u8> fmt::Display for Bit<u8, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}

// Struct impl
impl<const P: u8> Bit<u128, P> {
    pub fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    fn data(&self) -> *const u128 {
        let a = self as *const Self;
        a.cast::<u128>()
    }

    fn data_mut(&mut self) -> *mut u128 {
        let a = self as *mut Self;
        a.cast::<u128>()
    }
}
impl<const P: u8> Bit<u64, P> {
    pub fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a.cast::<u64>()
    }

    fn data_mut(&mut self) -> *mut u64 {
        let a = self as *mut Self;
        a.cast::<u64>()
    }
}
impl<const P: u8> Bit<u32, P> {
    pub fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a.cast::<u32>()
    }

    fn data_mut(&mut self) -> *mut u32 {
        let a = self as *mut Self;
        a.cast::<u32>()
    }
}
impl<const P: u8> Bit<u16, P> {
    pub fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a.cast::<u16>()
    }

    fn data_mut(&mut self) -> *mut u16 {
        let a = self as *mut Self;
        a.cast::<u16>()
    }
}
impl<const P: u8> Bit<u8, P> {
    pub fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a.cast::<u8>()
    }

    fn data_mut(&mut self) -> *mut u8 {
        let a = self as *mut Self;
        a.cast::<u8>()
    }
}

impl<const P: u8> From<&Bit<u128, P>> for bool {
    fn from(this: &Bit<u128, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 == 1 }
    }
}
impl<const P: u8> From<&Bit<u64, P>> for bool {
    fn from(this: &Bit<u64, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 == 1 }
    }
}
impl<const P: u8> From<&Bit<u32, P>> for bool {
    fn from(this: &Bit<u32, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 == 1 }
    }
}
impl<const P: u8> From<&Bit<u16, P>> for bool {
    fn from(this: &Bit<u16, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 == 1 }
    }
}
impl<const P: u8> From<&Bit<u8, P>> for bool {
    fn from(this: &Bit<u8, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 == 1 }
    }
}

impl<const P: u8> From<&Bit<u128, P>> for u8 {
    fn from(this: &Bit<u128, P>) -> Self {
        let a = unsafe { (*this.data() >> P) & 1 };
        u8::try_from(a).unwrap()
    }
}
impl<const P: u8> From<&Bit<u64, P>> for u8 {
    fn from(this: &Bit<u64, P>) -> Self {
        let a = unsafe { (*this.data() >> P) & 1 };
        u8::try_from(a).unwrap()
    }
}
impl<const P: u8> From<&Bit<u32, P>> for u8 {
    fn from(this: &Bit<u32, P>) -> Self {
        let a = unsafe { (*this.data() >> P) & 1 };
        u8::try_from(a).unwrap()
    }
}
impl<const P: u8> From<&Bit<u16, P>> for u8 {
    fn from(this: &Bit<u16, P>) -> Self {
        let a = unsafe { (*this.data() >> P) & 1 };
        u8::try_from(a).unwrap()
    }
}
impl<const P: u8> From<&Bit<u8, P>> for u8 {
    fn from(this: &Bit<u8, P>) -> Self {
        unsafe { (*this.data() >> P) & 1 }
    }
}

// PartialEq impl
impl<const P: u8> PartialEq for Bit<u128, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: u8> PartialEq for Bit<u64, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: u8> PartialEq for Bit<u32, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: u8> PartialEq for Bit<u16, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: u8> PartialEq for Bit<u8, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}

// PartialEq<bool> impl
impl<const P: u8> PartialEq<bool> for Bit<u128, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: u8> PartialEq<bool> for Bit<u64, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: u8> PartialEq<bool> for Bit<u32, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: u8> PartialEq<bool> for Bit<u16, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: u8> PartialEq<bool> for Bit<u8, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}

// Eq impl
impl<const P: u8> Eq for Bit<u128, P> {}
impl<const P: u8> Eq for Bit<u64, P> {}
impl<const P: u8> Eq for Bit<u32, P> {}
impl<const P: u8> Eq for Bit<u16, P> {}
impl<const P: u8> Eq for Bit<u8, P> {}

// Mask functions
const fn mask_u128(start: u8, end: u8) -> u128 {
    // Since we can't define a const closure
    const fn sub(x: u128) -> u128 {
        x - 1
    }

    let front = map_or!(2u128.checked_pow(start as u32), u128::MAX, sub);
    let back = map_or!(2u128.checked_pow(end as u32), u128::MAX, sub);
    !front & back
}
const fn mask_u64(start: u8, end: u8) -> u64 {
    // Since we can't define a const closure
    const fn sub(x: u64) -> u64 {
        x - 1
    }

    let front = map_or!(2u64.checked_pow(start as u32), u64::MAX, sub);
    let back = map_or!(2u64.checked_pow(end as u32), u64::MAX, sub);
    !front & back
}
const fn mask_u32(start: u8, end: u8) -> u32 {
    // Since we can't define a const closure
    const fn sub(x: u32) -> u32 {
        x - 1
    }

    let front = map_or!(2u32.checked_pow(start as u32), u32::MAX, sub);
    let back = map_or!(2u32.checked_pow(end as u32), u32::MAX, sub);
    !front & back
}
const fn mask_u16(start: u8, end: u8) -> u16 {
    const fn sub(x: u16) -> u16 {
        x - 1
    }
    let front = map_or!(2u16.checked_pow(start as u32), u16::MAX, sub);
    let back = map_or!(2u16.checked_pow(end as u32), u16::MAX, sub);
    !front & back
}
const fn mask_u8(start: u8, end: u8) -> u8 {
    // Since we can't define a const closure
    const fn sub(x: u8) -> u8 {
        x - 1
    }

    let front = map_or!(2u8.checked_pow(start as u32), u8::MAX, sub);
    let back = map_or!(2u8.checked_pow(end as u32), u8::MAX, sub);
    !front & back
}
/// A replace for `Option::<T>::map_or` as it is not yet stable as a const fn.
#[macro_export]
macro_rules! map_or {
    ($in:expr,$default:expr,$function:expr) => {{
        match $in {
            Some(v) => $function(v),
            None => $default,
        }
    }};
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;
    use crate as bit_fields;
    bitfield!(
        GeneratedBitField,
        u32,
        [
            RANGE1,
            0..1,
            SSE,
            2,
            SSE1,
            3,
            RANGE2,
            4..6,
            SSE2,
            9,
            SSE3,
            10,
            RANGE3,
            12..15,
            SSE4,
            18
        ]
    );
    #[test]
    fn main() {
        println!("started");
        let mut bitfield = GeneratedBitField::from(23548);
        println!("bitfield ptr: {:?}", &bitfield as *const GeneratedBitField);
        println!(
            "bitfield: {:032b} | {:?} | {}",
            bitfield, bitfield, bitfield
        );
        println!(
            "size_of::<GeneratedBitField>(): {}",
            size_of::<GeneratedBitField>(),
        );
        println!("bitfield ptr: {:?}", &bitfield as *const GeneratedBitField);

        assert_eq!(mask_u16(0, 16), 0b1111_1111_1111_1111);
        assert_eq!(
            mask_u16(0, 8),
            0b0000_0000_1111_1111,
            "{:016b} != {:016b}",
            mask_u16(0, 8),
            0b0000_0000_1111_1111
        );
        assert_eq!(
            mask_u16(8, 16),
            0b1111_1111_0000_0000,
            "{:016b} != {:016b}",
            mask_u16(8, 16),
            0b1111_1111_0000_0000
        );
        assert_eq!(mask_u16(4, 12), 0b0000_1111_1111_0000);
        assert_eq!(mask_u16(6, 10), 0b0000_0011_1100_0000);

        assert_eq!(mask_u8(0, 8), 0b1111_1111);
        assert_eq!(mask_u8(0, 4), 0b0000_1111);
        assert_eq!(mask_u8(4, 8), 0b1111_0000);

        assert_eq!(bitfield.RANGE1_mut().checked_add_assign(1), Ok(()));
        assert_eq!(
            bitfield.RANGE1_mut().checked_add_assign(1),
            Err(CheckedAddAssignErr::Overflow)
        );
        assert_eq!(
            bitfield.RANGE2_mut().checked_add_assign(1),
            Err(CheckedAddAssignErr::Overflow)
        );
        assert_eq!(bitfield.RANGE3_mut().checked_add_assign(2), Ok(()));
        assert_eq!(
            bitfield.RANGE3_mut().checked_add_assign(1),
            Err(CheckedAddAssignErr::Overflow)
        );
        assert_eq!(
            bitfield.RANGE3_mut().checked_add_assign(8),
            Err(CheckedAddAssignErr::OutOfRange)
        );

        println!(
            "bitfield: {:032b} | {:?} | {}",
            bitfield, bitfield, bitfield
        );

        assert_eq!(bitfield.RANGE1_mut().checked_sub_assign(1), Ok(()));
        assert_eq!(
            bitfield.RANGE1_mut().checked_sub_assign(1),
            Err(CheckedSubAssignErr::Underflow)
        );
        assert_eq!(
            bitfield.RANGE1_mut().checked_sub_assign(2),
            Err(CheckedSubAssignErr::OutOfRange)
        );
        assert_eq!(bitfield.RANGE2_mut().checked_sub_assign(1), Ok(()));

        println!(
            "bitfield: {:032b} | {:?} | {}",
            bitfield, bitfield, bitfield
        );
    }
}
