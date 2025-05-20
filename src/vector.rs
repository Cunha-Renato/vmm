#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecN<T, const N: usize>(pub [T; N]);
impls!(
    {N}
    T,
    VecN,
    [T; N]
);
// Adition
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::AddAssign<T> for VecN<T, N> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.0[i] += rhs;
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::AddAssign<&[T; N]> for VecN<T, N> {
    #[inline]
    fn add_assign(&mut self, rhs: &[T; N]) {
        for i in 0..N {
            self.0[i] += rhs[i];
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::AddAssign for VecN<T, N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs.0;
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Add<T> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Add<&[T; N]> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &[T; N]) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Add for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs.0
    }
}

// Subtraction
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::SubAssign<T> for VecN<T, N> {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.0[i] -= rhs;
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::SubAssign<&[T; N]> for VecN<T, N> {
    #[inline]
    fn sub_assign(&mut self, rhs: &[T; N]) {
        for i in 0..N {
            self.0[i] -= rhs[i];
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::SubAssign for VecN<T, N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs.0;
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Sub<T> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Sub<&[T; N]> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: &[T; N]) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Sub for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs.0
    }
}

// Multiplication
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::MulAssign<T> for VecN<T, N> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.0[i] *= rhs;
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Mul<T> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

// Division
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::DivAssign<T> for VecN<T, N> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self.0[i] /= rhs;
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const N: usize> std::ops::Div<T> for VecN<T, N> {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Copy, const N: usize> VecN<T, N> {
    #[inline(always)]
    pub fn with(val: T) -> Self {
        Self([val; N])
    }
}

impl<T: num_traits::NumCast + Copy, const N: usize> VecN<T, N> {
    pub fn cast<U>(self) -> Option<VecN<U, N>> 
    where
        U: num_traits::NumCast + num_traits::Num + Copy
    {
        let mut res = VecN::with(U::zero());
        for i in 0..N {
            res[i] = U::from(self.0[i])?;
        }
            
        Some(res)
    }
}

impl<T: num_traits::NumAssign + Copy, const N: usize> VecN<T, N> {
    #[inline(always)]
    pub fn dot(&self, rhs: &[T; N]) -> T {
        dot(&self, rhs)
    }

}

impl<T: num_traits::NumAssign + num_traits::Float + Copy, const N: usize> VecN<T, N> {
    pub fn mag(&self) -> T {
        mag(&self)
    }
}

#[inline]
pub fn dot<T, const N: usize>(a: &[T; N], b: &[T; N]) -> T
where
    T: num_traits::NumAssign + Copy,
{
    let mut res = T::zero();
    for i in 0..N {
        res += a[i] * b[i];
    }

    res
}

#[inline]
pub fn mag<T, const N: usize>(val: &[T; N]) -> T
where
    T: num_traits::Float
    + num_traits::NumAssign
    + Copy
{
    let mut sq = T::zero();
    for i in 0..N {
        sq += val[i] * val[i];
    }

    sq.sqrt()
}

pub type TVec2<T> = VecN<T, 2>;
pub type TVec3<T> = VecN<T, 3>;
pub type TVec4<T> = VecN<T, 4>;

pub type Vec2 = VecN<f32, 2>;
pub type Vec3 = VecN<f32, 2>;
pub type Vec4 = VecN<f32, 2>;
pub type UVec2 = VecN<u32, 2>;
pub type UVec3 = VecN<u32, 2>;
pub type UVec4 = VecN<u32, 2>;
pub type IVec2 = VecN<i32, 2>;
pub type IVec3 = VecN<i32, 2>;
pub type IVec4 = VecN<i32, 2>;

impl<T: Copy> TVec2<T> {
    first!();
    second!();

    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}
impl<T: Copy> TVec3<T> {
    first!();
    second!();
    third!();
    
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}
impl<T: Copy> TVec4<T> {
    first!();
    second!();
    third!();
    fourth!();
    
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }
}