use crate::{
    network::{CurrentPlayerProfile, TargetServer, TargetServerState},
    player::{PlayerLabel, PlayerMaterialHandle},
    GameState,
};
use bevy::color::palettes::css::ORANGE;
use bevy::prelude::*;
use shared::messages::{PlayerId, PlayerSpawnEvent};

#[derive(Component, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub vertical_velocity: f32,
    pub on_ground: bool,
    // pub view_mode: ViewMode,
    // pub is_chunk_debug_mode_enabled: bool,
    pub is_flying: bool,
    // pub inventory: HashMap<RegistryId, items::Item>,
    pub height: f32,
    pub width: f32,
}

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

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self {
            id,
            name,
            vertical_velocity: 0.0,
            on_ground: true,
            is_flying: false,
            height: 1.8,
            width: 0.8,
        }
    }

    pub fn toggle_fly_mode(&mut self) {
        self.is_flying = !self.is_flying;
        self.vertical_velocity = 0.0; // Réinitialisation de la vélocité
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
        let player = Player::new(event.id, event.name.clone());

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
