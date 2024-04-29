use modint2::Modint998244353;
use num::{One, Zero};
use proconio::input;

use crate::{
    matrix::{MatPow, Matrix, Vector},
    modint2::Factorial,
};

type Mint = Modint998244353;

fn main() {
    input! {
        (n, d): (usize, usize),
    }

    let fac = Factorial::<Mint>::new(d);

    let calc_comb_num = |k: usize| {
        let bb = fac.combinations(d - 1, k);
        let bw = if k >= 1 {
            fac.combinations(d - 1, k - 1)
        } else {
            Mint::new(0)
        };
        let ww = if k >= 2 {
            fac.combinations(d - 1, k - 2)
        } else {
            Mint::new(0)
        };

        let mat = Matrix::from_flattened((2, 2), vec![bb, bw, bw, ww]).mat_pow(n - 1);

        let black = Vector::from(vec![Mint::new(1), Mint::new(0)]).apply_from(&mat);
        let white = Vector::from(vec![Mint::new(0), Mint::new(1)]).apply_from(&mat);

        *black.get(0) * bb + (*black.get(1) + *white.get(0)) * bw + *white.get(1) * ww
    };

    let ans = (0..=d + 1).map(calc_comb_num).sum::<Mint>();
    println!("{}", ans);
}

impl Zero for Mint {
    fn zero() -> Self {
        Mint::new(0)
    }

    fn is_zero(&self) -> bool {
        self == &Mint::zero()
    }
}

impl One for Mint {
    fn one() -> Self {
        Mint::new(1)
    }
}

pub mod modint2 {
    //! This module implements modular arithmetic.

