use bevy::prelude::*;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use super::{ClientToServerMessage, PlayerId};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash, Eq)]
pub struct CustomQuaternion {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
    pub z: OrderedFloat<f32>,
    pub w: OrderedFloat<f32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
pub enum NetworkPlayerInput {
    MoveForward,
    MoveRight,
    MoveBackward,
    MoveLeft,
    Jump,
    ToggleFlyMode,
    FlyUp,
    FlyDown,
    CameraMovement(CustomQuaternion),
}

impl From<CustomQuaternion> for Quat {
    fn from(val: CustomQuaternion) -> Self {
        Quat::from_xyzw(val.x.into(), val.y.into(), val.z.into(), val.w.into())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlayerInputs {
    pub tick: u64,
    pub actions: Vec<NetworkPlayerInput>,
}

impl From<PlayerInputs> for ClientToServerMessage {
    fn from(val: PlayerInputs) -> Self {
        ClientToServerMessage::PlayerInputs(val)
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
}
