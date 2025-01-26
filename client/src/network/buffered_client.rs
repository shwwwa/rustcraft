use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};
use shared::messages::NetworkPlayerInput;

#[derive(Debug, Default, Resource)]
pub struct BufferedInputs {
    #[allow(dead_code)]
    pub buffer: Vec<PlayerFrameInputs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerFrameInputs {
    pub time_ms: u64,
    pub inputs: HashSet<NetworkPlayerInput>,
    pub camera: Quat,
}
