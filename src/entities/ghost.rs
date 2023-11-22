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
    fn get_component_raw(&self) -> &Named {
        &self.named
    }
}

impl HasComponent<Positioned> for Ghost {
    fn get_component_raw(&self) -> &Positioned {
        &self.positioned
    }
}

impl AwareOfComponent<Named> for Ghost {
    fn try_get_component_raw(&self) -> Option<&Named> {
        Some(&self.named)
    }
}

impl AwareOfComponent<Positioned> for Ghost {
    fn try_get_component_raw(&self) -> Option<&Positioned> {
        Some(&self.positioned)
    }
}

impl AwareOfComponent<Displayed> for Ghost {
    fn try_get_component_raw(&self) -> Option<&Displayed> {
        None
    }
}