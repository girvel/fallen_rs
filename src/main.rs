use crate::components::{Displayed, Named, Positioned};
use crate::ecs::CanRegister;
use crate::entities::ghost::Ghost;
use crate::entities::player::Player;
use crate::systems::print::{PrintSystem};
use crate::vector::Vector2;

mod vector;
mod ecs;
mod components;
mod entities;
mod systems;  // TODO figure out what to do with this thing


// TODO mutable components
// TODO multiple systems

fn main() {
    let mut print_system = PrintSystem::new();

    print_system.register(Player::new(
        Named { name: String::from("Hugh") },
        Positioned { position: Vector2(42, 0) },
        Displayed { character: String::from("@") },
    ));

    print_system.register(Ghost::new(
        Named { name: String::from("Ghost 1") },
        Positioned { position: Vector2(0, 0) },
    ));

    print_system.register(Ghost::new(
        Named { name: String::from("Ghost 2") },
        Positioned { position: Vector2(0, 1) },
    ));

    print_system.register(Ghost::new(
        Named { name: String::from("Ghost 3") },
        Positioned { position: Vector2(0, 2) },
    ));

    print_system.update();
}
