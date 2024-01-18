use crate::ecs::AwareOfComponent;
use crate::vector::Vector2;
use ecs_macros::{component, register_components};

#[component]
pub struct Named {
    pub name: String,
}

#[component]
pub struct Positioned {
    pub position: Vector2<i32>,
}

#[component]
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

register_components!();
