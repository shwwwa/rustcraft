use bevy::prelude::*;

mod fox;

pub use fox::*;

#[derive(Component)]
pub struct Mob {
    #[allow(dead_code)]
    pub name: String,
}
