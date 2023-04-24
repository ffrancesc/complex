use std::ops::{Add, Div, Mul, Neg, Sub};
pub trait Zero {
    const O: Self;
}
pub trait One {
    const U: Self;
}
pub trait Group: Sized + Copy + Add<Output = Self> + Zero {}
pub trait Ring: Group + Sub<Output = Self> + Neg<Output = Self> + One + Mul<Output = Self> {}
pub trait Field: Ring + Div<Output = Self> {}

pub trait Module<R>
where
    R: Ring,
    Self: Sized + Copy + Add<Output = Self> + Mul<R, Output = Self>,
{
}

pub trait VectorSpace<F>
where
    F: Field,
    Self: Sized + Copy + Add<Output = Self> + Mul<F, Output = Self>,
{
}

// Group of u32
impl Zero for u32 {
    const O: Self = 0;
}
impl Group for u32 {}

// Ring of i32
impl Zero for i32 {
    const O: Self = 0;
}
impl One for i32 {
    const U: Self = 1;
}
impl Group for i32 {}
impl Ring for i32 {}

// Field of f32
impl Zero for f32 {
    const O: Self = 0.0;
}
impl One for f32 {
    const U: Self = 1.0;
}
impl Group for f32 {}
impl Ring for f32 {}
impl Field for f32 {}

// Complex Numbers
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> From<T> for Complex<T>
where
    T: Zero,
{
    fn from(re: T) -> Self {
        Complex { re, im: T::O }
    }
}

impl<T> Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn norm_sq(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T> Add<Self> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> Sub<Self> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, t: T) -> Self::Output {
        Complex {
            re: t * self.re,
            im: t * self.im,
        }
    }
}

impl<T> Mul<Self> for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> Div<T> for Complex<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;
    fn div(self, t: T) -> Self::Output {
        Complex {
            re: self.re / t,
            im: self.im / t,
        }
    }
}

impl<T> Div<Self> for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re + self.im * rhs.im,
            im: self.im * rhs.re - self.re * rhs.im,
        } / rhs.norm_sq()
    }
}

impl<T> Zero for Complex<T>
where
    T: Zero,
{
    const O: Complex<T> = Complex { re: T::O, im: T::O };
}

impl<T> One for Complex<T>
where
    T: Zero + One,
{
    const U: Complex<T> = Complex { re: T::U, im: T::O };
}

impl<T> Complex<T>
where
    T: Zero + One,
{
    pub const I: Complex<T> = Complex { re: T::O, im: T::U };
}

impl<T> Group for Complex<T> where T: Group {}
impl<T> Ring for Complex<T> where T: Ring {}
impl<T> Field for Complex<T> where T: Field {}
impl<T> Module<T> for Complex<T> where T: Ring {}
impl<T> VectorSpace<T> for Complex<T> where T: Field {}

impl std::fmt::Display for Complex<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let re_sign = if self.re > 0.0 { "" } else { " -" };
        let im_sign = if self.im > 0.0 { " + " } else { " - " };
        write!(
            f,
            "{}{}{}{}i",
            re_sign,
            self.re.abs(),
            im_sign,
            self.im.abs()
        )
    }
}
