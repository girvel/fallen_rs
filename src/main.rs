use crate::vector::Vector2;

mod vector;


fn main() {
    let v = Vector2(2, 2);
    let u = Vector2(-8, 3);

    println!("v + u is {:?}", v + u);
}
