pub mod math;
pub use math::*;

use std::ops::{IndexMut, Index, Add, Sub, Mul};
use crate::{Vec2, Vec3, Vec4, VecN, vec2, vec3, vec4};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatN<T, const N: usize>
where
    T: Default + Copy,
    f64: From<T>
{
    data: [VecN<T, N>; N]    
}
impl<T, const N: usize> MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    pub fn new() -> Self
    {
        Self { data: [VecN::default(); N] }
    }
    pub fn new_with(value: T) -> Self
    {
        Self { data: [VecN::new_with(value); N] } 
    }
    pub fn from_mat_vec(data: &[VecN<T, N>; N]) -> Self
    {
        Self { data: *data }
    }
    pub fn from_mat(data: &[[T; N]; N]) -> Self
    {
        let mut result = Self::new();
        
        for (vec, other) in result.data.iter_mut().zip(data.iter())
        {
            *vec = VecN::from_array(other);
        }
        
        result
    }
    pub fn to_mat_vec(&self) -> &[VecN<T, N>; N]
    {
        &self.data
    }
    pub fn to_mut_mat_vec(&mut self) -> &mut [VecN<T, N>; N]
    {
        &mut self.data
    }
    pub fn to_mat(&self) -> [[T; N]; N]
    {
        let mut result = [[T::default(); N]; N];
        
        for (val, other) in result.iter_mut().zip(self.data.iter())
        {
            *val = other.to_arr().clone();
        }
        
        result
    }
    pub fn fill(&mut self, value: T)
    {
        *self = Self::new_with(value); 
    }
    pub fn transpose(&self) -> Self
    {
        let mut result = self.clone();

        for i in 0..N {
            for j in 0..N
            {
                result[i][j] = self[j][i];
            }
        }

        result
    }
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, VecN<T, N>>
    {
        self.data.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, VecN<T, N>>
    {
        self.data.iter_mut()
    }
}
impl<T, const N: usize> Index<usize> for MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    type Output = VecN<T, N>; 
    
    fn index(&self, index: usize) -> &Self::Output 
    {
        &self.data[index]     
    }
}
impl<T, const N: usize> IndexMut<usize> for MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output 
    {
        &mut self.data[index]     
    }
}
impl<T, const N: usize> Default for MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    fn default() -> Self 
    {
        Self { data: [VecN::default(); N] }     
    }
}

impl<T: Add<Output = T>, const N: usize> Add for MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        let mut result = self;
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val + *other; 
        }
        
        result
    } 
}
impl<T: Sub<Output = T>, const N: usize> Sub for MatN<T, N>
where
    T: Default + Copy,
    f64: From<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output
    {
        let mut result = self;
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val - *other; 
        }
        
        result
    } 
}
impl<T: Mul<Output = T>, const N: usize> Mul for MatN<T, N>
where
    T: Default + Copy + std::ops::Add<Output = T>,
    f64: From<T>
{
    type Output = Self; 
    
    fn mul(self, rhs: Self) -> Self::Output 
    {
        let mut result = MatN::new();
        
        for i in 0..N {
            for j in 0..N {
                for k in 0..N
                {
                    result[i][j] = result[i][j] + self[i][k] * rhs[k][j];
                }
            }
        }

        result
    }
}

impl<T, const N: usize> MatVecMath<T, N> for MatN<T, N>
where
    T: Default + Copy
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>,
    f64: From<T>
{
    fn mul_mat_vec(&self, vec: &VecN<T, N>) -> VecN<T, N> 
    {
        let mut result = VecN::new();

        for (i, vector) in self.data.iter().enumerate()
        {
            for (j, val) in vector.to_arr().iter().enumerate()
            {
                result[i] = result[i] + vec[j] * *val
            }
        }

        result
    } 
}

pub type Mat2<T> = MatN<T, 2>;
pub type Mat3<T> = MatN<T, 3>;
pub type Mat4<T> = MatN<T, 4>;

