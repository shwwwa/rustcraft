use bevy::prelude::Component;

use crate::messages::PlayerId;

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
