use bevy::prelude::*;
use shared::world::{ServerMob, ServerWorldMap};
use ulid::Ulid;

use crate::init::ServerTime;

fn create_new_mob_id() -> u128 {
    Ulid::new().0
}

pub fn manage_mob_spawning_system(mut world_map: ResMut<ServerWorldMap>, time: Res<ServerTime>) {
    if time.0 == 300 {
        debug!("Should spawn mob");

        let id = create_new_mob_id();

        let position = Vec3::new(0.0, 90.0, 0.0);

        let mob = ServerMob {
            id,
            kind: shared::world::MobKind::Fox,
            position,
        };

        info!("Spawning new mob on server: {:?}", mob);

        world_map.mobs.push(mob);
    }
}
