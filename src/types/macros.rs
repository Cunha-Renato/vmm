/// Macro for creating a 2D vector (`Vec2`) using a concise syntax.
///
/// # Syntax
///
/// The `vec2!` macro accepts a comma-separated list of values, representing the elements of
/// a 2D vector. It constructs a `Vec2` instance using the provided values.
///
/// # Examples
///
/// ```
/// # use vmm::*;
/// let vec = vec2![1.0, 2.0];
/// 
/// assert_eq!(vec.to_arr(), &[1.0, 2.0]);
/// ```
///
/// # Notes
///
/// - The macro internally uses the `from_array` function to create the vector.
/// - Ensure that the provided expressions are suitable for initializing a 2D vector.
/// - The resulting `Vec2` struct will be created using the `from_array` function.
///
/// # See Also
///
/// - [`Vec2`](super::vectors::Vec2): The 2D vector type used by this macro.
/// - [`from_array`](super::vectors::VecN::from_array): Function to construct a vector from an array.
#[macro_export]
macro_rules! vec2 
{
    ($($e:expr),*) => (vmm::Vec2::from_array(&[$($e),*]));
}

/// Macro for creating a 3D vector (`Vec3`) using a concise syntax.
///
/// # Syntax
///
/// The `vec3!` macro accepts a comma-separated list of values, representing the elements of
/// a 3D vector. It constructs a `Vec3` instance using the provided values.
///
/// # Examples
///
/// ```
/// # use vmm::*;
/// let vec = vec3![1.0, 2.0, 3.0];
/// 
/// assert_eq!(vec.to_arr(), &[1.0, 2.0, 3.0]);
/// ```
///
/// # Notes
///
/// - The macro internally uses the `from_array` function to create the vector.
/// - Ensure that the provided expressions are suitable for initializing a 3D vector.
/// - The resulting `Vec3` struct will be created using the `from_array` function.
///
/// # See Also
///
/// - [`Vec3`](super::vectors::Vec3): The 3D vector type used by this macro.
/// - [`from_array`](super::vectors::VecN::from_array): Function to construct a vector from an array.
#[macro_export]
macro_rules! vec3 
{
    ($($e:expr),*) => (vmm::Vec3::from_array(&[$($e),*]));
}

/// Macro for creating a 4D vector (`Vec4`) using a concise syntax.
///
/// # Syntax
///
/// The `vec4!` macro accepts a comma-separated list of values, representing the elements of
/// a 4D vector. It constructs a `Vec4` instance using the provided values.
///
/// # Examples
///
/// ```
/// # use vmm::*;
/// let vec = vec4![1.0, 2.0, 3.0, 4.0];
/// 
/// assert_eq!(vec.to_arr(), &[1.0, 2.0, 3.0, 4.0]);
/// ```
///
/// # Notes
///
/// - The macro internally uses the `from_array` function to create the vector.
/// - Ensure that the provided expressions are suitable for initializing a 4D vector.
/// - The resulting `Vec4` struct will be created using the `from_array` function.
///
/// # See Also
///
/// - [`Vec4`](super::vectors::Vec4): The 4D vector type used by this macro.
/// - [`from_array`](super::vectors::VecN::from_array): Function to construct a vector from an array.
#[macro_export]
macro_rules! vec4 
{
    ($($e:expr),*) => (vmm::Vec4::from_array(&[$($e),*]));
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
/// - [`Mat2`](super::matrices::Mat2): The 2x2 matrix type used by this macro.
/// - [`from_mat_vec`](super::matrices::MatN::from_mat_vec): Function to construct a matrix from an array of Vec2.
#[macro_export]
macro_rules! mat2 
{
    ($($e:expr),*) => (vmm::Mat2::from_mat_vec(&[$($e),*]));
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
/// - [`Mat2`](super::matrices::Mat2): The 2x2 matrix type used by this macro.
/// - [`from_mat`](super::matrices::MatN::from_mat): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat2_raw 
{
    ($($e:expr),*) => (vmm::Mat2::from_mat(&[$($e),*]));
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
/// - [`Mat3`](super::matrices::Mat3): The 3x3 matrix type used by this macro.
/// - [`from_mat_vec`](super::matrices::MatN::from_mat_vec): Function to construct a matrix from an array of Vec3.
#[macro_export]
macro_rules! mat3 
{
    ($($e:expr),*) => (vmm::Mat3::from_mat_vec(&[$($e),*]));
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
/// - [`Mat3`](super::matrices::Mat3): The 3x3 matrix type used by this macro.
/// - [`from_mat`](super::matrices::MatN::from_mat): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat3_raw 
{
    ($($e:expr),*) => (vmm::Mat3::from_mat(&[$($e),*]));
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
/// - [`Mat4`](super::matrices::Mat4): The 4x4 matrix type used by this macro.
/// - [`from_mat_vec`](super::matrices::MatN::from_mat_vec): Function to construct a matrix from an array of Vec4.
#[macro_export]
macro_rules! mat4 
{
    ($($e:expr),*) => (vmm::Mat4::from_mat_vec(&[$($e),*]));
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
/// - [`Mat4`](super::matrices::Mat4): The 4x4 matrix type used by this macro.
/// - [`from_mat`](super::matrices::MatN::from_mat): Function to construct a matrix from a 2D array of elements.
#[macro_export]
macro_rules! mat4_raw 
{
    ($($e:expr),*) => (vmm::Mat4::from_mat(&[$($e),*]));
}