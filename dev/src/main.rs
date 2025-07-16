use vmm::{mul_other, transpose, transpose_square};

fn main() {
    let mut a = [1, 2, 5, 6];
    let mut b = [1, 2, 3, 4, 5, 6];

    let c = transpose_square(&mut a, 2);

    println!("{a:#?}");
}