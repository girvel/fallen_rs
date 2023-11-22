use crate::ecs::AwareOfComponent;
use crate::vector::Vector2;

pub struct Named {
    pub name: String,
}

pub struct Positioned {
    pub position: Vector2<i32>,
}

pub struct Displayed {
    pub character: String,
}

pub trait Entity:
    AwareOfComponent<Named> +
    AwareOfComponent<Positioned> +
    AwareOfComponent<Displayed> {}

impl<
    T: AwareOfComponent<Named> +
    AwareOfComponent<Positioned> +
    AwareOfComponent<Displayed>
> Entity for T {}