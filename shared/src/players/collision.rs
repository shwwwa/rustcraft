use bevy::math::{bounding::Aabb3d, Vec3};

use crate::world::WorldMap;

use super::Player;

pub fn check_player_collision(
    player_position: &Vec3,
    player: &Player,
    world_map: &impl WorldMap,
) -> bool {
    world_map.check_collision_box(&Aabb3d::new(
        *player_position,
        Vec3::new(player.width, player.height, player.width) / 2.0,
    ))
}
