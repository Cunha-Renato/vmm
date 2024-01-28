use super::VecN;
use crate::types::math::*;

impl<T, const N: usize> ScalarMath<T> for VecN<T, N>
where
    T: Default + Copy 
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
    /// Computes the dot product of two `VecN`.
    ///
    /// The dot product of two vectors is the sum of the products of their corresponding components.
    ///
    /// # Arguments
    ///
    /// * `other` - The second vector to compute the dot product with.
    ///
    /// # Returns
    ///
    /// The dot product as a `f64` value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let vec1 = vec3![1.0, 2.0, 3.0];
    /// let vec2 = vec3![4.0, 5.0, 6.0];
    /// let dot_product = vec1.dot(&vec2);
    ///
    /// assert_eq!(dot_product, 32.0);
    /// ```
    ///
    /// # Notes
    ///
    /// - Both input vectors are need to be of the same dimension.
    /// - This method assumes that the element type `T` can be converted into `f64`.
    fn dot(&self, other: &Self) -> T;

    /// Computes the Euclidean length (magnitude) of the vector.
    ///
    /// The Euclidean length of a vector is the square root of the sum of the squares
    /// of its individual components.
    ///
    /// # Returns
    ///
    /// The Euclidean length of the vector as a `f64` value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let vec = vec3![1.0, 2.0, 2.0];
    /// let length = vec.length();
    ///
    /// assert_eq!(length, 3.0);
    /// ```
    ///
    /// # Notes
    ///
    /// - The length is computed using the formula: `sqrt(a^2 + b^2 + c^2 + ...)`.
    /// - This method assumes that the element type `T` can be converted into `f64`.
    fn length(&self) -> T;
}
impl<T, const N: usize> VecMath<T> for VecN<T, N>
where
    T: Default + Copy
        + Sqrrt
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::iter::Sum,
{
    fn dot(&self, other: &Self) -> T
    {
        self.data.iter()
            .zip(other.data.iter())
            .map(|(&a, &b)|
            {
                a * b
            })
            .sum()
    }
    fn length(&self) -> T
    {
        self.data.iter()
            .map(|&val|
            {
                let n = val;
                n*n
            }) 
            .sum::<T>()
            .sqrrt()
    } 
}
pub trait Normalize 
{
    /// Normalizes the vector to have a unit length.
    ///
    /// Normalizing a vector involves dividing each component of the vector by its Euclidean length,
    /// resulting in a new vector with a magnitude of 1.
    ///
    /// # Returns
    ///
    /// A new normalized vector with the same direction as the original.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let vec = vec3![4.0, 2.0, 0.0];
    /// let normalized_vec = vec.normalize();
    ///
    /// assert_eq!(normalized_vec.to_arr(), &[0.8944271909999159, 0.4472135954999579, 0.0]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The normalization is performed by dividing each component by the Euclidean length of the vector.
    /// - If the length of the vector is zero, the result is a vector with components set to zero.
    /// - This method assumes that the element type `T` can be converted into `f64`.
    ///
    /// # See Also
    ///
    /// - [`length`](super::VecN::length): Method to compute the Euclidean length of the vector.
    fn normalize(&self) -> Self;    
}
impl<T, const N: usize> Normalize for VecN<T, N> 
where
    T: Default + Copy
        + Sqrrt
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::iter::Sum,
{
    fn normalize(&self) -> Self 
    {
        let len = self.length();     
        let mut result = self.clone();

        for val in result.data.iter_mut()
        {
            *val = *val/len;
        }

        result 
    }
}