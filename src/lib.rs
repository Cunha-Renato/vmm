//! # VMM 
//!
//! `vmm` is a Rust crate that provides a collection of types and utilities for mathematical
//! operations, specifically tailored for vectors and matrices. The library is designed to be
//! generic, allowing users to work with vectors and matrices of different dimensions and element types.
//!
//! ## Features
//!
//! - **VecN:** Generic vector type representing an array of elements of type `T` with a fixed size.
//! - **MatN:** Generic matrix type representing a 2D array of elements of type `T` with fixed dimensions.
//! - **Vector Operations:** Methods for common vector operations such as dot product, cross product, normalization, etc.
//! - **Matrix Operations:** Functions for matrix operations, including transpose, matrix-vector multiplication, and more.
//! - **Concise Macros:** Macros for creating vectors and matrices with a concise syntax.
//!
//! ## Examples
//!
//! ```rust
//! # use vmm::*;
//!
//! // Creating a 2D vector
//! let vector_2d = Vec2::<i32>::new();
//! let vec_2d = vec2![1.0, 2.0];
//!
//! // Creating a 3x3 matrix
//! let mat_3x3 = Mat3::<i32>::new();
//! let matrix_3x3 = mat3_raw![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
//!
//! // Performing vector operations
//! let dot_product = vec3![1.0, 2.0, 3.0].dot(&vec3![4.0, 5.0, 6.0]);
//! ```
//!
//! ## Usage
//!
//! Add the following line to your `Cargo.toml` file to include the `my_library` crate in your project:
//!
//! ```toml
//! [dependencies]
//! my_library = "0.1.0"
//! ```
//!
//! Then, you can use the types and functions from the library in your Rust code.
//!
//! ## License
//!
//! This crate is licensed under the MIT License - see the [LICENSE](https://github.com/your_username/my_library/blob/main/LICENSE) file for details.

pub mod types;
pub mod utils;
pub use utils::*;
pub use types::*;