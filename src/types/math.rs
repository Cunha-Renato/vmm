pub trait ScalarMath<T>
{
    /// Adds a scalar value to each element of self.
    ///
    /// # Arguments
    ///
    /// * `value` - The scalar value to be added to each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new Self with the same dimensions as the original, where each element is the sum
    /// of the corresponding element in the original self and the provided scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let matrix = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    /// let result = matrix.sum_scalar(2.0);
    /// assert_eq!(result, mat2_raw![[3.0, 4.0], [5.0, 6.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The original self remains unchanged; a new Self is returned with the updated values.
    /// - This method relies on the `Clone` trait, so the elements of self must implement `Clone`.
    ///
    /// # See Also
    ///
    /// - [`Mat2`](struct.Mat2.html): The matrix type used by the example.
    /// - [`Vec2`](struct.Vec2.html): The vector type representing rows or columns of the matrix.
    fn sum_scalar(&self, value: T) -> Self;
    /// Subtracts a scalar value to each element of self.
    ///
    /// # Arguments
    ///
    /// * `value` - The scalar value to be subtracted to each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new Self with the same dimensions as the original, where each element is the subtraction
    /// of the corresponding element in the original self and the provided scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let matrix = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    /// let result = matrix.sub_scalar(2.0);
    /// assert_eq!(result, mat2_raw![[-1.0, 0.0], [1.0, 2.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The original self remains unchanged; a new Self is returned with the updated values.
    /// - This method relies on the `Clone` trait, so the elements of self must implement `Clone`.
    ///
    /// # See Also
    ///
    /// - [`Mat2`](struct.Mat2.html): The matrix type used by the example.
    /// - [`Vec2`](struct.Vec2.html): The vector type representing rows or columns of the matrix.
    fn sub_scalar(&self, value: T) -> Self;
    /// Multiply each element of self by a scalar value.
    ///
    /// # Arguments
    ///
    /// * `value` - The scalar value to multiply each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new Self with the same dimensions as the original, where each element is the multiplication
    /// of the corresponding element in the original self by the provided scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let matrix = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    /// let result = matrix.mul_scalar(2.0);
    /// assert_eq!(result, mat2_raw![[2.0, 4.0], [6.0, 8.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The original self remains unchanged; a new Self is returned with the updated values.
    /// - This method relies on the `Clone` trait, so the elements of self must implement `Clone`.
    ///
    /// # See Also
    ///
    /// - [`Mat2`](struct.Mat2.html): The matrix type used by the example.
    /// - [`Vec2`](struct.Vec2.html): The vector type representing rows or columns of the matrix.
    fn mul_scalar(&self, value: T) -> Self;
    /// Divides each element of self by a scalar value.
    ///
    /// # Arguments
    ///
    /// * `value` - The scalar value to divide each element of the matrix.
    ///
    /// # Returns
    ///
    /// A new Self with the same dimensions as the original, where each element is the division 
    /// of the corresponding element in the original self by the provided scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vmm::*;
    /// let matrix = mat2_raw![[1.0, 2.0], [3.0, 4.0]];
    /// let result = matrix.div_scalar(2.0);
    /// assert_eq!(result, mat2_raw![[0.5, 1.0], [1.5, 2.0]]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The original self remains unchanged; a new Self is returned with the updated values.
    /// - This method relies on the `Clone` trait, so the elements of self must implement `Clone`.
    ///
    /// # See Also
    ///
    /// - [`Mat2`](struct.Mat2.html): The matrix type used by the example.
    /// - [`Vec2`](struct.Vec2.html): The vector type representing rows or columns of the matrix.
    fn div_scalar(&self, value: T) -> Self;
}