    use std::{
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    type InnerType = u32;

    /// Returns `x` such that `a * x` is equivalent to `1` with `m` as the modulus.
    fn modinv(a: u32, m: u32) -> u32 {
        let (mut a, mut b, mut s, mut t) = (a as i64, m as i64, 1, 0);
        while b != 0 {
            let q = a / b;
            a -= q * b;
            std::mem::swap(&mut a, &mut b);
            s -= q * t;
            std::mem::swap(&mut s, &mut t);
        }

        assert_eq!(
            a.abs(),
            1,
            "\
There is no multiplicative inverse of `a` with `m` as the modulus, \
because `a` and `m` are not prime to each other (gcd(a, m) = {}).",
            a.abs()
        );

        ((s % m as i64 + m as i64) % m as i64) as u32
    }

    pub trait Reminder {
        /// Returns the remainder divided by `modulus`.
        fn reminder(self, modulus: InnerType) -> InnerType;
    }

    macro_rules! impl_reminder_for_small_unsigned_int {
        ($($unsigned_small_int: tt), *) => {
            $(
                impl Reminder for $unsigned_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        self as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `u8`, `u16` and `u32`.
    impl_reminder_for_small_unsigned_int!(u8, u16, u32);

    macro_rules! impl_reminder_for_large_unsigned_int {
        ($($unsigned_large_int: tt), *) => {
            $(
                impl Reminder for $unsigned_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self) as InnerType
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `usize`, `u64` and `u128`.
    impl_reminder_for_large_unsigned_int!(usize, u64, u128);

    macro_rules! impl_reminder_for_small_signed_int {
        ($($signed_small_int: tt), *) => {
            $(
                impl Reminder for $signed_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self as i32 % modulus as i32 + modulus as i32) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `i8`, `i16` and `i32`.
    impl_reminder_for_small_signed_int!(i8, i16, i32);

    macro_rules! impl_reminder_for_large_signed_int {
        ($($signed_large_int: tt), *) => {
            $(
                impl Reminder for $signed_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self + modulus as Self) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `isize`, `i64` and `i128`.
    impl_reminder_for_large_signed_int!(isize, i64, i128);

    /// Structure for modular arithmetic.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modint<const MODULUS: InnerType> {
        rem: InnerType,
    }

    impl<const MODULUS: InnerType> std::fmt::Display for Modint<MODULUS> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.rem)
        }
    }

    impl<const MODULUS: InnerType> Default for Modint<MODULUS> {
        /// Returns a `Modint` instance equivalent to `0`.
        fn default() -> Self {
            Self::raw(0)
        }
    }

    impl<T, const MODULUS: InnerType> From<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn from(value: T) -> Self {
            Self::new(value)
        }
    }

    impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + MODULUS - rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem as u64 * rhs.rem as u64 % MODULUS as u64) as InnerType)
        }
    }

    impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MODULUS: InnerType> Neg for Modint<MODULUS> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::raw((MODULUS - self.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> AddAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn add_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self + rhs;
        }
    }

    impl<const MODULUS: InnerType> SubAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn sub_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self - rhs;
        }
    }

    impl<const MODULUS: InnerType> MulAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn mul_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self * rhs;
        }
    }

    impl<const MODULUS: InnerType> DivAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn div_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self / rhs;
        }
    }

    impl<const MODULUS: InnerType, T> Add<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn add(self, rhs: T) -> Self::Output {
            self + Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Sub<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn sub(self, rhs: T) -> Self::Output {
            self - Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Mul<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn mul(self, rhs: T) -> Self::Output {
            self * Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Div<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn div(self, rhs: T) -> Self::Output {
            self / Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> AddAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn add_assign(&mut self, rhs: T) {
            *self += Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> SubAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self -= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> MulAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self *= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> DivAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn div_assign(&mut self, rhs: T) {
            *self /= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType> Sum<Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }

    impl<'a, const MODULUS: InnerType> Sum<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, &x| acc + x)
        }
    }

    impl<const MODULUS: InnerType> Product<Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }

    impl<'a, const MODULUS: InnerType> Product<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, &x| acc * x)
        }
    }

    impl<const MODULUS: InnerType> Modint<MODULUS> {
        /// Returns the modulus.
        pub fn modulus() -> InnerType {
            MODULUS
        }

        /// Returns a `Modint` instance equivalent to `a`.
        pub fn new<T>(a: T) -> Self
        where
            T: Reminder,
        {
            Self {
                rem: a.reminder(MODULUS),
            }
        }

        /// Creates a `Modint` instance from a non-negative integer less than `MODULUS`.
        pub fn raw(a: InnerType) -> Self {
            Self { rem: a }
        }

        /// Set the remainder of `Modint` instance to `a`.
        pub fn set_rem<T>(&mut self, a: T)
        where
            T: Reminder,
        {
            self.rem = a.reminder(MODULUS);
        }

        /// Returns `x` such that `x * q` is equivalent to `p`.
        pub fn frac<T>(p: T, q: T) -> Self
        where
            T: Reminder,
        {
            Self::new(p) / Self::new(q)
        }

        /// Returns the remainder divided by `MODULUS`.
        /// The returned value is a non-negative integer less than `MODULUS`.
        pub fn rem(self) -> InnerType {
            self.rem
        }

        /// Returns the modular multiplicative inverse.
        pub fn inv(self) -> Self {
            Self {
                rem: modinv(self.rem, MODULUS),
            }
        }

        /// Calculates the power of `exp` using the iterative squaring method.
        pub fn pow<T>(self, exp: T) -> Self
        where
            T: ToExponent,
        {
            let mut ret = Self::new(1);
            let mut mul = self;
            let exp = exp.to_exponent();
            let mut t = exp.abs;

            while t != 0 {
                if t & 1 == 1 {
                    ret *= mul;
                }

                mul *= mul;
                t >>= 1;
            }

            if exp.neg {
                ret = ret.inv();
            }

            ret
        }
    }

    pub struct Exponent {
        neg: bool,
        abs: u128,
    }

    pub trait ToExponent {
        fn to_exponent(self) -> Exponent;
    }

    macro_rules! impl_to_exponent_for_unsigned_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: false,
                            abs: self as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_unsigned_int!(usize, u8, u16, u32, u64, u128);

    macro_rules! impl_to_exponent_for_signed_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: self.is_negative(),
                            abs: self.abs() as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_signed_int!(isize, i8, i16, i32, i64, i128);

    #[derive(Debug, Clone)]
    pub struct Factorial<Modint> {
        /// Upper limit of available factorial argument.
        upper_limit: usize,

        /// List of factorials.
        fac: Vec<Modint>,

        /// List of factorial inverses.
        inv_fac: Vec<Modint>,
    }

    impl<const MODULUS: InnerType> Factorial<Modint<MODULUS>> {
        /// Calculates factorial and its inverse for non-negative integers bellow `upper_limit`.
        pub fn new(upper_limit: usize) -> Self {
            let mut fac = vec![Modint::new(1); upper_limit + 1];
            for i in 0..upper_limit {
                fac[i + 1] = fac[i] * (i + 1);
            }

            let mut inv_fac = vec![fac[upper_limit].inv(); upper_limit + 1];
            for i in (0..upper_limit).rev() {
                inv_fac[i] = inv_fac[i + 1] * (i + 1);
            }

            Self {
                upper_limit,
                fac,
                inv_fac,
            }
        }

        /// Returns the factorial `n`.
        pub fn factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.fac[n]
        }

        /// Returns the inverse of the factorial of `n`.
        pub fn inverse_factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.inv_fac[n]
        }

        /// Calculates the number of ways to select and arrange `k` objects from `n` unique objects.
        pub fn permutations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects.
        pub fn combinations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k) * self.inverse_factorial(k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects, allowing for duplicates.
        pub fn combinations_with_repetition(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n == 0 {
                return if k == 0 {
                    Modint::new(1)
                } else {
                    Modint::new(0)
                };
            }

            self.combinations(n + k - 1, k)
        }
    }

    /// The type `Modint` with 1000000007 as the modulus.
    pub type Modint1000000007 = Modint<1000000007>;

    /// The type `Modint` with 998244353 as the modulus.
    pub type Modint998244353 = Modint<998244353>;
}

