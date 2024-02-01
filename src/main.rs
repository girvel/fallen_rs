use crate::components::{Displayed, Named, Positioned};
use crate::ecs::CanRegister;
use crate::entities::{Ghost, Player};
use crate::systems::pseudo_gravity::GravitySystem;
use crate::vector::Vector2;
use std::time::Instant;

mod vector;
mod ecs;
mod components;
mod entities;
mod systems;  // TODO figure out what to do with this thing


// TODO mutable components
// TODO multiple systems
// TODO third system that accepts only one of the entity types

fn main() {
    let mut gravity_system = GravitySystem::new();

    gravity_system.register(Player::new(
        Named { name: String::from("Hugh") },
        Positioned { position: Vector2(42, 0) },
        Displayed { character: String::from("@") },
    ));
    
    for i in 0..1_000_000 {
        gravity_system.register(Ghost::new(
            Named { name: format!("Ghost #{}", i) },
            Positioned { position: Vector2(i, 0) },
        ));
    }

    let now = Instant::now();
    gravity_system.update();
    println!("dt: {:.2?}", now.elapsed());
}
