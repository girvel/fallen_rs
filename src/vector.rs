use std::ops::Add;

#[derive(Debug)]  // TODO custom Debug/Display impl
pub struct Vector2<T>(pub T, pub T);

impl<R, T: Add<Output = R>> Add for Vector2<T> {
    type Output = Vector2<R>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
