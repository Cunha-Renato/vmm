use super::MatN;
use crate::types::{math::*, vectors::{VecN, Vec3}};

impl<T, const N: usize> ScalarMath<T> for MatN<T, N>
where 
    T: Default + Copy + Into<f64>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
    f64: From<T>   
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
    /// Creates an identity matrix of size N.
    ///
    /// An identity matrix is a square matrix with ones on the main diagonal and zeros elsewhere.
    ///
    /// # Returns
    ///
    /// A new identity matrix of size N.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// 
    /// let mat = MatN::<f64, 2>::identity();
    /// 
    /// assert_eq!(mat.to_mat(), [[1.0, 0.0], [0.0, 1.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The identity matrix is a special case of a diagonal matrix where all diagonal elements are 1.
    /// - This method assumes that the element type `T` supports conversion from `f64`.
    fn identity() -> Self;
}
pub trait MatVecMath<T, const N: usize>
where
    T: Default + Copy,
    f64: From<T>
{
    /// Multiplies the matrix by a vector.
    ///
    /// This method performs matrix-vector multiplication, resulting in a new vector.
    ///
    /// # Arguments
    ///
    /// * `vec` - The vector to multiply with the matrix.
    ///
    /// # Returns
    ///
    /// A new vector (`VecN`) representing the result of the matrix-vector multiplication.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let mat = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    /// let vec = vec2![5.0, 6.0];
    /// 
    /// assert_eq!(mat.mul_mat_vec(&vec).to_arr(), &[17.0, 39.0]);
    /// ```
    ///
    /// # Notes
    ///
    /// - Matrix-vector multiplication is performed by multiplying each row of the matrix by the corresponding
    ///   element of the vector and summing the results.
    /// - This method assumes that the element type `T` supports multiplication and addition.
    /// - The resulting vector is of the same size as the input vector.
    ///
    /// # See Also
    ///
    /// - [`VecN`](struct.VecN.html): The vector type used by this method.
    fn mul_mat_vec(&self, vec: &VecN<T, N>) -> VecN<T, N>;
}
pub trait MatTransforms<T, const N: usize>
where
    T: Default + Copy,
    f64: From<T>
{
    /// Creates a `translation` matrix and multiplies with `self`, it is dependent on the matrix dimension. 
    fn translate(&self, vec: &VecN<T, N>) -> Self;
    /// Creates a `rotation` matrix and multiplies with `self`, it is dependent on the matrix dimension.
    fn rotate(&self, angle: f64, axis: &Vec3<T>) -> Self;
    /// Creates a `scaling` matrix and multiplies with `self`, it is dependent on the matrix dimension.
    fn scale(&self, values: &Vec3<T>) -> Self;
}