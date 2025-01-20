use crate::input::keyboard::is_action_pressed;
use crate::KeyMap;
use crate::{input::data::GameAction, world::time::ClientTime};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use ordered_float::OrderedFloat;
use shared::messages::{CustomQuaternion, NetworkPlayerInput, PlayerInputs};

use super::SendGameMessageExtension;

pub fn upload_player_inputs_system(
    mut client: ResMut<RenetClient>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
    mut last_camera_orientation: Local<Option<Quat>>,
    camera: Query<&Transform, With<Camera>>,
    client_time: Res<ClientTime>,
) {
    let mut actions: Vec<NetworkPlayerInput> = vec![];
    if is_action_pressed(GameAction::MoveBackward, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::MoveBackward)
    }
    if is_action_pressed(GameAction::MoveForward, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::MoveForward)
    }
    if is_action_pressed(GameAction::MoveLeft, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::MoveLeft)
    }
    if is_action_pressed(GameAction::MoveRight, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::MoveRight)
    }
    if is_action_pressed(GameAction::Jump, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::Jump);
    }
    if is_action_pressed(GameAction::ToggleFlyMode, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::ToggleFlyMode)
    }
    if is_action_pressed(GameAction::FlyUp, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::FlyUp);
    }
    if is_action_pressed(GameAction::FlyDown, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::FlyDown);
    }

    let camera_transform = camera.single();
    let camera_orientation = camera_transform.rotation;

    if let Some(last_cam) = last_camera_orientation.as_mut() {
        if *last_cam != camera_orientation {
            actions.push(NetworkPlayerInput::CameraMovement(CustomQuaternion {
                x: OrderedFloat(camera_orientation.x),
                y: OrderedFloat(camera_orientation.y),
                z: OrderedFloat(camera_orientation.z),
                w: OrderedFloat(camera_orientation.w),
            }));
            *last_camera_orientation = Some(camera_orientation);
        }
    } else {
        *last_camera_orientation = Some(camera_orientation);
    }

    let msg = PlayerInputs {
        tick: client_time.0,
        actions,
    };
    if !msg.actions.is_empty() {
        // debug!("Sending player inputs: {:?}", msg);
        client.send_game_message(msg.into());
    }
}
