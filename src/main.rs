use crate::components::{Displayed, Named, Positioned};
use crate::entities::ghost::Ghost;
use crate::entities::player::Player;
use crate::systems::print::print_name_and_position;
use crate::vector::Vector2;

mod vector;
mod ecs;
mod components;
mod entities;
mod systems;  // TODO figure out what to do with this thing


// TODO print for multiple entities
// TODO mutable components
// TODO split into files

fn main() {
    let p = Player::new(
        Named { name: String::from("Hugh") },
        Positioned { position: Vector2(42, 0) },
        Displayed { character: String::from("@") },
    );

    let g = Ghost::new(
        Named { name: String::from("Ghost") },
        Positioned { position: Vector2(0, 0) },
    );

    print_name_and_position(&p);
    print_name_and_position(&g);
}
