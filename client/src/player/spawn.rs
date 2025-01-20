use crate::{
    network::{CurrentPlayerProfile, TargetServer, TargetServerState},
    player::{PlayerLabel, PlayerMaterialHandle},
    GameState,
};
use bevy::color::palettes::css::ORANGE;
use bevy::prelude::*;
use shared::{
    messages::{PlayerSpawnEvent, PlayerUpdateEvent},
    players::Player,
};

#[derive(Component)]
pub struct CurrentPlayerMarker {}

#[derive(Debug, PartialEq, Clone, Copy, Resource)]
pub enum ViewMode {
    FirstPerson,
    ThirdPerson,
}

impl ViewMode {
    pub fn toggle(&mut self) {
        *self = match *self {
            ViewMode::FirstPerson => ViewMode::ThirdPerson,
            ViewMode::ThirdPerson => ViewMode::FirstPerson,
        };
    }
}

pub const PLAYER_LABEL_FONT_SIZE: f32 = 24.0;

pub fn spawn_players_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_profile: Res<CurrentPlayerProfile>,
    mut ev_spawn: EventReader<PlayerSpawnEvent>,
    mut target_server: ResMut<TargetServer>,
    players: Query<&Player>,
    assets: Res<AssetServer>,
) {
    let current_id = player_profile.into_inner().id;
    let spawn_coords = Vec3::new(7.5, 80.0, 7.5);
    'event_loop: for event in ev_spawn.read() {
        info!("Executing spawn player for event: {:?}", event);
        for player in players.iter() {
            if player.id == event.id {
                info!(
                    "Ignored spawn order, player was already there: {}",
                    player.id
                );
                continue 'event_loop;
            }
        }
        let is_current_player = event.id == current_id;
        let player = Player::new(event.id, event.name.clone(), spawn_coords);

        let color = if is_current_player {
            Color::srgba(1.0, 0.0, 0.0, 1.0)
        } else {
            Color::srgba(0.0, 0.0, 1.0, 1.0)
        };

        info!("Spawning new player object: {}", player.id);

        let player_name = event.name.clone();

        let mut entity = commands.spawn((
            StateScoped(GameState::Game),
            Transform::from_translation(spawn_coords),
            Visibility::default(),
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                player.width,
                player.height,
                player.width,
            )))),
            MeshMaterial3d(materials.add(color)),
            PlayerMaterialHandle {
                handle: materials.add(color),
            },
            player.clone(),
            Name::new(player_name.clone()),
        ));

        // We need the full version of this font so we can use box drawing characters.
        let text_style = TextFont {
            font: assets.load("fonts/FiraMono-Medium.ttf"),
            font_size: PLAYER_LABEL_FONT_SIZE,
            ..default()
        };

        let label_text_style = (text_style.clone(), TextColor(ORANGE.into()));

        if is_current_player {
            target_server.state = TargetServerState::FullyReady;
            entity.insert(CurrentPlayerMarker {});
            info!("Inserted current player marker");
        }

        let entity_id = entity.id();

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                PlayerLabel {
                    entity: entity_id,
                    name: player_name.clone(),
                },
                Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
                StateScoped(GameState::Game),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(player_name),
                    label_text_style.clone(),
                    TextLayout::default().with_no_wrap(),
                ));
            });
    }
}

pub fn update_players_system(
    mut players: Query<(&Player, &mut Transform), With<Player>>,
    mut ev_player_update: EventReader<PlayerUpdateEvent>,
) {
    for event in ev_player_update.read() {
        for (player, mut transform) in players.iter_mut() {
            if player.id == event.id {
                transform.translation = event.position;
            }
        }
    }
}
