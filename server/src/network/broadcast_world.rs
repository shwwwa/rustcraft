use crate::init::ServerTime;
use crate::network::extensions::SendGameMessageExtension;
use crate::world::generation::generate_chunk;
use bevy::math::IVec3;
use bevy::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_renet::renet::RenetServer;
use shared::messages::{ItemStackUpdateEvent, ServerToClientMessage, WorldUpdate};
use shared::world::{world_position_to_chunk_position, ServerChunk, ServerWorldMap};
use shared::TICKS_PER_SECOND;
use std::collections::HashMap;

use shared::world::data::WorldSeed;

pub fn broadcast_world_state(
    mut server: ResMut<RenetServer>,
    time: Res<ServerTime>,
    mut world_map: ResMut<ServerWorldMap>,
    seed: Res<WorldSeed>,
) {
    if time.0 % (2 * TICKS_PER_SECOND) != 0 {
        return;
    }

    info!("Broadcast world update at tick {}", time.0);
    let message = ServerToClientMessage::WorldUpdate(to_network(&mut world_map, seed, time.0));
    server.broadcast_game_message(message);
}

fn to_network(world_map: &mut ServerWorldMap, seed: Res<WorldSeed>, tick: u64) -> WorldUpdate {
    WorldUpdate {
        tick,
        player_positions: {
            let new_map = world_map
                .players
                .clone()
                .iter()
                .map(|(k, v)| (*k, v.position))
                .collect();

            new_map
        },
        new_map: {
            // let mut m: HashMap<IVec3, ServerChunk> = HashMap::new();
            // // Only send chunks that must be updated
            // for v in world_map.chunks_to_update.iter() {
            //     m.insert(*v, world_map.map.get(v).unwrap().clone());
            // }
            // // Chunks are up do date, clear the vector
            // world_map.chunks_to_update.clear();
            // m

            // Send all chunks
            //           let full_map = world_map.map.clone();
            //           info!("Sending full map: {:?}", full_map.keys());
            //           full_map

            // Send only chunks in render distance
            let mut map: HashMap<IVec3, ServerChunk> = HashMap::new();

            let active_chunks = get_all_active_chunks(world_map);
            for c in active_chunks {
                let chunk = world_map.map.get(&c);

                // If chunk already exists, transmit it to client
                if let Some(chunk) = chunk {
                    map.insert(c, chunk.clone());
                } else {
                    // If chunk does not exists, generate it before transmitting it
                    let chunk = generate_chunk(c, seed.0);

                    info!("Generated chunk: {:?}", c);

                    map.insert(c, chunk.clone());
                    world_map.map.insert(c, chunk);
                }
            }

            map
        },
        time: world_map.time,
        mobs: world_map.mobs.clone(),
        item_stacks: world_map
            .item_stacks
            .iter()
            .map(|stack| ItemStackUpdateEvent {
                id: stack.id,
                data: if stack.despawned {
                    None
                } else {
                    Some((stack.stack, stack.pos))
                },
            })
            .collect(),
    }
}

const RENDER_DISTANCE: i32 = 1;

fn get_all_active_chunks(world_map: &ServerWorldMap) -> Vec<IVec3> {
    let player_chunks: Vec<IVec3> = world_map
        .players
        .values()
        .map(|v| world_position_to_chunk_position(v.position))
        .flat_map(|v| get_player_nearby_chunks_coords(v, RENDER_DISTANCE))
        .collect();

    let mut chunks: Vec<IVec3> = Vec::new();

    for c in player_chunks {
        if !chunks.contains(&c) {
            chunks.push(c);
        }
    }

    chunks
}

fn get_player_nearby_chunks_coords(
    player_chunk_position: IVec3,
    render_distance: i32,
) -> Vec<IVec3> {
    let mut chunks: Vec<IVec3> = Vec::new();
    for x in -render_distance..=render_distance {
        for y in -render_distance..=render_distance {
            for z in -render_distance..=render_distance {
                chunks.push(player_chunk_position + IVec3::new(x, y, z));
            }
        }
    }

    chunks
}
