use std::ops::{Add, Mul, Sub};

use num_traits::{PrimInt};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hex<T: PrimInt> { // Cubic coordinate storage
    pub q: T,
    pub r: T,
    pub s: T,
}

impl<T: PrimInt> Hex<T> {
    // Cubic constructor
    pub fn new(q: T, r: T, s: T) -> Self {
        assert!(q + r + s == T::zero(), "Invalid hex coordinates: q + r + s must equal 0");
        Hex { q, r, s }
    }

    // Axial constructor
    pub fn new(q: T, r: T) -> Self {
        let s: T = -q - r;
        Hex { q, r, s}
    }

    pub fn add(&self, other: &Hex<T>) -> Hex<T> {
        Hex {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl Add<T: PrimInt> for Hex<T: PrimInt> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Sub<T: PrimInt> for Hex<T: PrimInt> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl Mul<T: PrimInt> for Hex<T: PrimInt> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            q: self.q * rhs,
            r: self.r * rhs,
            s: self.s * rhs,
        }
    }
}