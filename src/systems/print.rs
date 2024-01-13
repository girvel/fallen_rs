use crate::components::{Displayed, Entity, Named, Positioned};
use crate::ecs::{CanRegister, component, HasComponent, try_component};
use crate::entities::ghost::Ghost;
use crate::entities::player::Player;


fn print_name_and_position<
    T: HasComponent<Named> + HasComponent<Positioned> + Entity
>(entity: &mut T) {
    let postfix = try_component::<Displayed, _>(entity)
        .map_or(String::from(""), |displayed| format!(" ({})", displayed.character));

    component::<Positioned, _>(entity).position.1 += 1;

    println!(
        "{}{} at {}",
        component::<Named, _>(entity).name,
        postfix,
        component::<Positioned, _>(entity).position,
    );
}


// ECS: autogenerated section //

pub struct PrintSystem {
    players: Vec<Player>,
    ghosts: Vec<Ghost>,
}

impl PrintSystem {
    pub fn new() -> Self {
        Self {
            players: vec![],
            ghosts: vec![]
        }
    }

    pub fn update(&mut self) {
        for player in &mut self.players {
            print_name_and_position(player);
        }

        for ghost in &mut self.ghosts {
            print_name_and_position(ghost);
        }
    }
}

// TODO pointer to entity to register in multiple systems
impl CanRegister<Player> for PrintSystem {
    fn register(&mut self, entity: Player) {
        self.players.push(entity);
    }
}

impl CanRegister<Ghost> for PrintSystem {
    fn register(&mut self, entity: Ghost) {
        self.ghosts.push(entity);
    }
}