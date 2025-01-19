use bevy::prelude::*;
use shared::messages::mob::MobUpdateEvent;

use crate::mob::setup_fox;

use super::MobRoot;

pub fn spawn_mobs_system(
    mut ev_update: EventReader<MobUpdateEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut mobs: Query<(&MobRoot, &mut Transform)>,
) {
    'event_loop: for event in ev_update.read() {
        info!("--- SHOULD SPAWN OR UPDATE MOB ON CLIENT: {:?}", event);
        let id = event.mob.id;

        let position = event.mob.position;

        for (mob, mut transform) in mobs.iter_mut() {
            if mob.id == id {
                info!(
                    "Mob already exists, updating: id={:?}, position={:?}",
                    id, position
                );
                transform.translation = position;
                continue 'event_loop;
            }
        }

        if event.mob.kind == shared::world::MobKind::Fox {
            setup_fox(id, position, &mut commands, &asset_server, &mut graphs);
        }
    }
}
