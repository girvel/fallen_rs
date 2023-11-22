#[derive(Debug)]  // TODO custom Debug/Display impl
struct Vector2<T>(T, T);


fn main() {
    let v = Vector2(2, 2);

    println!("v is {:?}", v);
}
