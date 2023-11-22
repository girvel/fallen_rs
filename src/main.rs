mod vector;  // TODO figure out what to do with this thing


struct Named {
    name: String,
}

struct Entity {
    named: Named,
}

trait HasComponent<T> {
    fn get_component(&self) -> &T;
}

impl HasComponent<Named> for Entity {
    fn get_component(&self) -> &Named {
        &self.named
    }
}

// TODO print name and position
// TODO print for multiple kinds of entities
// TODO print for multiple types

fn print_name<T: HasComponent<Named>>(entity: &T) {
    println!("{}", entity.get_component().name);
}


fn main() {
    let e = Entity {
        named: Named { name: String::from("Hugh") },
    };

    print_name(&e);
}
