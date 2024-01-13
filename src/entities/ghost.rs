use crate::components::{Displayed, Named, Positioned};
use crate::ecs::{AwareOfComponent, HasComponent};

pub struct Ghost {
    named: Named,
    positioned: Positioned,
}

impl Ghost {
    pub fn new(named: Named, positioned: Positioned) -> Self {
        Self { named, positioned }
    }
}

impl HasComponent<Named> for Ghost {
    fn get_component_raw(&mut self) -> &mut Named {
        &mut self.named
    }
}

impl HasComponent<Positioned> for Ghost {
    fn get_component_raw(&mut self) -> &mut Positioned {
        &mut self.positioned
    }
}

impl AwareOfComponent<Named> for Ghost {
    fn try_get_component_raw(&mut self) -> Option<&mut Named> {
        Some(&mut self.named)
    }
}

impl AwareOfComponent<Positioned> for Ghost {
    fn try_get_component_raw(&mut self) -> Option<&mut Positioned> {
        Some(&mut self.positioned)
    }
}

impl AwareOfComponent<Displayed> for Ghost {
    fn try_get_component_raw(&mut self) -> Option<&mut Displayed> {
        None
    }
}