use bevy::{prelude::*, utils::HashSet};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use super::PlayerId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash, Eq)]
pub struct CustomQuaternion {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
    pub z: OrderedFloat<f32>,
    pub w: OrderedFloat<f32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum NetworkAction {
    MoveForward,
    MoveRight,
    MoveBackward,
    MoveLeft,
    JumpOrFlyUp,
    SneakOrFlyDown,
    ToggleFlyMode,
}

impl From<CustomQuaternion> for Quat {
    fn from(val: CustomQuaternion) -> Self {
        Quat::from_xyzw(val.x.into(), val.y.into(), val.z.into(), val.w.into())
    }
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlayerSpawnEvent {
    pub id: PlayerId,
    pub name: String,
    pub position: Vec3,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlayerUpdateEvent {
    pub id: PlayerId,
    pub position: Vec3,
    pub orientation: Quat,
    pub last_ack_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PlayerFrameInput {
    pub time_ms: u64, // time of creation
    pub delta_ms: u64,
    pub inputs: HashSet<NetworkAction>,
    pub camera: Quat,
    #[serde(skip)]
    pub position: Vec3,
}
