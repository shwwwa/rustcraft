use bevy::{prelude::*, utils::HashSet};
use shared::messages::NetworkPlayerInput;

#[derive(Debug, Default, Resource)]
pub struct BufferedInputs {
    // represents the buffer of inputs of the current tick
    // these inputs will be flushed within the same tick (50ms)
    pub buffer: HashSet<NetworkPlayerInput>,
}
