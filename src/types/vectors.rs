pub mod math;
pub use math::*;

use std::ops::{IndexMut, Index, Add, Sub, Mul, Div};

/// Generic object representing a mathematical vector, with elements of type `T` and a fixed size `N`.
///
/// # Type Parameters
///
/// - `T`: The type of each element in the vector.
/// - `N`: The fixed size of the vector.
///
/// # Examples
///
/// ```
/// # use vmm::*;
/// let empty_vec: VecN<i32, 2> = VecN::new();
/// let filled_vec: VecN<f64, 3>= VecN::new_with(3.1415);
/// 
/// assert_eq!(empty_vec.to_arr(), &[0, 0]);
/// assert_eq!(filled_vec.to_arr(), &[3.1415, 3.1415, 3.1415]);
/// ```
///
/// # Notes
///
/// - The size of the vector is determined at compile time, providing safety against
///   accessing elements out of bounds, in addition to the speed.
///
/// # See Also
///
/// - [`MatN`](struct.MatN.html): Matrix type using vectors of fixed size (`VecN`) as rows.
/// - [`Vec2`](struct.Vec2.html), [`Vec3`](struct.Vec3.html), ...: Specialized vector types
///   with fixed sizes for 2D, 3D, and higher dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VecN<T, const N: usize>
where
    T: Default + Copy,
    f64: From<T>
{
    data: [T; N]
}
impl<T, const N: usize> VecN<T, N>
where 
    T: Default + Copy,
    f64: From<T>

{
    /// Creates a new instance of the `VecN` object with default values for each element.
    ///
    /// This function initializes a new vector of fixed size `N` with each element set to its default value.
    ///
    /// # Returns
    ///
    /// A new `VecN` instance with elements initialized to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let vec_3d_default = VecN::<f64, 3>::new();
    /// let vec_2d_default = VecN::<i32, 2>::new();
    /// ```
    ///
    /// # Notes
    ///
    /// - The default value for each element is determined by the `Default` trait implementation for `T`.
    /// - The size of the vector is fixed at compile time based on the constant `N`.
    pub fn new() -> Self
    {
        Self { data: [T::default(); N] }
    }
    /// Creates a new instance of the `VecN` object with `value` as the value for each element.
    ///
    /// This function initializes a new vector of fixed size `N` with each element set to its default value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to initialize the `VecN` with.
    ///
    /// # Returns
    ///
    /// A new `VecN` instance with elements initialized to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let vec = VecN::<f64, 3>::new_with(6.9);
    /// 
    /// assert_eq!(vec.to_arr(), &[6.9, 6.9, 6.9]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The size of the vector is fixed at compile time based on the constant `N`.
    pub fn new_with(value: T) -> Self
    {
        Self { data: [value; N] }
    }
    /// This function constructs a new vector of fixed size `N` using the elements from the provided
    /// array reference `data`. The elements of the array are copied to initialize the vector.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to an array containing elements to initialize the vector.
    ///
    /// # Returns
    ///
    /// A new `VecN` instance with elements copied from the provided array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let array = [1.0, 2.0, 3.0];
    /// let vec = VecN::from_array(&array);
    /// 
    /// assert_eq!(vec.to_arr(), &array);
    /// ```
    ///
    /// # Notes
    ///
    /// - The size of the vector is fixed at compile time based on the constant `N`.
    /// - This function creates a new `VecN` with copied elements, leaving the original array unchanged.
    pub fn from_array(data: &[T; N]) -> Self
    { 
        Self { data: *data }
    }

    /// Fills all elements of `VecN` with `value`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to fill the vector with.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mut vec = VecN::<f64, 3>::new();
    /// vec.fill(42.0);
    /// 
    /// assert_eq!(vec.to_arr(), &[42.0, 42.0, 42.0])
    /// ```
    ///
    /// # Notes
    ///
    /// - This method directly delegates to the `fill` method of the underlying array.
    ///
    /// # See Also
    ///
    /// - [`fill`](https://doc.rust-lang.org/std/primitive.array.html#method.fill): The standard library method
    ///   used internally to fill the underlying array.
    pub fn fill(&mut self, value: T)
    {
        self.data.fill(value);
    }    

    /// Returns a reference to the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mut vec = VecN::<f64, 3>::new();
    /// vec.fill(42.0);
    /// 
    /// assert_eq!(vec.to_arr(), &[42.0, 42.0, 42.0])
    /// ```
    pub fn to_arr(&self) -> &[T; N]
    {
        &self.data
    }

    /// Returns a mutable reference to the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mut vec = VecN::<f64, 3>::new();
    /// vec.fill(42.0);
    /// 
    /// vec.to_mut_arr()[1] = 6.9;
    ///
    /// assert_eq!(vec.to_arr(), &[42.0, 6.9, 42.0]);
    /// ```
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
where
    T: Default + Copy,
    f64: From<T>
{
    type Output = T;    
    
    fn index(&self, index: usize) -> &Self::Output 
    {
        &self.data[index]     
    }
}
impl<T, const N: usize> IndexMut<usize> for VecN<T, N>
where
    T: Default + Copy,
    f64: From<T>
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
    f64: From<T>
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
    T: Default + Copy,
    f64: From<T>
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
    T: Default + Copy,
    f64: From<T>
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
    T: Default + Copy,
    f64: From<T>
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
    T: Default + Copy
    + std::ops::Mul<Output = T>
    + std::ops::Sub<Output = T>,
    f64: From<T>
{
    /// Computes the cross product of two 3D vectors.
    ///
    /// The cross product of two 3D vectors results in a new vector that is perpendicular
    /// to both input vectors. The cross product is computed as follows:
    ///
    /// ```plaintext
    /// | i  j  k  |
    /// | a1 a2 a3 |
    /// | b1 b2 b3 |
    ///
    /// Result: [ (a2*b3 - a3*b2), (a3*b1 - a1*b3), (a1*b2 - a2*b1) ]
    /// ```
    ///
    /// # Arguments
    ///
    /// * `other` - The second vector to compute the cross product with.
    ///
    /// # Returns
    ///
    /// A new `Vec3` instance representing the cross product of the two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let vec1 = vec3![1.0, 2.0, 3.0];
    /// let vec2 = vec3![4.0, 5.0, 6.0];
    /// let result = vec1.cross(&vec2);
    ///
    /// assert_eq!(result.to_arr(), &[-3.0, 6.0, -3.0]); 
    /// ```
    ///
    /// # Notes
    ///
    /// - Both input `VecN` and the resulting `VecN` are in 3D space, ie `Vec3`.
    ///
    /// # See Also
    ///
    /// - [`Vec3`](struct.Vec3.html): The 3D vector type used by this method.
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