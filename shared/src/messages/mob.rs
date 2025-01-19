use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::ServerMob;

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct MobUpdateEvent {
    pub mob: ServerMob,
}
