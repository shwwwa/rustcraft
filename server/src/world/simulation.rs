use bevy::{
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};
use bevy_renet::renet::{ClientId, RenetServer};
use shared::{
    messages::{NetworkAction, PlayerFrameInput, PlayerUpdateEvent},
    players::movement::simulate_player_movement,
    world::{ServerWorldMap, WorldSeed},
};

use crate::{network::extensions::SendGameMessageExtension, world::generation::generate_chunk};

use super::broadcast_world::get_all_active_chunks;

#[derive(Event, Debug)]
pub struct PlayerInputsEvent {
    pub client_id: ClientId,
    pub input: PlayerFrameInput,
}

pub fn handle_player_inputs_system(
    mut events: EventReader<PlayerInputsEvent>,
    mut world_map: ResMut<ServerWorldMap>,
    mut server: ResMut<RenetServer>,
    seed: Res<WorldSeed>,
) {
    let active_chunks = get_all_active_chunks(&world_map, 1);
    for c in active_chunks {
        let chunk = world_map.map.get(&c);

        if chunk.is_none() {
            let chunk = generate_chunk(c, seed.0);
            info!("Generated chunk: {:?}", c);
            world_map.map.insert(c, chunk);
        }
    }

    let world_clone = world_map.clone();

    let mut player_actions = HashMap::<u64, HashSet<NetworkAction>>::new();
    for client_id in world_map.players.keys() {
        player_actions.insert(*client_id, HashSet::new());
    }

    for ev in events.read() {
        let player = world_map.players.get_mut(&ev.client_id).unwrap();
        // info!("Received player inputs: {:?} for {:?}", ev.input, player);

        let requires_network_broadcast =
            simulate_player_movement(player, &world_clone, &ev.input.clone(), 17);

        if requires_network_broadcast {
            server.broadcast_game_message(shared::messages::ServerToClientMessage::PlayerUpdate(
                PlayerUpdateEvent {
                    id: player.id,
                    position: player.position,
                    orientation: player.camera_transform.rotation,
                },
            ));
        }
    }
}
