use ecs_macros::entity;
use crate::components::{Displayed, Named, Positioned};

#[entity(Named, Positioned, Displayed)]
pub struct Ghost(Named, Positioned);

