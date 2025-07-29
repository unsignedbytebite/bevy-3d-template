#![allow(warnings)]

#[cfg(feature = "dev_native")]
mod dev_utils;
mod game;
mod game_parameters;

use avian3d::prelude::*;
use bevy::{math::VectorSpace, prelude::*, text::FontSmoothing};

const CLEAR_COLOUR: Color = Color::hsv(219.0, 0.58, 0.93);
const WINDOW_RESOLUTION: (u16, u16) = (1280, 720);
const TITLE: &str = "{{project-name}}";
const APP_NAME: &str = "{{project-name}}";
const RESIZABLE: bool = true;
const MOUSE_VISIBLE: bool = true;
const LOG_GLTF_NODES: bool = true;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainAppState {
    #[default]
    Gameplay,
}

fn windows_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: TITLE.into(),
            name: Some(APP_NAME.into()),
            resolution: WINDOW_RESOLUTION.into(),
            resizable: RESIZABLE,
            mode: bevy::window::WindowMode::Windowed,
            present_mode: bevy::window::PresentMode::AutoVsync,
            cursor_options: bevy::window::CursorOptions {
                visible: MOUSE_VISIBLE,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(windows_settings()).set(AssetPlugin {
            #[cfg(feature = "dev_native")]
            watch_for_changes_override: Some(true),
            ..default()
        }),
        PhysicsPlugins::default(),
        game::plugin::Game::new(),
    ));
    #[cfg(feature = "dev_native")]
    dev_utils::register(&mut app);

    app.add_systems(Update, exit);

    app.insert_resource(ClearColor(CLEAR_COLOUR));

    game_parameters::register(&mut app);

    app.run();
}

/// A quick exit
fn exit(keyboard: Res<ButtonInput<KeyCode>>, mut ev_exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        ev_exit.write(AppExit::Success);
    }
}
