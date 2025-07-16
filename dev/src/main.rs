use vmm::{Matrix, MatrixOps};

fn main() {
    let mut a = Matrix::from_slice(2, 3, &[1, 2, 3 ,4, 5, 6]);
    println!("{a:#?}");
    a.transpose_assign();
    println!("{a:#?}");
}
