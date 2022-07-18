#![allow(incomplete_features)] // For `adt_const_params`
#![feature(ptr_const_cast)]
#![feature(const_mut_refs)]
#![feature(adt_const_params)]
#![feature(const_option_ext)]

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

use std::fmt;
use std::marker::PhantomData;

use std::ops::Range;

pub use bit_fields_macros::*;

pub trait BitIndex<T, const I: usize> {
    fn bit(&self) -> &Bit<T, I>;
}
pub trait BitIndexMut<T, const I: usize> {
    fn bit_mut(&mut self) -> &mut Bit<T, I>;
}

/// A type interface for a range of bits.
#[derive(Debug, Clone, Copy)]
pub struct BitRange<T, const R: Range<usize>>(pub PhantomData<T>);

// Display impl
impl<const R: Range<usize>> fmt::Display for BitRange<u128, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u128 = self.into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u64, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u64 = self.into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u32, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u32 = self.into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u16, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u16 = self.into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u8, R> {
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

// Struct impl
impl<const R: Range<usize>> BitRange<u128, R> {
    const MASK: u128 = mask_u128(R);
    /// The maximum value this range can store plus one.
    const MAX: u128 = 2u128.pow((R.end - R.start) as u32);

    const fn data(&self) -> *const u128 {
        let a = self as *const Self;
        a as *const u128
    }

    const fn data_mut(&mut self) -> *mut u128 {
        let a = self as *const Self;
        let b = a as *const u128;
        b.as_mut()
    }

