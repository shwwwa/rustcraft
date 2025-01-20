use crate::input::data::GameAction;
use crate::input::keyboard::*;
use crate::network::buffered_client::BufferedInputs;
use crate::player::ViewMode;
use crate::ui::hud::debug::DebugOptions;
use crate::ui::hud::UIMode;
use crate::world::{ClientWorldMap, WorldRenderRequestUpdateEvent};
use crate::KeyMap;
use bevy::prelude::*;
use shared::messages::NetworkPlayerInput;
use shared::players::Player;

use super::CurrentPlayerMarker;

#[derive(Component)]
pub struct PlayerMaterialHandle {
    pub handle: Handle<StandardMaterial>,
}

pub fn player_inputs_handling_system(
    queries: Query<(&mut Player, &mut PlayerMaterialHandle), With<CurrentPlayerMarker>>,
    resources: (
        Res<ButtonInput<KeyCode>>,
        Res<UIMode>,
        Res<KeyMap>,
        ResMut<Assets<StandardMaterial>>,
        ResMut<ClientWorldMap>,
        ResMut<ViewMode>,
        ResMut<DebugOptions>,
        ResMut<BufferedInputs>,
    ),
    mut commands: Commands,
    mut ev_writer: EventWriter<WorldRenderRequestUpdateEvent>,
) {
    let mut player_query = queries;
    let (
        keyboard_input,
        ui_mode,
        key_map,
        mut materials,
        mut world_map,
        mut view_mode,
        mut debug_options,
        mut buffer_client,
    ) = resources;

    let res = player_query.get_single_mut();
    // Return early if the player has not been spawned yet
    if res.is_err() {
        debug!("player not found");
        return;
    }

    let (mut player, material_handle) = player_query.single_mut();

    if *ui_mode == UIMode::Closed {
        if is_action_just_pressed(GameAction::ToggleViewMode, &keyboard_input, &key_map) {
            view_mode.toggle();
        }

        if is_action_just_pressed(GameAction::ToggleChunkDebugMode, &keyboard_input, &key_map) {
            debug_options.toggle_chunk_debug_mode();
        }

        // fly mode (f key)
        if is_action_just_pressed(GameAction::ToggleFlyMode, &keyboard_input, &key_map) {
            player.toggle_fly_mode();
            buffer_client
                .buffer
                .insert(NetworkPlayerInput::ToggleFlyMode);
        }
    }

    let force_chunk_reload =
        is_action_just_pressed(GameAction::ReloadChunks, &keyboard_input, &key_map);

    // If a re-render has been requested by the player
    if force_chunk_reload {
        // Send an event to re-render all chunks loaded
        for (pos, chunk) in world_map.map.iter_mut() {
            // Despawn the chunk's entity
            if let Some(e) = chunk.entity {
                commands.entity(e).despawn_recursive();
                chunk.entity = None;
            }
            // Request a render for this chunk
            ev_writer.send(WorldRenderRequestUpdateEvent::ChunkToReload(*pos));
        }
    }

    let material_handle = &material_handle.handle;
    match *view_mode {
        ViewMode::FirstPerson => {
            // make player transparent
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
        }
        ViewMode::ThirdPerson => {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(1.0, 0.0, 0.0, 1.0);
            }
        }
    }
}