impl<T> Identity for Mat2<T>
where
    T: Default + Copy + From<f64>,
    f64: From<T>
{
    fn identity() -> Self 
    {
        let zero = T::default();
        let one = Into::<T>::into(1.0);
        Mat2 {
            data: [
                vec2![one, zero],
                vec2![zero, one]
            ]     
        }
    }
}
impl<T> Identity for Mat3<T>
where
    T: Default + Copy + From<f64>,
    f64: From<T>
{
    fn identity() -> Self 
    {
        let zero = T::default();
        let one = Into::<T>::into(1.0);
        Mat3 {
            data: [
                vec3![one, zero, zero],
                vec3![zero, one, zero],
                vec3![zero, zero, one]
            ]     
        }
    }
}
impl<T> Identity for Mat4<T>
where
    T: Default + Copy + From<f64>,
    f64: From<T>
{
    fn identity() -> Self 
    {
        let zero = T::default();
        let one = Into::<T>::into(1.0);
        Mat4 {
            data: [
                vec4![one, zero, zero, zero],
                vec4![zero, one, zero, zero],
                vec4![zero, zero, one, zero],
                vec4![zero, zero, zero, one]
            ]     
        }
    }
}

impl<T> MatTransforms<T, 2> for Mat3<T>
where
    T: Default + Copy + From<f64> + Into<f64> 
        + std::ops::Neg<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
    f64: From<T>
{
    fn translate(&self, vec: &Vec2<T>) -> Self 
    {
        let mut result = Mat3::identity();
        result[0][2] = vec[0];
        result[1][2] = vec[1];
        
        result
    }
    fn rotate(&self, angle: f64, axis: &Vec3<T>) -> Self 
    {
        let x_angle: f64 = axis[0].into() * angle; 
        let y_angle: f64 = axis[1].into() * angle;
        let z_angle: f64 = axis[2].into() * angle;

        let x_cos: T = x_angle.cos().into();
        let x_sin: T = x_angle.sin().into();
        let y_cos: T = y_angle.cos().into();
        let y_sin: T = y_angle.sin().into();
        let z_cos: T = z_angle.cos().into();
        let z_sin: T = z_angle.sin().into();

        let mut x_mat = Mat3::identity();
        x_mat[1][1] = x_cos;
        x_mat[1][2] = -x_sin;
        x_mat[2][1] = x_sin;
        x_mat[2][2] = x_cos;
        
        let mut y_mat = Mat3::identity();
        y_mat[0][0] = y_cos;
        y_mat[0][2] = y_sin;
        y_mat[2][0] = -y_sin;
        y_mat[2][2] = y_cos;
        
        let mut z_mat = Mat3::identity();
        z_mat[0][0] = z_cos;
        z_mat[0][1] = -z_sin;
        z_mat[1][0] = z_sin;
        z_mat[1][1] = z_cos;
        
        *self * (x_mat * y_mat * z_mat)
    }
    fn scale(&self, values: &Vec3<T>) -> Self 
    {
        let mut result = Mat3::identity();     
        
        result[0][0] = values[0];
        result[1][1] = values[1];
        result[2][2] = values[2];
        
        result
    }
}
impl<T> MatTransforms<T, 3> for Mat4<T>
where
    T: Default + Copy + From<f64> + Into<f64> 
        + std::ops::Neg<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>,
    f64: From<T>
{
    fn translate(&self, vec: &Vec3<T>) -> Self 
    {
        let mut result = Mat4::identity();             
        result[0][3] = vec[0];
        result[1][3] = vec[1];
        result[2][3] = vec[2];
        
        result
    }
    fn rotate(&self, angle: f64, axis: &Vec3<T>) -> Self 
    {
        let x_angle: f64 = axis[0].into() * angle; 
        let y_angle: f64 = axis[1].into() * angle;
        let z_angle: f64 = axis[2].into() * angle;

        let x_cos: T = x_angle.cos().into();
        let x_sin: T = x_angle.sin().into();
        let y_cos: T = y_angle.cos().into();
        let y_sin: T = y_angle.sin().into();
        let z_cos: T = z_angle.cos().into();
        let z_sin: T = z_angle.sin().into();

        let mut x_mat = Mat4::identity();
        x_mat[1][1] = x_cos;
        x_mat[1][2] = -x_sin;
        x_mat[2][1] = x_sin;
        x_mat[2][2] = x_cos;
        
        let mut y_mat = Mat4::identity();
        y_mat[0][0] = y_cos;
        y_mat[0][2] = y_sin;
        y_mat[2][0] = -y_sin;
        y_mat[2][2] = y_cos;
        
        let mut z_mat = Mat4::identity();
        z_mat[0][0] = z_cos;
        z_mat[0][1] = -z_sin;
        z_mat[1][0] = z_sin;
        z_mat[1][1] = z_cos;
        
        *self * (x_mat * y_mat * z_mat)
    }
    fn scale(&self, values: &Vec3<T>) -> Self 
    {
        let mut result = Mat4::identity();     
        
        result[0][0] = values[0];
        result[1][1] = values[1];
        result[2][2] = values[2];
        
        result
    }
}