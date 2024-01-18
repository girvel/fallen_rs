use ecs_macros::entity;
use crate::components::{Displayed, Named, Positioned};
use crate::all_components;

#[entity(all_components!())]
pub struct Ghost(Named, Positioned);

#[entity(all_components!())]
pub struct Player(Named, Positioned, Displayed);