pub mod matrix {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

    use num::{One, Zero};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Matrix<T>
    where
        T: Clone,
    {
        shape: (usize, usize),
        flattened: Vec<T>,
    }

    pub trait Transpose<T>
    where
        T: Clone,
    {
        fn transposed(&self) -> Self;
    }

    pub trait Identity<T>
    where
        T: Clone + One + Zero,
    {
        fn identity(n: usize) -> Self;
    }

    pub trait MatMul<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_mul(&self, rhs: &Self) -> Self;
    }

    pub trait MatMulAssign<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_mul_assign(&mut self, rhs: &Self);
    }

    pub trait MatPow<T>
    where
        T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_pow(&self, exp: usize) -> Self;
    }

    pub trait MatMulMod<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_mul_mod(&self, rhs: &Self, modulus: T) -> Self;
    }

    pub trait MatMulAssignMod<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_mul_assign_mod(&mut self, rhs: &Self, modulus: T);
    }

    pub trait MatPowMod<T>
    where
        T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_pow_mod(&self, exp: usize, modulus: T) -> Self;
    }

    impl<T> Transpose<T> for Matrix<T>
    where
        T: Clone,
    {
        fn transposed(&self) -> Self {
            let mut flattened = vec![];

            for i in 0..self.elem_num() {
                let coord = (i % self.shape.0, i / self.shape.0);
                flattened.push(self.flattened[self.coord_to_idx(coord)].clone());
            }

            Self {
                shape: (self.shape.1, self.shape.0),
                flattened,
            }
        }
    }

    impl<T> Identity<T> for Matrix<T>
    where
        T: Clone + Zero + One,
    {
        fn identity(n: usize) -> Self {
            let mut flattened = vec![T::zero(); n * n];
            for i in 0..n {
                flattened[n * i + i] = T::one();
            }

            Self {
                shape: (n, n),
                flattened,
            }
        }
    }

    impl<T> MatMul<T> for Matrix<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_mul(&self, rhs: &Self) -> Self {
            let (h1, w1) = self.shape;
            let (h2, w2) = rhs.shape;

            assert_eq!(w1, h2);

            let calc_elem = |coord: (usize, usize)| {
                let (i, j) = coord;

                let init = self.get((i, 0)).clone() * rhs.get((0, j)).clone();

                (1..w1)
                    .map(|k| self.get((i, k)).clone() * rhs.get((k, j)).clone())
                    .fold(init, |acc, x| acc + x)
            };

            let flattened: Vec<T> = (0..(h1 * w2))
                .map(|idx| calc_elem((idx / w2, idx % w2)))
                .collect();

            Self {
                shape: (h1, w2),
                flattened,
            }
        }
    }

    impl<T> MatMulAssign<T> for Matrix<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_mul_assign(&mut self, rhs: &Self) {
            *self = self.mat_mul(rhs);
        }
    }

    impl<T> MatPow<T> for Matrix<T>
    where
        T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn mat_pow(&self, exp: usize) -> Self {
            assert!(self.is_square());

            let mut ret = Self::identity(self.shape.0);
            let mut mul = self.clone();
            let mut exp = exp;

            while exp != 0 {
                if exp % 2 == 1 {
                    ret.mat_mul_assign(&mul);
                }

                mul = mul.mat_mul(&mul);
                exp /= 2;
            }

            ret
        }
    }

    impl<T> MatMulMod<T> for Matrix<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_mul_mod(&self, rhs: &Self, modulus: T) -> Self {
            let (h1, w1) = self.shape;
            let (h2, w2) = rhs.shape;

            assert_eq!(w1, h2);

            let lhs = self.clone() % modulus.clone();
            let rhs = rhs.clone() % modulus.clone();

            let calc_elem = |coord: (usize, usize)| {
                let (i, j) = coord;

                let init = self.get((i, 0)).clone() * rhs.get((0, j)).clone() % modulus.clone();

                let elem = (1..w1)
                    .map(|k| lhs.get((i, k)).clone() * rhs.get((k, j)).clone() % modulus.clone())
                    .fold(init, |acc, x| (acc + x) % modulus.clone());

                (elem + modulus.clone()) % modulus.clone()
            };

            let flattened: Vec<T> = (0..(h1 * w2))
                .map(|idx| calc_elem((idx / w2, idx % w2)))
                .collect();

            Self {
                shape: (h1, w2),
                flattened,
            }
        }
    }

    impl<T> MatMulAssignMod<T> for Matrix<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_mul_assign_mod(&mut self, rhs: &Self, modulus: T) {
            *self = self.mat_mul_mod(rhs, modulus);
        }
    }

    impl<T> MatPowMod<T> for Matrix<T>
    where
        T: Clone + Zero + One + Add<T, Output = T> + Mul<T, Output = T> + Rem<T, Output = T>,
    {
        fn mat_pow_mod(&self, exp: usize, modulus: T) -> Self {
            assert!(self.is_square());

            let mut ret = Self::identity(self.shape.0) % modulus.clone();
            let mut mul = self.clone();
            let mut exp = exp;

            while exp != 0 {
                if exp % 2 == 1 {
                    ret.mat_mul_assign_mod(&mul, modulus.clone());
                }

                mul = mul.mat_mul_mod(&mul, modulus.clone());
                exp /= 2;
            }

            ret
        }
    }

    impl<T> From<Vec<Vec<T>>> for Matrix<T>
    where
        T: Clone,
    {
        fn from(mat: Vec<Vec<T>>) -> Self {
            let h = mat.len();
            assert_ne!(h, 0);

            let w = mat[0].len();
            assert_ne!(w, 0);

            assert!(mat.iter().all(|x| x.len() == w));

            Self {
                shape: (h, w),
                flattened: mat.into_iter().flatten().collect(),
            }
        }
    }

    impl<T> Add<&Matrix<T>> for &Matrix<T>
    where
        T: Clone + Add<T, Output = T>,
    {
        type Output = Matrix<T>;

        fn add(self, rhs: &Matrix<T>) -> Self::Output {
            assert_eq!(self.shape, rhs.shape);

            let flattened = self
                .flattened
                .iter()
                .zip(rhs.flattened.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect();

            Matrix {
                shape: self.shape,
                flattened,
            }
        }
    }

    impl<T> AddAssign<&Matrix<T>> for Matrix<T>
    where
        T: Clone + AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: &Matrix<T>) {
            self.flattened
                .iter_mut()
                .zip(rhs.flattened.iter())
                .for_each(|(x, y)| *x += y.clone());
        }
    }

    impl<T> Sub<&Matrix<T>> for &Matrix<T>
    where
        T: Clone + Sub<T, Output = T>,
    {
        type Output = Matrix<T>;

        fn sub(self, rhs: &Matrix<T>) -> Self::Output {
            assert_eq!(self.shape, rhs.shape);

            let flattened = self
                .flattened
                .iter()
                .zip(rhs.flattened.iter())
                .map(|(x, y)| x.clone() - y.clone())
                .collect();

            Self::Output {
                shape: self.shape,
                flattened,
            }
        }
    }

    impl<T> SubAssign<&Matrix<T>> for Matrix<T>
    where
        T: Clone + SubAssign<T>,
    {
        fn sub_assign(&mut self, rhs: &Matrix<T>) {
            self.flattened
                .iter_mut()
                .zip(rhs.flattened.iter())
                .for_each(|(x, y)| *x -= y.clone());
        }
    }

    impl<T> Mul<&Matrix<T>> for &Matrix<T>
    where
        T: Clone + Mul<T, Output = T>,
    {
        type Output = Matrix<T>;

        fn mul(self, rhs: &Matrix<T>) -> Self::Output {
            assert_eq!(self.shape, rhs.shape);

            let flattened: Vec<T> = self
                .flattened
                .iter()
                .zip(rhs.flattened.iter())
                .map(|(x, y)| x.clone() * y.clone())
                .collect();

            Self::Output {
                shape: self.shape,
                flattened,
            }
        }
    }

    impl<T> MulAssign<&Matrix<T>> for Matrix<T>
    where
        T: Clone + MulAssign<T>,
    {
        fn mul_assign(&mut self, rhs: &Matrix<T>) {
            assert_eq!(self.shape, rhs.shape);

            self.flattened
                .iter_mut()
                .zip(rhs.flattened.iter())
                .for_each(|(x, y)| *x *= y.clone());
        }
    }

    impl<T> Rem<T> for Matrix<T>
    where
        T: Clone + Rem<T, Output = T>,
    {
        type Output = Matrix<T>;

        fn rem(self, rhs: T) -> Self::Output {
            let flattened: Vec<T> = self
                .flattened
                .iter()
                .map(|x| x.clone() % rhs.clone())
                .collect();

            Self::Output {
                shape: self.shape,
                flattened,
            }
        }
    }

    impl<T> RemAssign<T> for Matrix<T>
    where
        T: Clone + RemAssign<T>,
    {
        fn rem_assign(&mut self, rhs: T) {
            self.flattened.iter_mut().for_each(|x| *x %= rhs.clone());
        }
    }

    impl<T> Matrix<T>
    where
        T: Clone,
    {
        pub fn new(shape: (usize, usize)) -> Self
        where
            T: Default,
        {
            assert!(shape.0 >= 1 && shape.1 >= 1);

            Self {
                shape,
                flattened: vec![T::default(); shape.0 * shape.1],
            }
        }

        pub fn from_flattened(shape: (usize, usize), flattened: Vec<T>) -> Self {
            assert!(shape.0 >= 1 && shape.1 >= 1);
            assert_eq!(shape.0 * shape.1, flattened.len());

            Self { shape, flattened }
        }

        pub fn filled(shape: (usize, usize), x: T) -> Self {
            Self::from_flattened(shape, vec![x; shape.0 * shape.1])
        }

        pub fn zero(shape: (usize, usize)) -> Self
        where
            T: Zero,
        {
            Self::from_flattened(shape, vec![T::zero(); shape.0 * shape.1])
        }

        pub fn one(shape: (usize, usize)) -> Self
        where
            T: One,
        {
            Self::from_flattened(shape, vec![T::one(); shape.0 * shape.1])
        }

        pub fn from_vector(vec: Vec<T>) -> Self {
            Self {
                shape: (vec.len(), 1),
                flattened: vec,
            }
        }

        pub fn shape(&self) -> (usize, usize) {
            self.shape
        }

        pub fn flattened(self) -> Vec<T> {
            self.flattened
        }

        pub fn elem_num(&self) -> usize {
            self.shape.0 * self.shape.1
        }

        pub fn to_vec(self) -> Vec<Vec<T>> {
            let (h, w) = self.shape;

            let mut mat = vec![vec![]; h];
            mat.iter_mut().for_each(|x| x.reserve(w));

            for (i, elem) in self.flattened.into_iter().enumerate() {
                mat[i / w].push(elem)
            }

            mat
        }

        #[allow(unused)]
        fn coord_to_idx(&self, coord: (usize, usize)) -> usize {
            debug_assert!(coord.0 < self.shape.0 && coord.1 < self.shape.1);

            coord.0 * self.shape.1 + coord.1
        }

        #[allow(unused)]
        fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
            debug_assert!(idx < self.elem_num());

            (idx / self.shape.1, idx % self.shape.1)
        }

        pub fn get(&self, coord: (usize, usize)) -> &T {
            let idx = self.coord_to_idx(coord);

            &self.flattened[idx]
        }

        pub fn get_mut(&mut self, coord: (usize, usize)) -> &mut T {
            let idx = self.coord_to_idx(coord);

            &mut self.flattened[idx]
        }

        pub fn set(&mut self, coord: (usize, usize), val: T) {
            let idx = self.coord_to_idx(coord);

            self.flattened[idx] = val;
        }

        pub fn is_square(&self) -> bool {
            self.shape.0 == self.shape.1
        }

        pub fn apply_to(&self, vec: &Vector<T>) -> Vector<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            let (h, w) = self.shape;

            assert_eq!(w, vec.len());

            let calc_elem = |i: usize| {
                let mut elem = self.get((i, 0)).clone() * vec.get(0).clone();

                for j in 1..w {
                    elem = elem + self.get((i, j)).clone() * vec.get(j).clone();
                }

                elem
            };

            (0..h).map(calc_elem).collect::<Vec<T>>().into()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Vector<T>
    where
        T: Clone,
    {
        elements: Vec<T>,
    }

    pub trait InnerProduct<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn inner_product(&self, rhs: &Vector<T>) -> T;
    }

    impl<T> InnerProduct<T> for Vector<T>
    where
        T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
    {
        fn inner_product(&self, rhs: &Vector<T>) -> T {
            assert_eq!(self.len(), rhs.len());

            let mut ret = self.get(0).clone() * rhs.get(0).clone();

            for i in 1..self.len() {
                ret = ret + self.get(i).clone() * rhs.get(i).clone();
            }

            ret
        }
    }

    impl<T> From<Vec<T>> for Vector<T>
    where
        T: Clone,
    {
        fn from(elements: Vec<T>) -> Self {
            assert!(!elements.is_empty());

            Self { elements }
        }
    }

    impl<T> Add<&Vector<T>> for &Vector<T>
    where
        T: Clone + Add<T, Output = T>,
    {
        type Output = Vector<T>;

        fn add(self, rhs: &Vector<T>) -> Self::Output {
            let elements: Vec<T> = self
                .elements
                .iter()
                .zip(rhs.elements.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect();

            Self::Output { elements }
        }
    }

    impl<T> AddAssign<&Vector<T>> for Vector<T>
    where
        T: Clone + AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: &Vector<T>) {
            self.elements
                .iter_mut()
                .zip(rhs.elements.iter())
                .for_each(|(x, y)| *x += y.clone());
        }
    }

    impl<T> Sub<&Vector<T>> for &Vector<T>
    where
        T: Clone + Sub<T, Output = T>,
    {
        type Output = Vector<T>;

        fn sub(self, rhs: &Vector<T>) -> Self::Output {
            let elements: Vec<T> = self
                .elements
                .iter()
                .zip(rhs.elements.iter())
                .map(|(x, y)| x.clone() - y.clone())
                .collect();

            Self::Output { elements }
        }
    }

    impl<T> SubAssign<&Vector<T>> for Vector<T>
    where
        T: Clone + SubAssign<T>,
    {
        fn sub_assign(&mut self, rhs: &Vector<T>) {
            self.elements
                .iter_mut()
                .zip(rhs.elements.iter())
                .for_each(|(x, y)| *x -= y.clone());
        }
    }

    impl<T> Mul<&Vector<T>> for &Vector<T>
    where
        T: Clone + Mul<T, Output = T>,
    {
        type Output = Vector<T>;

        fn mul(self, rhs: &Vector<T>) -> Self::Output {
            let elements: Vec<T> = self
                .elements
                .iter()
                .zip(rhs.elements.iter())
                .map(|(x, y)| x.clone() * y.clone())
                .collect();

            Self::Output { elements }
        }
    }

    impl<T> MulAssign<&Vector<T>> for Vector<T>
    where
        T: Clone + MulAssign<T>,
    {
        fn mul_assign(&mut self, rhs: &Vector<T>) {
            self.elements
                .iter_mut()
                .zip(rhs.elements.iter())
                .for_each(|(x, y)| *x *= y.clone());
        }
    }

    impl<T> Vector<T>
    where
        T: Clone,
    {
        pub fn new(n: usize) -> Self
        where
            T: Default,
        {
            assert_ne!(n, 0);

            vec![T::default(); n].into()
        }

        pub fn len(&self) -> usize {
            self.elements.len()
        }

        pub fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }

        pub fn elements(&self) -> &Vec<T> {
            &self.elements
        }

        pub fn to_vec(self) -> Vec<T> {
            self.elements
        }

        pub fn zero(n: usize) -> Self
        where
            T: Zero,
        {
            vec![T::zero(); n].into()
        }

        pub fn one(n: usize) -> Self
        where
            T: One,
        {
            vec![T::one(); n].into()
        }

        pub fn filled(x: T, n: usize) -> Self {
            vec![x; n].into()
        }

        pub fn get(&self, idx: usize) -> &T {
            &self.elements[idx]
        }

        pub fn apply_from(&self, mat: &Matrix<T>) -> Vector<T>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
        {
            let (h, w) = mat.shape;

            assert_eq!(self.len(), w);

            let calc_elem = |i: usize| {
                let mut elem = mat.get((i, 0)).clone() * self.get(0).clone();

                for j in 1..w {
                    elem = elem + mat.get((i, j)).clone() * self.get(j).clone();
                }

                elem
            };

            (0..h).map(calc_elem).collect::<Vec<T>>().into()
        }
    }
}
