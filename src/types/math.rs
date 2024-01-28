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
    /// - [`Mat2`](super::matrices::Mat2): The matrix type used by the example.
    /// - [`Vec2`](super::vectors::Vec2): The vector type representing rows or columns of the matrix.
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
    /// - [`Mat2`](super::matrices::Mat2): The matrix type used by the example.
    /// - [`Vec2`](super::vectors::Vec2): The vector type representing rows or columns of the matrix.
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
    /// - [`Mat2`](super::matrices::Mat2): The matrix type used by the example.
    /// - [`Vec2`](super::vectors::Vec2): The vector type representing rows or columns of the matrix.
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
    /// - [`Mat2`](super::matrices::Mat2): The matrix type used by the example.
    /// - [`Vec2`](super::vectors::Vec2): The vector type representing rows or columns of the matrix.
    fn div_scalar(&self, value: T) -> Self;
}

pub trait Sqrrt {
    fn sqrrt(&self) -> Self;
}
impl Sqrrt for f32 {
    fn sqrrt(&self) -> Self {
        self.sqrt()
    }
}
impl Sqrrt for f64 {
    fn sqrrt(&self) -> Self {
        self.sqrt()
    } 
}

pub trait UnitValue {
    fn unit_value() -> Self;
}
impl UnitValue for i8 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for i16 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for i32 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for i64 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for i128 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for isize {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for u8 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for u16 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for u32 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for u64 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for u128 {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for usize {
    fn unit_value() -> Self {
        1
    } 
}
impl UnitValue for f32 {
    fn unit_value() -> Self {
        1.0_f32
    } 
}
impl UnitValue for f64 {
    fn unit_value() -> Self {
        1.0_f64
    } 
}

pub trait SinCosTan {
    fn coss(&self) -> Self;
    fn sinn(&self) -> Self;
    fn tann(&self) -> Self;
}
impl SinCosTan for i8 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as i8
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as i8
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as i8
    }
}
impl SinCosTan for i16 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as i16
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as i16
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as i16
    }
}
impl SinCosTan for i32 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as i32
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as i32
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as i32
    }
}
impl SinCosTan for i64 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as i64
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as i64
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as i64
    }
}
impl SinCosTan for u8 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as u8
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as u8
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as u8
    }
}
impl SinCosTan for u16 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as u16
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as u16
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as u16
    }
}
impl SinCosTan for u32 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as u32
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as u32
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as u32
    }
}
impl SinCosTan for u64 {
    fn coss(&self) -> Self {
        (*self as f64).cos() as u64
    }
    fn sinn(&self) -> Self {
        (*self as f64).cos() as u64
    } 
    fn tann(&self) -> Self {
        (*self as f64).cos() as u64
    }
}
impl SinCosTan for f32 {
    fn coss(&self) -> Self {
        self.cos() 
    }
    fn sinn(&self) -> Self {
        self.sin()
    } 
    fn tann(&self) -> Self {
        self.tan()
    }
}
impl SinCosTan for f64 {
    fn coss(&self) -> Self {
        self.cos() 
    }
    fn sinn(&self) -> Self {
        self.sin()
    } 
    fn tann(&self) -> Self {
        self.tan()
    }
}