use bevy::{
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};
use bevy_renet::renet::{ClientId, RenetServer};
use shared::{
    messages::{NetworkPlayerInput, PlayerInputs, PlayerUpdateEvent},
    players::{
        collision::check_player_collision,
        constants::{GRAVITY, JUMP_VELOCITY},
    },
    world::{ServerWorldMap, WorldSeed},
};

use crate::{network::extensions::SendGameMessageExtension, world::generation::generate_chunk};

use super::broadcast_world::get_all_active_chunks;

#[derive(Event, Debug)]
pub struct PlayerInputsEvent {
    #[allow(dead_code)]
    pub client_id: ClientId,
    pub inputs: PlayerInputs,
}

pub fn handle_player_inputs_system(
    mut events: EventReader<PlayerInputsEvent>,
    mut world_map: ResMut<ServerWorldMap>,
    mut server: ResMut<RenetServer>,
    seed: Res<WorldSeed>,
) {
    let active_chunks = get_all_active_chunks(&world_map);
    for c in active_chunks {
        let chunk = world_map.map.get(&c);

        if chunk.is_none() {
            let chunk = generate_chunk(c, seed.0);
            info!("Generated chunk: {:?}", c);
            world_map.map.insert(c, chunk);
        }
    }

    let world_clone = world_map.clone();

    let mut player_actions = HashMap::<u64, HashSet<NetworkPlayerInput>>::new();
    for client_id in world_map.players.keys() {
        player_actions.insert(*client_id, HashSet::new());
    }

    for ev in events.read() {
        let player = world_map.players.get_mut(&ev.client_id).unwrap();

        info!("Received player inputs: {:?} for {:?}", ev.inputs, player);

        let actions = player_actions.get_mut(&ev.client_id).unwrap();
        actions.extend(ev.inputs.actions.clone());
    }

    for p in player_actions.iter_mut() {
        let player = world_map.players.get_mut(p.0).unwrap();
        let initial_pos = player.position;
        let initial_rot = player.camera_transform.rotation;

        let mut is_jumping = false;

        let mut direction = Vec3::ZERO;

        for action in p.1.iter() {
            if *action == NetworkPlayerInput::ToggleFlyMode {
                player.is_flying = !player.is_flying;
            }

            match action {
                NetworkPlayerInput::CameraMovement(v) => {
                    debug!("Camera movement: {:?}", v);
                    player.camera_transform.rotation = v.clone().into();
                }
                NetworkPlayerInput::Jump => {
                    is_jumping = true;
                }
                _ => {}
            }

            // Calculate movement directions relative to the camera
            let mut forward = player.camera_transform.forward().xyz();
            forward.y = 0.0;

            let mut right = player.camera_transform.right().xyz();
            right.y = 0.0;

            // Adjust direction based on key presses
            if *action == NetworkPlayerInput::MoveBackward {
                direction -= forward;
            }
            if *action == NetworkPlayerInput::MoveForward {
                direction += forward;
            }
            if *action == NetworkPlayerInput::MoveLeft {
                direction -= right;
            }
            if *action == NetworkPlayerInput::MoveRight {
                direction += right;
            }
        }

        // Handle jumping (if on the ground) and gravity, only if not flying
        if !player.is_flying {
            if player.on_ground && is_jumping {
                // Player can jump only when grounded
                player.velocity.y = JUMP_VELOCITY;
                player.on_ground = false;
            } else if !player.on_ground {
                // Apply gravity when the player is in the air
                player.velocity.y += GRAVITY;
            }
        }

        let new_y = player.position.y + player.velocity.y;
        let new_vec = &Vec3::new(player.position.x, new_y, player.position.z);

        let max_velocity = 0.9;

        if player.velocity.y > max_velocity {
            player.velocity.y = max_velocity;
        }

        // Vérifier uniquement les collisions verticales (sol et plafond)
        if check_player_collision(new_vec, player, &world_clone) {
            // Si un bloc est détecté sous le joueur, il reste sur le bloc
            player.on_ground = true;
            player.velocity.y = 0.0; // Réinitialiser la vélocité verticale si le joueur est au sol
        } else {
            // Si aucun bloc n'est détecté sous le joueur, il continue de tomber
            player.position.y = new_y;
            player.on_ground = false;
        }

        // Attempt to move the player by the calculated direction
        let new_x = player.position.x + direction.x * 0.1;
        let new_z = player.position.z + direction.z * 0.1;

        let new_vec = &Vec3::new(new_x, player.position.y, new_z);
        if check_player_collision(new_vec, player, &world_clone) {
            // If a block is detected in the new position, don't move the player
        } else {
            player.position.x = new_x;
            player.position.z = new_z;
        }

        // If the player is below the world, reset their position
        const FALL_LIMIT: f32 = -50.0;
        if player.position.y < FALL_LIMIT {
            player.position = Vec3::new(0.0, 100.0, 0.0);
            player.velocity.y = 0.0;
        }

        let has_moved = player.position != initial_pos;
        let has_rotated = player.camera_transform.rotation != initial_rot;

        let requires_network_broadcast = has_moved || has_rotated;

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
