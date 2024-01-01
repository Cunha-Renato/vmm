use super::VecN;
use crate::types::math::*;

impl<T, const N: usize> ScalarMath<T> for VecN<T, N>
where
    T: Default + Copy + Into<f64>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    fn sum_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();

        for val in result.data.iter_mut()
        {
            *val = *val + value;
        }
        
        result
    }
    fn sub_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();     
        
        for val in result.data.iter_mut()
        {
            *val = *val - value;
        }

        result
    }
    fn mul_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();
        
        for val in result.data.iter_mut()
        {
            *val = *val * value;
        }
        
        result
    }
    fn div_scalar(&self, value: T) -> Self 
    {
        let mut result = self.clone();
        
        for val in result.data.iter_mut()
        {
            *val = *val / value;
        }
        
        result
    }
}
pub trait VecMath<T>
{
    fn dot(&self, other: &Self) -> f64;
    fn length(&self) -> f64;
}
impl<T, const N: usize> VecMath<T> for VecN<T, N>
where
    T: Default + Copy + Into<f64>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    fn dot(&self, other: &Self) -> f64
    {
        self.data.iter()
            .zip(other.data.iter())
            .map(|(&a, &b)|
            {
                Into::<f64>::into(a) * Into::<f64>::into(b)
            })
            .sum()
    }
    fn length(&self) -> f64
    {
        self.data.iter()
            .map(|&val|
            {
                let n = Into::<f64>::into(val);
                n*n
            }) 
            .sum::<f64>()
            .sqrt()
    } 
}
pub trait Normalize 
{
    fn normalize(&self) -> Self;    
}
impl<T, const N: usize> Normalize for VecN<T, N> 
where
    T: Default + Copy + Into<f64> + From<f64>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    fn normalize(&self) -> Self 
    {
        let len = self.length();     
        let mut result = self.clone();

        for val in result.data.iter_mut()
        {
            *val = (Into::<f64>::into(*val)/len).into();
        }

        result 
    }
}