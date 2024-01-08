pub mod math;
pub use math::*;

use std::ops::{IndexMut, Index, Add, Sub, Mul};
use crate::{Vec2, Vec3, VecN};

/// Generic object representing a mathematical square matrix, with elements of type `T` and a fixed size `N`.
///
/// # Type Parameters
///
/// - `T`: The type of each element in the matrix.
/// - `N`: The fixed size of the matrix.
///
/// # Examples
///
/// ```
/// # use vmm::*;
/// let empty_mat: MatN<i32, 2> = MatN::new();
/// let filled_mat: MatN<f64, 2>= MatN::new_with(3.1415);
/// 
/// assert_eq!(empty_mat.to_mat(), [[0, 0], [0, 0]]);
/// assert_eq!(filled_mat.to_mat(), [[3.1415, 3.1415], [3.1415, 3.1415]]);
/// ```
///
/// # Notes
///
/// - Uses the type VecN as its rows.
///
/// # See Also
/// 
/// - [`VecN`].
/// - [`Mat2`], [`Mat3`] and [`Mat4`].
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
    /// Creates a new instance of the `MatN` object with default values for each element.
    ///
    /// This function initializes a new matrix of fixed size `N` with each element set to its default value.
    ///
    /// # Returns
    ///
    /// A new `MatN` instance with elements initialized to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mat = MatN::<f64, 2>::new();
    ///
    /// assert_eq!(mat.to_mat(), [[0.0, 0.0], [0.0, 0.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The default value for each element is determined by the `Default` trait implementation for `T`.
    /// - The size of the matrix is fixed at compile time based on the constant `N`.
    pub fn new() -> Self
    {
        Self { data: [VecN::default(); N] }
    }
    
    /// Creates a new instance of the `MatN` object with `value` as the value for each element.
    ///
    /// This function initializes a new matrix of fixed size `N` with each element set to its default value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to initialize the `VecN` with.
    ///
    /// # Returns
    ///
    /// A new `MatN` instance with elements initialized to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let mat = MatN::<f64, 3>::new_with(6.9);
    /// 
    /// assert_eq!(mat.to_mat(), [[6.9, 6.9, 6.9], [6.9, 6.9, 6.9], [6.9, 6.9, 6.9]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The size of the matrix is fixed at compile time based on the constant `N`.
    pub fn new_with(value: T) -> Self
    {
        Self { data: [VecN::new_with(value); N] } 
    }

    /// This function constructs a new matrix of fixed size `N` using the elements from the provided
    /// array of `VecN` reference `data`.
    ///
    /// # arguments
    ///
    /// * `data` - a reference to an array containing `VecN` to initialize the matrix.
    ///
    /// # returns
    ///
    /// a new `MatN` instance with elements copied from the provided array.
    ///
    /// # examples
    ///
    /// ```
    /// # use vmm::*;
    /// let array = [vec2![1.2, 4.20], vec2![6.0, 9.0]];
    /// let mat = MatN::from_mat_vec(&array);
    /// 
    /// assert_eq!(mat.to_mat_vec(), &array);
    /// ```
    ///
    /// # notes
    ///
    /// - the size of the matrix is fixed at compile time based on the constant `N`.
    pub fn from_mat_vec(data: &[VecN<T, N>; N]) -> Self
    {
        Self { data: *data }
    }

    /// This function constructs a new matrix of fixed size `N` using the elements from the provided
    /// 2D array reference `data`.
    ///
    /// # arguments
    ///
    /// * `data` - a reference to a 2D array to initialize the matrix.
    ///
    /// # returns
    ///
    /// a new `MatN` instance with elements copied from the provided array.
    ///
    /// # examples
    ///
    /// ```
    /// # use vmm::*;
    /// let array = [[1.2, 4.20], [6.0, 9.0]];
    /// let mat = MatN::from_mat(&array);
    /// 
    /// assert_eq!(mat.to_mat(), array);
    /// ```
    ///
    /// # notes
    ///
    /// - the size of the matrix is fixed at compile time based on the constant `N`.
    /// - This function creates a new `MatN` with copied elements, leaving the original 2D array unchanged 
    /// - maybe a little more expensive than `from_mat_vec()`.
    pub fn from_mat(data: &[[T; N]; N]) -> Self
    {
        let mut result = Self::new();
        
        for (vec, other) in result.data.iter_mut().zip(data.iter())
        {
            *vec = VecN::from_array(other);
        }
        
        result
    }
    
    /// Returns a reference to the underlying 2D array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mat = mat2_raw![[4, 3], [1, 2]];
    /// 
    /// assert_eq!(mat.to_mat_vec(), &[vec2![4, 3], vec2![1, 2]]);
    /// ```
    pub fn to_mat_vec(&self) -> &[VecN<T, N>; N]
    {
        &self.data
    }

    /// Returns a mutable reference to the underlying 2D array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mut mat = mat2_raw![[4, 3], [1, 2]];
    /// mat.to_mut_mat_vec()[0][1] = 180;
    /// 
    /// assert_eq!(mat.to_mat_vec(), &[vec2![4, 180], vec2![1, 2]]);
    /// ```
    pub fn to_mut_mat_vec(&mut self) -> &mut [VecN<T, N>; N]
    {
        &mut self.data
    }
    
    /// Returns a `copy` of the underlying raw 2D array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mat = mat2_raw![[4, 3], [1, 2]];
    /// 
    /// assert_eq!(mat.to_mat(), [[4, 3], [1, 2]]);
    /// ```
    ///
    /// # Notes
    /// 
    /// - More expensive than `to_mat_vec`.
    pub fn to_mat(&self) -> [[T; N]; N]
    {
        let mut result = [[T::default(); N]; N];
        
        for (val, other) in result.iter_mut().zip(self.data.iter())
        {
            *val = other.to_arr().clone();
        }
        
        result
    }
    
    /// Fills all elements of `MatN` with `value`.
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
    /// let mut mat = mat2_raw![[1.0, 3.0], [2.0, 4.0]];
    /// mat.fill(42.0);
    /// 
    /// assert_eq!(mat.to_mat(), [[42.0, 42.0], [42.0, 42.0]]);
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
        self.data.fill(VecN::new_with(value));
    }

    /// Transposes the matrix, swapping rows with columns.
    ///
    /// The transpose of a matrix is obtained by swapping its rows and columns.
    ///
    /// # Returns
    ///
    /// A new matrix representing the transpose of the original matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    ///
    /// let mat = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    ///
    /// assert_eq!(mat.transpose().to_mat(), [[1.0, 3.0], [2.0, 4.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The transpose operation swaps the positions of each element across the main diagonal of the matrix.
    /// - This method assumes that the element type `T` implements `Clone` to create a new matrix.
    /// - This has a time complexity of `O(n^2)`.
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
impl<T, const N: usize> Identity for MatN<T, N>
where
    T: Default + Copy + From<i32>,
    f64: From<T>
{
    fn identity() -> Self 
    {
        let mut result = MatN::new();
        let one = Into::<T>::into(1);
        
        for i in 0..N
        {
            result[i][i] = one;
        }
        
        result
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

impl<T> MatTransforms<T, 2> for Mat3<T>
where
    T: Default + Copy + From<f64> + Into<f64> + From<i32> 
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
    T: Default + Copy + From<f64> + Into<f64> + From<i32>
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