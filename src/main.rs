use vmm::{MatMN, Vec2};

fn main() {
    let a = MatMN::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);

    println!("{:#?}", a.to_vec());
    println!("{:#?}", a.transpose().to_vec())
}