    pub const fn checked_add_assign(&mut self, x: u128) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x < Self::MAX - cur {
                let shift = x << R.start;
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

    pub const fn checked_sub_assign(&mut self, x: u128) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x <= cur {
                let shift = x << R.start;
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
    pub const fn checked_assign(&mut self, x: u128) -> Result<(), ()> {
        if x < Self::MAX {
            let shift = x << R.start;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
impl<const R: Range<usize>> BitRange<u64, R> {
    const MASK: u64 = mask_u64(R);
    /// The maximum value this range can store plus one.
    const MAX: u64 = 2u64.pow((R.end - R.start) as u32);

    const fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a as *const u64
    }

    const fn data_mut(&mut self) -> *mut u64 {
        let a = self as *const Self;
        let b = a as *const u64;
        b.as_mut()
    }

    pub const fn checked_add_assign(&mut self, x: u64) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x < Self::MAX - cur {
                let shift = x << R.start;
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

    pub const fn checked_sub_assign(&mut self, x: u64) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x <= cur {
                let shift = x << R.start;
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
    pub const fn checked_assign(&mut self, x: u64) -> Result<(), ()> {
        if x < Self::MAX {
            let shift = x << R.start;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
impl<const R: Range<usize>> BitRange<u32, R> {
    const MASK: u32 = mask_u32(R);
    /// The maximum value this range can store plus one.
    const MAX: u32 = 2u32.pow((R.end - R.start) as u32);

    const fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a as *const u32
    }

    const fn data_mut(&mut self) -> *mut u32 {
        let a = self as *const Self;
        let b = a as *const u32;
        b.as_mut()
    }

    pub const fn checked_add_assign(&mut self, x: u32) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x < Self::MAX - cur {
                let shift = x << R.start;
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

    pub const fn checked_sub_assign(&mut self, x: u32) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x <= cur {
                let shift = x << R.start;
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
    pub const fn checked_assign(&mut self, x: u32) -> Result<(), ()> {
        if x < Self::MAX {
            let shift = x << R.start;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
impl<const R: Range<usize>> BitRange<u16, R> {
    const MASK: u16 = mask_u16(R);
    /// The maximum value this range can store plus one.
    const MAX: u16 = 2u16.pow((R.end - R.start) as u32);

    const fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a as *const u16
    }

    const fn data_mut(&mut self) -> *mut u16 {
        let a = self as *const Self;
        let b = a as *const u16;
        b.as_mut()
    }

    pub const fn checked_add_assign(&mut self, x: u16) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x < Self::MAX - cur {
                let shift = x << R.start;
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

    pub const fn checked_sub_assign(&mut self, x: u16) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x <= cur {
                let shift = x << R.start;
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
    pub const fn checked_assign(&mut self, x: u16) -> Result<(), ()> {
        if x < Self::MAX {
            let shift = x << R.start;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
impl<const R: Range<usize>> BitRange<u8, R> {
    const MASK: u8 = mask_u8(R);
    /// The maximum value this range can store plus one.
    const MAX: u8 = 2u8.pow((R.end - R.start) as u32);

    const fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a as *const u8
    }

    const fn data_mut(&mut self) -> *mut u8 {
        let a = self as *const Self;
        let b = a as *const u8;
        b.as_mut()
    }

    pub const fn checked_add_assign(&mut self, x: u8) -> Result<(), CheckedAddAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x < Self::MAX - cur {
                let shift = x << R.start;
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

    pub const fn checked_sub_assign(&mut self, x: u8) -> Result<(), CheckedSubAssignErr> {
        if x < Self::MAX {
            let cur = (Self::MASK & unsafe { *self.data() }) >> R.start;
            if x <= cur {
                let shift = x << R.start;
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
    pub const fn checked_assign(&mut self, x: u8) -> Result<(), ()> {
        if x < Self::MAX {
            let shift = x << R.start;
            unsafe {
                *self.data_mut() = shift;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}

// impl<const R: Range<usize>> AddAssign<u32> for BitRange<u32, R> {
//     fn add_assign(&mut self, x: u32) {
//         let a = x << R.start;
//         let b = a + unsafe { *self.data() };
//         unsafe { *self.data_mut() = b; }
//     }
// }

// Into<uint> impl
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u128> for &BitRange<u128, R> {
    fn into(self) -> u128 {
        let a = BitRange::<u128, R>::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u64> for &BitRange<u64, R> {
    fn into(self) -> u64 {
        let a = BitRange::<u64, R>::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u32> for &BitRange<u32, R> {
    fn into(self) -> u32 {
        let a = BitRange::<u32, R>::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u16> for &BitRange<u16, R> {
    fn into(self) -> u16 {
        let a = BitRange::<u16, R>::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u8> for &BitRange<u8, R> {
    fn into(self) -> u8 {
        let a = BitRange::<u8, R>::MASK & unsafe { *self.data() };
        a >> R.start
    }
}

/// A type interface for a single bit.
#[derive(Debug, Clone, Copy)]
pub struct Bit<T, const P: usize>(pub PhantomData<T>);

// Display impl
impl<const P: usize> fmt::Display for Bit<u128, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u64, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u32, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u16, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u8, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}

// Struct impl
impl<const P: usize> Bit<u128, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u128 {
        let a = self as *const Self;
        a as *const u128
    }

    const fn data_mut(&mut self) -> *mut u128 {
        let a = self as *const Self;
        let b = a as *const u128;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u64, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a as *const u64
    }

    const fn data_mut(&mut self) -> *mut u64 {
        let a = self as *const Self;
        let b = a as *const u64;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u32, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a as *const u32
    }

    const fn data_mut(&mut self) -> *mut u32 {
        let a = self as *const Self;
        let b = a as *const u32;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u16, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a as *const u16
    }

    const fn data_mut(&mut self) -> *mut u16 {
        let a = self as *const Self;
        let b = a as *const u16;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u8, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a as *const u8
    }

    const fn data_mut(&mut self) -> *mut u8 {
        let a = self as *const Self;
        let b = a as *const u8;
        b.as_mut()
    }
}

// Into<bool> impl
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u128, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u64, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u32, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u16, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u8, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}

// Into<u8> impl
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u128, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u64, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u32, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u16, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u8, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}

// PartialEq impl
impl<const P: usize> PartialEq for Bit<u128, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u64, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u32, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u16, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u8, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}

// PartialEq<bool> impl
impl<const P: usize> PartialEq<bool> for Bit<u128, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u64, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u32, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u16, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u8, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}

// Eq impl
impl<const P: usize> Eq for Bit<u128, P> {}
impl<const P: usize> Eq for Bit<u64, P> {}
impl<const P: usize> Eq for Bit<u32, P> {}
impl<const P: usize> Eq for Bit<u16, P> {}
impl<const P: usize> Eq for Bit<u8, P> {}

// Mask functions
const fn mask_u128(Range { start, end }: Range<usize>) -> u128 {
    // Since we can't define a const closure
    const fn sub(x: u128) -> u128 {
        x - 1
    }

    let front = 2u128.checked_pow(start as u32).map_or(u128::MAX, sub);
    let back = 2u128.checked_pow(end as u32).map_or(u128::MAX, sub);
    !front & back
}
const fn mask_u64(Range { start, end }: Range<usize>) -> u64 {
    // Since we can't define a const closure
    const fn sub(x: u64) -> u64 {
        x - 1
    }

    let front = 2u64.checked_pow(start as u32).map_or(u64::MAX, sub);
    let back = 2u64.checked_pow(end as u32).map_or(u64::MAX, sub);
    !front & back
}
const fn mask_u32(Range { start, end }: Range<usize>) -> u32 {
    // Since we can't define a const closure
    const fn sub(x: u32) -> u32 {
        x - 1
    }

    let front = 2u32.checked_pow(start as u32).map_or(u32::MAX, sub);
    let back = 2u32.checked_pow(end as u32).map_or(u32::MAX, sub);
    !front & back
}
const fn mask_u16(Range { start, end }: Range<usize>) -> u16 {
    // Since we can't define a const closure
    const fn sub(x: u16) -> u16 {
        x - 1
    }

    let front = 2u16.checked_pow(start as u32).map_or(u16::MAX, sub);
    let back = 2u16.checked_pow(end as u32).map_or(u16::MAX, sub);
    !front & back
}
const fn mask_u8(Range { start, end }: Range<usize>) -> u8 {
    // Since we can't define a const closure
    const fn sub(x: u8) -> u8 {
        x - 1
    }

    let front = 2u8.checked_pow(start as u32).map_or(u8::MAX, sub);
    let back = 2u8.checked_pow(end as u32).map_or(u8::MAX, sub);
    !front & back
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

        assert_eq!(mask_u16(0..16), 0b1111_1111_1111_1111);
        assert_eq!(
            mask_u16(0..8),
            0b0000_0000_1111_1111,
            "{:016b} != {:016b}",
            mask_u16(0..8),
            0b0000_0000_1111_1111
        );
        assert_eq!(
            mask_u16(8..16),
            0b1111_1111_0000_0000,
            "{:016b} != {:016b}",
            mask_u16(8..16),
            0b1111_1111_0000_0000
        );
        assert_eq!(mask_u16(4..12), 0b0000_1111_1111_0000);
        assert_eq!(mask_u16(6..10), 0b0000_0011_1100_0000);

        assert_eq!(mask_u8(0..8), 0b1111_1111);
        assert_eq!(mask_u8(0..4), 0b0000_1111);
        assert_eq!(mask_u8(4..8), 0b1111_0000);

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
