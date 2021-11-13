// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use std::ops::Neg;

use fpdec_core::ten_pow;

use crate::Decimal;

// TODO: remove this trait when feature(int_roundings) got stable
trait DivModInt: Sized {
    fn divmod(&self, other: &Self) -> (Self, Self);
    fn div_floor(&self, other: &Self) -> Self;
    fn div_ceil(&self, other: &Self) -> Self;
}

impl DivModInt for i128 {
    #[inline(always)]
    fn divmod(&self, other: &Self) -> (Self, Self) {
        (*self / *other, *self % other)
    }
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        let (q, r) = self.divmod(other);
        if (r > 0 && *other < 0) || (r < 0 && *other > 0) {
            q - 1
        } else {
            q
        }
    }
    #[inline]
    fn div_ceil(&self, other: &Self) -> Self {
        let (q, r) = self.divmod(other);
        if (r > 0 && *other > 0) || (r < 0 && *other < 0) {
            q + 1
        } else {
            q
        }
    }
}

impl Neg for Decimal {
    type Output = Self;

    /// Returns -self.
    ///
    /// # Panics
    ///
    /// Panics with 'attempt to negate with overflow' when called on a
    /// `Decimal` with a coefficient equal to `i128::MIN`!
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output {
            coeff: -self.coeff,
            n_frac_digits: self.n_frac_digits,
        }
    }
}

impl Neg for &Decimal {
    type Output = <Decimal as Neg>::Output;

    /// Returns -self.
    ///
    /// #Panics
    ///
    /// Panics with 'attempt to negate with overflow' when called on a
    /// `Decimal` with a coefficient equal to `i128::MIN`!
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output {
            coeff: -self.coeff,
            n_frac_digits: self.n_frac_digits,
        }
    }
}

impl Decimal {
    /// Returns the absolute value of `self`.
    #[inline(always)]
    pub fn abs(&self) -> Self {
        Self {
            coeff: self.coeff.abs(),
            n_frac_digits: self.n_frac_digits,
        }
    }

    /// Returns the largest integral value <= `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use fpdec::{Dec, Decimal};
    /// let d = Dec!(17.5);
    /// assert_eq!(d.floor().to_string(), "17");
    /// let d = Dec!(-17.050);
    /// assert_eq!(d.floor().to_string(), "-18");
    /// ```
    #[inline]
    pub fn floor(&self) -> Self {
        match self.n_frac_digits {
            0 => self.clone(),
            n => Self {
                coeff: self.coeff.div_floor(&ten_pow(n)),
                n_frac_digits: 0,
            },
        }
    }

    /// Returns the smallest integral value >= `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use fpdec::{Dec, Decimal};
    /// let d = Dec!(17.5);
    /// assert_eq!(d.ceil().to_string(), "18");
    /// let d = Dec!(-17.50);
    /// assert_eq!(d.ceil().to_string(), "-17");
    /// ```
    #[inline]
    pub fn ceil(&self) -> Self {
        match self.n_frac_digits {
            0 => self.clone(),
            n => Self {
                coeff: self.coeff.div_ceil(&ten_pow(n)),
                n_frac_digits: 0,
            },
        }
    }

    /// Returns the integral part of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use fpdec::{Dec, Decimal};
    /// let d = Dec!(17.5);
    /// assert_eq!(d.trunc().to_string(), "17");
    /// let d = Dec!(-17.55555);
    /// assert_eq!(d.trunc().to_string(), "-17");
    /// ```
    #[inline]
    pub fn trunc(&self) -> Self {
        match self.n_frac_digits {
            0 => self.clone(),
            n => Self {
                coeff: self.coeff / &ten_pow(n),
                n_frac_digits: 0,
            },
        }
    }

