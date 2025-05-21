#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatMN<T, const M: usize, const N: usize>(pub [[T; M]; N]);
impl<T, const M: usize, const N: usize> core::ops::Deref for MatMN<T, M, N> {
    type Target = [[T; M]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const M: usize, const N: usize> core::ops::DerefMut for MatMN<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T, const M: usize, const N: usize> From<MatMN<T, M, N>> for [[T; M]; N] {
    fn from(val: MatMN<T, M, N>) -> Self {
        val.0
    }
}
impl<T, const M: usize, const N: usize> From<[[T; M]; N]> for MatMN<T, M, N> {
    fn from(val: [[T; M]; N]) -> Self {
        Self(val)
    }
}

// Adition
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::AddAssign<T>
    for MatMN<T, M, N>
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] += rhs;
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize>
    std::ops::AddAssign<&[[T; M]; N]> for MatMN<T, M, N>
{
    #[inline]
    fn add_assign(&mut self, rhs: &[[T; M]; N]) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] += rhs[i][j];
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::AddAssign
    for MatMN<T, M, N>
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs.0;
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Add<T>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Add<&[[T; M]; N]>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &[[T; M]; N]) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Add
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs.0
    }
}

// Subtraction
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::SubAssign<T>
    for MatMN<T, M, N>
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] -= rhs;
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize>
    std::ops::SubAssign<&[[T; M]; N]> for MatMN<T, M, N>
{
    #[inline]
    fn sub_assign(&mut self, rhs: &[[T; M]; N]) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] -= rhs[i][j];
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::SubAssign
    for MatMN<T, M, N>
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs.0;
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Sub<T>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Sub<&[[T; M]; N]>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: &[[T; M]; N]) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Sub
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs.0
    }
}

// Multiplication
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::MulAssign<T>
    for MatMN<T, M, N>
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] *= rhs;
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Mul<T>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize, const P: usize>
    std::ops::Mul<&[[T; N]; P]> for MatMN<T, M, N>
{
    type Output = MatMN<T, M, P>;

    fn mul(self, rhs: &[[T; N]; P]) -> Self::Output {
        todo!();
    }
}

// Division
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::DivAssign<T>
    for MatMN<T, M, N>
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            for j in 0..M {
                self.0[i][j] /= rhs;
            }
        }
    }
}
impl<T: num_traits::NumAssign + Copy, const M: usize, const N: usize> std::ops::Div<T>
    for MatMN<T, M, N>
{
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

// Matrices
impl<T: Copy, const M: usize, const N: usize> MatMN<T, M, N> {
    #[inline(always)]
    pub const fn with(val: T) -> Self {
        Self([[val; M]; N])
    }

    #[inline]
    pub fn transpose(&self) -> MatMN<T, N, M> {
        let mut result = MatMN::with(self.0[0][0]);
        for i in 0..N {
            for j in 0..M {
                result.0[j][i] = self.0[i][j];
            }
        }

        result
    }
}
impl<T, const M: usize, const N: usize> MatMN<T, M, N> {
    #[inline]
    pub fn to_2d_vec(self) -> Vec<Vec<T>> {
        self.0.into_iter().map(|row| row.into_iter().collect()).collect()
    }

    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        self.0.into_iter().flatten().collect()
    }
}

// Vectors
impl<T: num_traits::NumAssign + Copy, const M: usize> MatMN<T, M, 1> {
    #[inline]
    pub fn dot(&self, rhs: &[[T; M]; 1]) -> T {
        let mut result = T::zero();
        for i in 0..M {
            result += self.0[0][i] * rhs[0][i];
        }
        result
    }
}
impl<T: num_traits::NumAssign + num_traits::Float + Copy, const M: usize> MatMN<T, M, 1> {
    #[inline]
    pub fn mag(&self, rhs: &[[T; M]; 1]) -> T {
        let mut sq = T::zero();
        for i in 0..M {
            sq += self.0[0][i] * rhs[0][i];
        }
        sq.sqrt()
    }
}

pub type TVec2<T> = MatMN<T, 2, 1>;
pub type TVec3<T> = MatMN<T, 3, 1>;
pub type TVec4<T> = MatMN<T, 4, 1>;

pub type Vec2 = TVec2<f32>;
pub type Vec3 = TVec3<f32>;
pub type Vec4 = TVec4<f32>;
pub type UVec2 = TVec2<u32>;
pub type UVec3 = TVec3<u32>;
pub type UVec4 = TVec4<u32>;
pub type IVec2 = TVec2<i32>;
pub type IVec3 = TVec3<i32>;
pub type IVec4 = TVec4<i32>;

impl<T: num_traits::NumAssign + Copy> TVec2<T> {
    first!();
    second!();

    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self([[x, y]])
    }
}
impl<T: num_traits::NumAssign + Copy> TVec3<T> {
    first!();
    second!();
    third!();

    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([[x, y, z]])
    }
}
impl<T: num_traits::NumAssign + Copy> TVec4<T> {
    first!();
    second!();
    third!();
    fourth!();

    #[inline(always)]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([[x, y, z, w]])
    }
}