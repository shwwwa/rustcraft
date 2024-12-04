use bevy::prelude::*;
use bevy_simple_text_input::TextInputInactive;

use super::style::*;

#[derive(Component)]
pub struct ScrollingList {
    pub position: f32,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Solo,
    Multi,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsControls,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Solo,
    Multi,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsControls,
    #[default]
    Disabled,
}

// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    input_click_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<TextInputInactive>),
    >,
    mut text_input_query: Query<(Entity, &mut TextInputInactive)>,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }

    for (interaction_entity, interaction) in &input_click_query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive) in &mut text_input_query {
                inactive.0 = entity != interaction_entity;
            }
        }
    }
}