    /// Returns the fractional part of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use fpdec::{Dec, Decimal};
    /// let d = Dec!(17.050);
    /// assert_eq!(d.fract().to_string(), "0.050");
    /// let d = Dec!(-17.5);
    /// assert_eq!(d.fract().to_string(), "-0.5");
    /// ```
    #[inline]
    pub fn fract(&self) -> Self {
        match self.n_frac_digits {
            0 => Self::ZERO,
            n => Self {
                coeff: self.coeff % &ten_pow(n),
                n_frac_digits: n,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        let val = 1234567890i128;
        let x: Decimal = Decimal::new_raw(val, 2);
        let y = -x;
        assert_eq!(x.coeff, -y.coeff);
        let z = -y;
        assert_eq!(x.coeff, z.coeff);
        let a = &x;
        let b = -a;
        assert_eq!(a.coeff, -b.coeff);
    }

    #[test]
    fn test_neg_corner_cases_ok() {
        let x = Decimal::MAX;
        let y = -x;
        assert_eq!(x.coeff, -y.coeff);
        let z = -y;
        assert_eq!(x.coeff, z.coeff);
    }

    #[test]
    #[should_panic]
    fn test_neg_corner_cases_fail() {
        let x = Decimal::MIN;
        let _y = -x;
    }

    #[test]
    fn test_abs() {
        let x = Decimal::new_raw(-123456789, 4);
        let y = x.abs();
        assert_eq!(-x.coeff, y.coeff);
        let z = y.abs();
        assert_eq!(y.coeff, z.coeff);
        let a = &x;
        let b = a.abs();
        assert_eq!(-a.coeff, b.coeff);
    }

    #[test]
    fn test_floor() {
        let x = Decimal::new_raw(123, 0);
        let y = x.floor();
        assert_eq!(y.n_frac_digits, 0);
        assert_eq!(y.coeff, x.coeff);
        let x = Decimal::new_raw(123456789, 5);
        let y = x.floor();
        assert_eq!(y.coeff, 1234);
        assert_eq!(y.n_frac_digits, 0);
        let z = y.floor();
        assert_eq!(y.coeff, z.coeff);
        let x = Decimal::new_raw(-987, 9);
        let y = x.floor();
        assert_eq!(y.coeff, -1);
        assert_eq!(y.n_frac_digits, 0);
        let z = y.floor();
        assert_eq!(y.coeff, z.coeff);
        let a = &x;
        let b = a.floor();
        assert_eq!(b.coeff, y.coeff);
    }

    #[test]
    fn test_ceil() {
        let x = Decimal::new_raw(123, 0);
        let y = x.ceil();
        assert_eq!(y.coeff, x.coeff);
        assert_eq!(y.n_frac_digits, 0);
        let x = Decimal::new_raw(123400001, 5);
        let y = x.ceil();
        assert_eq!(y.coeff, 1235);
        assert_eq!(y.n_frac_digits, 0);
        let z = y.ceil();
        assert_eq!(y.coeff, z.coeff);
        let x = Decimal::new_raw(-987, 6);
        let y = x.ceil();
        assert_eq!(y.coeff, 0);
        assert_eq!(y.n_frac_digits, 0);
        let z = y.ceil();
        assert_eq!(y.coeff, z.coeff);
        let a = &x;
        let b = a.ceil();
        assert_eq!(b.coeff, y.coeff);
    }

    #[test]
    fn test_trunc() {
        let x = Decimal::new_raw(12345, 0);
        let y = x.trunc();
        assert_eq!(x.coeff, y.coeff);
        assert_eq!(y.n_frac_digits, 0);
        let x = Decimal::new_raw(98765, 3);
        let y = x.trunc();
        assert_eq!(y.coeff, 98);
        assert_eq!(y.n_frac_digits, 0);
        let x = Decimal::new_raw(999999, 7);
        let y = x.trunc();
        assert_eq!(y.coeff, 0);
        assert_eq!(y.n_frac_digits, 0);
        let a = &x;
        let b = a.trunc();
        assert_eq!(b.coeff, y.coeff);
    }

    #[test]
    fn test_fract() {
        let x = Decimal::new_raw(12345, 0);
        let y = x.fract();
        assert_eq!(y.coeff, 0);
        assert_eq!(y.n_frac_digits, 0);
        let x = Decimal::new_raw(987654, 3);
        let y = x.fract();
        assert_eq!(y.coeff, 654);
        assert_eq!(y.n_frac_digits, 3);
        let x = Decimal::new_raw(9999, 5);
        let y = x.fract();
        assert_eq!(y.coeff, 9999);
        assert_eq!(y.n_frac_digits, 5);
        let a = &x;
        let b = a.fract();
        assert_eq!(b.coeff, y.coeff);
    }
}