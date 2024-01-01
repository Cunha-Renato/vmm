use super::MatN;
use crate::types::{math::*, vectors::{VecN, Vec3}};

impl<T, const N: usize> ScalarMath<T> for MatN<T, N>
where 
    T: Default + Copy + Into<f64>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
{
    fn sum_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();

        for val in result.data.iter_mut()
        {
            *val = val.sum_scalar(value);
        } 
        
        result
    }
    fn sub_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();
        
        for val in result.data.iter_mut()
        {
            *val = val.sub_scalar(value);
        }
        
        result
    }
    fn mul_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();
        
        for val in result.data.iter_mut()
        {
            *val = val.mul_scalar(value);
        }
        
        result
    }    
    fn div_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();
        
        for val in result.data.iter_mut()
        {
            *val = val.div_scalar(value);
        }
        
        result
    }
}

pub trait Identity
{
    fn identity() -> Self;
}
pub trait MatVecMath<T, const N: usize>
{
    fn mul_mat_vec(&self, vec: &VecN<T, N>) -> VecN<T, N>;
}
pub trait MatTransforms<T, const N: usize>
{
    fn translate(&self, vec: &VecN<T, N>) -> Self;
    fn rotate(&self, angle: f64, axis: &Vec3<T>) -> Self;
    fn scale(&self, values: &Vec3<T>) -> Self;
}

