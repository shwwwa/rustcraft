use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MobKind {
    Fox,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerMob {
    pub id: u128,
    pub kind: MobKind,
    pub position: Vec3,
}
