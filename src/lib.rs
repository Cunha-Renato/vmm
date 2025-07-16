#[cfg(feature = "rayon")]
extern crate rayon;

pub mod ops;

use num_traits::{NumAssign, NumOps};
pub use ops::*;

#[derive(Default, Debug)]
pub struct Matrix<T>
where
    T: Clone + NumOps + NumAssign
{
    data: Box<[T]>,
    rows: usize,
    cols: usize,
}
impl<T> From<Matrix<T>> for Box<[T]> 
where
    T: Clone + NumOps + NumAssign
{
    fn from(value: Matrix<T>) -> Self {
        value.data
    }
}
impl<T> AsRef<[T]> for Matrix<T> 
where
    T: Clone + NumOps + NumAssign
{
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}
impl<T> ScalarOps for Matrix<T>
where
    T: Clone + NumOps + NumAssign
{
    type Number = T;

    #[inline(always)]
    fn add_assign_scalar(&mut self, scalar: Self::Number) {
        self.data.add_assign_scalar(scalar);
    }

    #[inline(always)]
    fn sub_assign_scalar(&mut self, scalar: Self::Number) {
        self.data.sub_assign_scalar(scalar);
    }

    #[inline(always)]
    fn mul_assign_scalar(&mut self, scalar: Self::Number) {
        self.data.mul_assign_scalar(scalar);
    }

    #[inline(always)]
    fn div_assign_scalar(&mut self, scalar: Self::Number) {
        self.data.div_assign_scalar(scalar);
    }
}
impl<T> MatrixOps for Matrix<T> 
where
    T: Clone + NumOps + NumAssign
{
    fn transpose(&self) -> Self {
        let mut result = Self {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols,
        };
        result.transpose_assign();
        result
    }

    fn transpose_assign(&mut self) {
        
        if self.rows == self.cols {
            let data = unsafe { &mut *((&mut self.data as &mut [T]) as *mut [T]) };

            for col in 0..self.cols {
                for row in 0..self.rows {
                    if col == row { continue; }
                    let idx = self.index(row, col);
                    let inv_idx = index(col, row, row);
                    std::mem::swap(&mut self.data[idx], &mut data[inv_idx]);
                }
            }
        } else {
            let mut buf = Vec::with_capacity(self.rows * self.cols);
            for col in 0..self.cols {
                for row in 0..self.rows {
                    let idx = self.index(row, col);
                    buf.push(self.data[idx].clone());
                }
            }
            self.data = buf.into();
        }

        std::mem::swap(&mut self.rows, &mut self.cols);
    }

    fn add_assign(&mut self, rhs: &Self) {
        todo!()
    }

    fn sub_assign(&mut self, rhs: &Self) {
        todo!()
    }

    fn mul_assign(&mut self, rhs: &Self) {
        todo!()
    }
}
impl<T> Matrix<T> where T: Clone + NumOps + NumAssign {
    #[inline(always)]
    pub fn from_slice(rows: usize, cols: usize, data: &[T]) -> Self {
        Self {
            rows,
            cols,
            data: data.to_vec().into()
        }
    }
    
    #[inline(always)]
    const fn index(&self, row: usize, col: usize) -> usize {
        index(row, col, self.cols)
    }
    
    #[inline(always)]
    fn assert_eq_dim(&self, other: &Self) {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
    }

    #[inline(always)]
    fn assert_mul_dim(&self, other: &Self) {
        assert_eq!(self.rows, other.cols);
        assert_eq!(self.cols, other.rows);
    }
}

#[inline(always)]
const fn index(row: usize, col: usize, dim: usize) -> usize {
    row * dim + col
}


