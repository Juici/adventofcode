use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Not, Shl, Shr, Sub};
use std::str::FromStr;

use anyhow::Error;

use crate::Opcode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Repr {
    #[default]
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct u3(Repr);

macro_rules! u3 {
    ($v:literal) => {{
        let __v: u8 = $v;
        if __v > $crate::uint3::u3::MAX.to_u8() {
            panic!("integer overflows u3");
        }
        $crate::uint3::u3::from_u8(__v)
    }};
}

#[allow(dead_code)]
impl u3 {
    pub const BITS: u32 = 3;

    pub const MIN: Self = Self(Repr::_0);
    pub const MAX: Self = Self(Repr::_7);

    pub const fn from_u8(n: u8) -> Self {
        Self(match n & 7 {
            0 => Repr::_0,
            1 => Repr::_1,
            2 => Repr::_2,
            3 => Repr::_3,
            4 => Repr::_4,
            5 => Repr::_5,
            6 => Repr::_6,
            7 => Repr::_7,
            _ => unreachable!(),
        })
    }

    pub const fn to_u8(self) -> u8 {
        self.0 as u8
    }

    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::from_u8(self.to_u8().wrapping_add(rhs.to_u8()))
    }

    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::from_u8(self.to_u8().wrapping_add((!rhs.to_u8()).wrapping_add(1)))
    }

    pub const fn to_opcode(self) -> Opcode {
        match self.0 {
            Repr::_0 => Opcode::Adv,
            Repr::_1 => Opcode::Bxl,
            Repr::_2 => Opcode::Bst,
            Repr::_3 => Opcode::Jnz,
            Repr::_4 => Opcode::Bxc,
            Repr::_5 => Opcode::Out,
            Repr::_6 => Opcode::Bdv,
            Repr::_7 => Opcode::Cdv,
        }
    }
}

impl BitOr for u3 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.to_u8() | rhs.to_u8())
    }
}

impl BitAnd for u3 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.to_u8() & rhs.to_u8())
    }
}

impl BitXor for u3 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.to_u8() ^ rhs.to_u8())
    }
}

impl Not for u3 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::from_u8(!self.to_u8())
    }
}

impl Shl for u3 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.to_u8() << rhs.to_u8())
    }
}

impl Shr for u3 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self::from_u8(self.to_u8() >> rhs.to_u8())
    }
}

impl Add for u3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }
}

impl Sub for u3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl FromStr for u3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse::<u8>()?;
        if v <= Self::MAX.to_u8() {
            Ok(Self::from_u8(v))
        } else {
            Err(anyhow::anyhow!("number too large to fit in target type"))
        }
    }
}

impl fmt::Display for u3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_u8().fmt(f)
    }
}

impl fmt::LowerHex for u3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_u8().fmt(f)
    }
}
