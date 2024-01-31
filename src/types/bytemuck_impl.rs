/// This is file aims to integrate the types with the crate [bytemuck](https://crates.io/crates/bytemuck/)

use std::marker;
use bytemuck::Zeroable;
use super::{VecN, MatN};

unsafe impl<T, const N: usize> Zeroable for VecN<T, N> 
where 
    T: Default + marker::Copy,
    f64: From<T> {}

unsafe impl<T, const N: usize> bytemuck::Pod for VecN<T, N> 
where
    T: Default + marker::Copy + 'static,
    f64: From<T> {}

unsafe impl<T, const N: usize> Zeroable for MatN<T, N> 
where 
    T: Default + marker::Copy,
    f64: From<T> {}
unsafe impl<T, const N: usize> bytemuck::Pod for MatN<T, N>
where
    T: Default + marker::Copy + 'static,
    f64: From<T> {}