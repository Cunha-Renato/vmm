pub mod math;
pub use math::*;

use std::ops::{IndexMut, Index, Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VecN<T, const N: usize>
{
    data: [T; N]
}
impl<T, const N: usize> VecN<T, N>
where 
    T: Default + Copy,
    f64: From<T>

{
    pub fn new() -> Self
    {
        Self { data: [T::default(); N] }
    }
    pub fn new_with(value: T) -> Self
    {
        Self { data: [value; N] }
    }
    pub fn from_array(data: &[T; N]) -> Self
    { 
        Self { data: *data }
    }
    pub fn fill(&mut self, value: T)
    {
        self.data.fill(value);
    }    
    pub fn to_arr(&self) -> &[T; N]
    {
        &self.data
    }
    pub fn to_mut_arr(&mut self) -> &mut [T; N]
    {
        &mut self.data
    }
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T>
    {
        self.data.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T>
    {
        self.data.iter_mut()
    }
}
impl<T, const N: usize> Index<usize> for VecN<T, N>
{
    type Output = T;    
    
    fn index(&self, index: usize) -> &Self::Output 
    {
        &self.data[index]     
    }
}
impl<T, const N: usize> IndexMut<usize> for VecN<T, N>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output 
    {
        &mut self.data[index]        
    }
}
impl<T, const N: usize> Default for VecN<T, N>
where 
    T: Default + Copy,
    f64: From<T>
{
    fn default() -> Self 
    {
        Self::new()     
    } 
}

// Operator overlads
impl<T: Add<Output = T>, const N: usize> Add for VecN<T, N>
where
    T: Default + Copy,
{
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output 
    {
        let mut result = self.clone();  
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val + *other;
        }
        
        result
    }
}
impl<T: Sub<Output = T>, const N: usize> Sub for VecN<T, N>
where
    T: Default + Copy 
{
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output 
    {
        let mut result = self.clone();  
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val - *other;
        }
        
        result
    }
}
impl<T: Mul<Output = T>, const N: usize> Mul for VecN<T, N>
where
    T: Default + Copy
{
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output 
    {
        let mut result = self.clone();     
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val * *other;
        } 
        
        result
    }
}
impl<T: Div<Output = T>, const N: usize> Div for VecN<T, N>
where
    T: Default + Copy
{
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output 
    {
        let mut result = self.clone();     
        
        for (val, other) in result.data.iter_mut().zip(rhs.data.iter())
        {
            *val = *val / *other;
        } 
        
        result
    }
}

pub type Vec2<T> = VecN<T, 2>;
pub type Vec3<T> = VecN<T, 3>;
pub type Vec4<T> = VecN<T, 4>;

impl<T> Vec3<T> 
where
    T: Copy
    + std::ops::Mul<Output = T>
    + std::ops::Sub<Output = T>
{
    pub fn cross(&self, other: &Self) -> Self
    {
        Self { data: 
        [
            self[1]*other[2] - self[2]*other[1],
            self[2]*other[0] - self[0]*other[2],
            self[0]*other[1] - self[1]*other[0]
        ]}
    }
}    