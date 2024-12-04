use bevy::prelude::*;

use crate::ui::style::NORMAL_BUTTON;
use crate::TEXT_COLOR;

use super::{MenuButtonAction, MenuState};

pub fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the custom game title image
    let background_image = asset_server.load("./graphics/background.png");
    let title_image = asset_server.load("./graphics/title.png");
    let font: Handle<Font> = asset_server.load("./fonts/RustCraftRegular-Bmg3.otf");

    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(400.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 33.0,
        color: TEXT_COLOR,
        ..Default::default()
    };

    // Main container for the menu
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column, // Center everything vertically
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            },
            UiImage::new(background_image), // Set the background image
            StateScoped(MenuState::Main),
        ))
        .with_children(|parent| {
            // Display the game title as an image
            let image_width = 800.0;
            let aspect_ratio = 601.0 / 94.0;
            let image_height = image_width / aspect_ratio;

            parent.spawn(ImageBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(120.0)), // Add space below the title image
                    width: Val::Px(image_width),            // Width calculated using aspect ratio
                    height: Val::Px(image_height),          // Height based on desired size
                    ..Default::default()
                },
                image: UiImage::new(title_image),
                ..Default::default()
            });

            // Add buttons for each action available in the menu
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    MenuButtonAction::Solo,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Singleplayer",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    MenuButtonAction::Multi,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Multiplayer",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    MenuButtonAction::Settings,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}
