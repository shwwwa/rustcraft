use crate::init::ServerTime;
use crate::network::extensions::SendGameMessageExtension;
use bevy::math::IVec3;
use bevy::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_renet::renet::RenetServer;
use shared::messages::{ItemStackUpdateEvent, ServerToClientMessage, WorldUpdate};
use shared::players::Player;
use shared::world::{world_position_to_chunk_position, ServerChunk, ServerWorldMap};
use std::collections::HashMap;

pub const BROADCAST_RENDER_DISTANCE: i32 = 2;

pub fn broadcast_world_state(
    mut server: ResMut<RenetServer>,
    time: Res<ServerTime>,
    mut world_map: ResMut<ServerWorldMap>,
) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    for c in get_all_active_chunks(&world_map, BROADCAST_RENDER_DISTANCE) {
        let chunk = world_map.map.get(&c);

        if chunk.is_none() {
            continue;
        }

        for client in server.clients_id().iter_mut() {
            let player = world_map.players.get_mut(client);
            let player = match player {
                Some(p) => p.clone(),
                None => continue,
            };
            let msg = WorldUpdate {
                tick: time.0,
                time: ts,
                player_positions: get_player_positions(&world_map),
                new_map: get_world_map_chunks_to_send(&mut world_map, &player),
                mobs: world_map.mobs.clone(),
                item_stacks: get_items_stacks(&world_map),
                player_events: vec![],
            };

            if msg.new_map.is_empty() {
                continue;
            }

            let message = ServerToClientMessage::WorldUpdate(msg);

            server.send_game_message(*client, message);
        }
    }
}

fn get_player_positions(world_map: &ServerWorldMap) -> HashMap<u64, Vec3> {
    let new_map = world_map
        .players
        .clone()
        .iter()
        .map(|(k, v)| (*k, v.position))
        .collect();

    new_map
}

fn get_world_map_chunks_to_send(
    world_map: &mut ServerWorldMap,
    player: &Player,
) -> HashMap<IVec3, ServerChunk> {
    // Send only chunks in render distance
    let mut map: HashMap<IVec3, ServerChunk> = HashMap::new();

    let active_chunks = get_all_active_chunks(world_map, BROADCAST_RENDER_DISTANCE);
    for c in active_chunks {
        if map.len() >= 10 {
            break;
        }

        let chunk = world_map.map.get_mut(&c);

        // If chunk already exists, transmit it to client
        if let Some(chunk) = chunk {
            if chunk.sent_to_clients.contains(&player.id) {
                continue;
            }

            map.insert(c, chunk.clone());
            chunk.sent_to_clients.push(player.id);
        }
    }

    map
}

fn get_items_stacks(world_map: &ServerWorldMap) -> Vec<ItemStackUpdateEvent> {
    world_map
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
        .collect()
}

pub fn get_all_active_chunks(world_map: &ServerWorldMap, radius: i32) -> Vec<IVec3> {
    let player_chunks: Vec<IVec3> = world_map
        .players
        .values()
        .map(|v| world_position_to_chunk_position(v.position))
        .flat_map(|v| get_player_nearby_chunks_coords(v, radius))
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

    // let's sort by distance to player
    chunks.sort_by_key(|&c| (c - player_chunk_position).length_squared());

    chunks
}
