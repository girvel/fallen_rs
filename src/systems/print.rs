use crate::components::{Displayed, Named, Positioned};
use crate::ecs::{AwareOfComponent, component, HasComponent, try_component};


// ECS better syntax like has_components!(Named, Positioned)
pub fn print_name_and_position<
    T: HasComponent<Named> + HasComponent<Positioned> +
    AwareOfComponent<Named> + AwareOfComponent<Positioned> + AwareOfComponent<Displayed>
>(entity: &T) {
    let postfix = try_component::<Displayed, _>(entity)
        .map_or(String::from(""), |displayed| format!(" ({})", displayed.character));

    println!(
        "{}{} at {}",
        component::<Named, _>(entity).name,
        postfix,
        component::<Positioned, _>(entity).position,
    );
}