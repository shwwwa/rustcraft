use shared::{messages::PlayerId, world::ServerWorldMap};

pub fn cleanup_all_players_from_world(world_map: &mut ServerWorldMap) {
    world_map.players.clear();
    for (_, chunk) in world_map.chunks.map.iter_mut() {
        chunk.sent_to_clients.clear();
    }
}

pub fn cleanup_player_from_world(world_map: &mut ServerWorldMap, player_id: &PlayerId) {
    world_map.players.remove(player_id);
    for (_, chunk) in world_map.chunks.map.iter_mut() {
        chunk.sent_to_clients.retain(|&id| id != *player_id);
    }
}
