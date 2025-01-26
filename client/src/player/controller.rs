use crate::input::data::GameAction;
use crate::input::keyboard::*;
use crate::player::ViewMode;
use crate::ui::hud::debug::DebugOptions;
use crate::ui::hud::UIMode;
use crate::world::{ClientWorldMap, WorldRenderRequestUpdateEvent};
use crate::KeyMap;
use bevy::prelude::*;
use shared::players::Player;

use super::CurrentPlayerMarker;

#[derive(Component)]
pub struct PlayerMaterialHandle {
    pub handle: Handle<StandardMaterial>,
}

pub fn player_movement_system(
    queries: Query<&mut Player, With<CurrentPlayerMarker>>,
    resources: (Res<ButtonInput<KeyCode>>, Res<UIMode>, Res<KeyMap>),
) {
    let mut player_query = queries;
    let (keyboard_input, ui_mode, key_map) = resources;

    let res = player_query.get_single_mut();
    // Return early if the player has not been spawned yet
    if res.is_err() {
        debug!("player not found");
        return;
    }

    let mut player = player_query.single_mut();

    if *ui_mode == UIMode::Closed
        && is_action_just_pressed(GameAction::ToggleFlyMode, &keyboard_input, &key_map)
    {
        player.toggle_fly_mode();
    }
}

pub fn first_and_third_person_view_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut view_mode: ResMut<ViewMode>,
    mut player_query: Query<&mut PlayerMaterialHandle, With<CurrentPlayerMarker>>,
    key_map: Res<KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ui_mode: Res<UIMode>,
) {
    if *ui_mode == UIMode::Closed
        && is_action_just_pressed(GameAction::ToggleViewMode, &keyboard_input, &key_map)
    {
        view_mode.toggle();
    }

    let material_handle = player_query.get_single_mut();
    // Return early if the player has not been spawned yet
    if material_handle.is_err() {
        debug!("player not found");
        return;
    }

    let material_handle = &material_handle.unwrap().handle;

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

pub fn toggle_chunk_debug_mode_system(
    mut debug_options: ResMut<DebugOptions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
) {
    if is_action_just_pressed(GameAction::ToggleChunkDebugMode, &keyboard_input, &key_map) {
        debug_options.toggle_chunk_debug_mode();
    }
}

pub fn chunk_force_reload_system(
    mut world_map: ResMut<ClientWorldMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
    mut ev_writer: EventWriter<WorldRenderRequestUpdateEvent>,
    mut commands: Commands,
) {
    if is_action_just_pressed(GameAction::ReloadChunks, &keyboard_input, &key_map) {
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
}
