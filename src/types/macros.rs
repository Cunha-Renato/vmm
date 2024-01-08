#[macro_export]
macro_rules! vec2 
{
    ($($e:expr),*) => (Vec2::from_array(&[$($e),*]));
}

#[macro_export]
macro_rules! vec3 
{
    ($($e:expr),*) => (Vec3::from_array(&[$($e),*]));
}

#[macro_export]
macro_rules! vec4 
{
    ($($e:expr),*) => (Vec4::from_array(&[$($e),*]));
}

/// Macro for creating a 2x2 matrix (`Mat2`) using a concise syntax.
///
/// # Syntax
///
/// The `mat2!` macro creates a Mat2 from an array of Vec2.
///
/// # Notes
///
/// - The macro internally uses the `from_mat_vec` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 2x2 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat2![vec2![1, 2], vec2![4, 3]];
///
/// assert_eq!(mat, Mat2::from_mat_vec(&[vec2![1, 2], vec2![4, 3]]));
/// ``` 
///
/// # See Also
///
/// - [`Mat2`](struct.Mat2.html): The 2x2 matrix type used by this macro.
/// - [`from_mat_vec`](fn.from_mat_vec.html): Function to construct a matrix from an array of Vec2.
#[macro_export]
macro_rules! mat2 
{
    ($($e:expr),*) => (Mat2::from_mat_vec(&[$($e),*]));
}
/// Macro for creating a 2x2 matrix (`Mat2`) using a concise syntax.
///
/// # Syntax
///
/// The `mat2_raw!` macro creates a Mat2 from a 2D array.
///
/// # Notes
///
/// - The macro internally uses the `from_mat` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 2x2 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat2_raw![[1, 2], [4, 3]];
///
/// assert_eq!(mat, Mat2::from_mat(&[[1, 2], [4, 3]]));
/// ``` 
///
/// # See Also
///
/// - [`Mat2`](struct.Mat2.html): The 2x2 matrix type used by this macro.
/// - [`from_mat`](fn.from_mat.html): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat2_raw 
{
    ($($e:expr),*) => (Mat2::from_mat(&[$($e),*]));
}
/// Macro for creating a 3x3 matrix (`Mat3`) using a concise syntax.
///
/// # Syntax
///
/// The `mat3!` macro creates a Mat3 from an array of Vec3.
///
/// # Notes
///
/// - The macro internally uses the `from_mat_vec` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 3x3 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat3![vec3![1, 2, 3], vec3![6, 5, 4], vec3![7, 8, 9]];
///
/// assert_eq!(mat, Mat3::from_mat_vec(&[vec3![1, 2, 3], vec3![6, 5, 4], vec3![7, 8, 9]]));
/// ``` 
///
/// # See Also
///
/// - [`Mat3`](struct.Mat3.html): The 3x3 matrix type used by this macro.
/// - [`from_mat_vec`](fn.from_mat_vec.html): Function to construct a matrix from an array of Vec3.
#[macro_export]
macro_rules! mat3 
{
    ($($e:expr),*) => (Mat3::from_mat_vec(&[$($e),*]));
}
/// Macro for creating a 3x3 matrix (`Mat3`) using a concise syntax.
///
/// # Syntax
///
/// The `mat3_raw!` macro creates a Mat3 from a 2D array.
///
/// # Notes
///
/// - The macro internally uses the `from_mat` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 3x3 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat3_raw![[1, 2, 3], [6, 5, 4], [7, 8, 9]];
///
/// assert_eq!(mat, Mat3::from_mat(&[[1, 2, 3], [6, 5, 4], [7, 8, 9]]));
/// ``` 
///
/// # See Also
///
/// - [`Mat3`](struct.Mat3.html): The 3x3 matrix type used by this macro.
/// - [`from_mat`](fn.from_mat.html): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat3_raw 
{
    ($($e:expr),*) => (Mat3::from_mat(&[$($e),*]));
}

/// Macro for creating a 4x4 matrix (`Mat4`) using a concise syntax.
///
/// # Syntax
///
/// The `mat4!` macro creates a Mat4 from an array of Vec4.
///
/// # Notes
///
/// - The macro internally uses the `from_mat_vec` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 4x4 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat4![vec4![1, 2, 3, 4], vec4![8, 7, 6, 5], vec4![9, 10, 11, 12], vec4![16, 15, 14, 13]];
///
/// assert_eq!(mat, Mat4::from_mat_vec(&[vec4![1, 2, 3, 4], vec4![8, 7, 6, 5], vec4![9, 10, 11, 12], vec4![16, 15, 14, 13]]));
/// ``` 
///
/// # See Also
///
/// - [`Mat4`](struct.Mat4.html): The 4x4 matrix type used by this macro.
/// - [`from_mat_vec`](fn.from_mat_vec.html): Function to construct a matrix from an array of Vec4.
#[macro_export]
macro_rules! mat4 
{
    ($($e:expr),*) => (Mat4::from_mat_vec(&[$($e),*]));
}

/// Macro for creating a 4x4 matrix (`Mat4`) using a concise syntax.
///
/// # Syntax
///
/// The `mat4_raw!` macro creates a Mat4 from a 2D array.
///
/// # Notes
///
/// - The macro internally uses the `from_mat` function to create the matrix.
/// - Ensure that the provided expressions are suitable for initializing a 4x4 matrix.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// let mat = mat4_raw![[1, 2, 3, 4], [8, 7, 6, 5], [9, 10, 11, 12], [16, 15, 14, 13]];
///
/// assert_eq!(mat, Mat4::from_mat(&[[1, 2, 3, 4], [8, 7, 6, 5], [9, 10, 11, 12], [16, 15, 14, 13]]));
/// ```
/// # See Also
///
/// - [`Mat4`](struct.Mat4.html): The 4x4 matrix type used by this macro.
/// - [`from_mat`](fn.from_mat.html): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat4_raw 
{
    ($($e:expr),*) => (Mat4::from_mat(&[$($e),*]));
}