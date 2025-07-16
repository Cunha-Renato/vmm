#[cfg(feature = "rayon")]
extern crate rayon;

use std::{fmt::Debug, ops::*};

#[inline(always)]
pub const fn index(i: usize, j: usize, rows: usize) -> usize {
    i * rows + j
}

#[inline(always)]
fn linear_op<T, F>(a: &mut [T], mut op: F)
where
    F: FnMut(&mut T),
{
    a.iter_mut().for_each(|d| (op)(d));
}

#[inline(always)]
fn element_op<T, F>(a: &mut [T], b: &[T], mut op: F)
where
    F: FnMut(&mut T, &T),
{
    assert_eq!(a.len(), b.len(), "Both arrays must have the same lenght.");
    a.iter_mut().zip(b.iter()).for_each(|(a, b)| (op)(a, b));
}

#[inline(always)]
fn add_scalar<T>(a: &mut [T], scalar: T)
where
    T: Copy + AddAssign,
{
    linear_op(a, |d| *d += scalar);
}

#[inline(always)]
fn sub_scalar<T>(a: &mut [T], scalar: T)
where
    T: Copy + SubAssign,
{
    linear_op(a, |d| *d -= scalar);
}

#[inline(always)]
fn mul_scalar<T>(a: &mut [T], scalar: T)
where
    T: Copy + MulAssign,
{
    linear_op(a, |d| *d *= scalar);
}

#[inline(always)]
fn div_scalar<T>(a: &mut [T], scalar: T)
where
    T: Copy + DivAssign,
{
    linear_op(a, |d| *d /= scalar);
}

#[inline(always)]
fn add_other<T>(a: &mut [T], b: &[T])
where
    T: Copy + AddAssign,
{
    element_op(a, b, |a, b| *a += *b);
}

#[inline(always)]
fn sub_other<T>(a: &mut [T], b: &[T])
where
    T: Copy + SubAssign,
{
    element_op(a, b, |a, b| *a -= *b);
}

/// We assume that A is transpose, so the math is a little bit better.
pub fn mul_other<T>(a: &[T], a_rows: usize, a_cols: usize, b: &[T], b_rows: usize, b_cols: usize) -> Box<[T]>
where T: Copy + Mul<Output = T> + std::iter::Sum<T> + Debug,
{
    // Must be true;
    assert_eq!(a_rows, b_rows, "Cannot multiply {a_cols}x{a_rows} with {b_rows}x{b_cols}.");
    
    let mut result = Vec::with_capacity(a_cols * b_cols);

    for i in 0..b_cols {
        for j in 0..a_cols {
            let i = i * b_rows;
            let j = j * a_rows;

            let c = &a[j..j+a_rows];
            let d = &b[i..i+b_rows];
            
            result.push(dot(c, d));
        }
    }

    result.into()
}

/// We assume that A and B are vectors.
#[inline(always)]
fn dot<T>(a: &[T], b: &[T]) -> T
where T: Copy + Mul<Output = T> + std::iter::Sum<T>,
{
    a.iter().zip(b).map(|(a, b)| *a * *b).sum()
}

/// We assume that it is a vector;
#[inline(always)]
fn lenght_sq<T>(vec: &[T]) -> T
where T: Copy + Mul<Output = T> + std::iter::Sum<T>,
{
    dot(&vec, &vec)
}

#[inline]
pub fn transpose<T>(a: &[T], rows: usize, cols: usize) -> Box<[T]>
where T: Copy
{
    let mut result = Vec::with_capacity(a.len());
    for i in 0..rows {
        for j in 0..cols {
            let index = index(j, i, rows);            
            result.push(a[index]);
        }
    }
    
    result.into()
}

/// A must be square n == m.
#[inline]
pub fn transpose_square<T>(a: &mut [T], dim: usize) 
where T: Copy
{
    assert_eq!(a.len(), dim * dim);
    
    for i in 0..dim {
        for j in 0..i {
            a.swap(index(i, j, dim), index(j, i, dim));
        }
    }
}

#[derive(Debug, Default)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Box<[T]>,
}
impl<T> Matrix<T> {
    #[inline(always)]
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}