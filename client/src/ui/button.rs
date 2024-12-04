use bevy::prelude::*;
use bevy_simple_text_input::TextInputInactive;

use super::{
    assets::{load_button_background_image, load_dark_button_background_image},
    style::*,
};

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

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiImage,
            &mut BackgroundColor,
            Option<&SelectedOption>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    input_click_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<TextInputInactive>),
    >,
    mut text_input_query: Query<(Entity, &mut TextInputInactive)>,
    asset_server: Res<AssetServer>, // Needed to load images
) {
    // Load button images
    let normal_image = load_button_background_image(&asset_server);
    let dark_image = load_dark_button_background_image(&asset_server);

    for (interaction, mut ui_image, mut background_color, selected) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                *ui_image = normal_image.clone().into(); // Optional: Keep the "normal" image while pressed
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                *ui_image = dark_image.clone().into(); // Switch to the "dark" image on hover
            }
            Interaction::None => {
                *background_color = match selected {
                    Some(_) => PRESSED_BUTTON.into(), // Selected buttons stay pressed
                    None => NORMAL_BUTTON.into(),
                };
                *ui_image = normal_image.clone().into(); // Default back to the normal image
            }
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
