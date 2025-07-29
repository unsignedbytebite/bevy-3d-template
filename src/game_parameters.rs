use bevy::input::keyboard::KeyboardInput;
use bevy::{asset::io::AssetSourceEvent, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{egui::InputState, prelude::*};
use serde::{Deserialize, Serialize};

use crate::game_parameters;

#[derive(Default, Deserialize, Serialize, Reflect)]
pub struct GameParameterValues {
    pub camera_speed: f32,
    pub cube_reset_z: f32,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GameParameters {
    values: GameParameterValues,
}

impl GameParameters {
    pub fn values(&self) -> &GameParameterValues {
        &self.values
    }
}

impl Default for GameParameters {
    #[cfg(feature = "dev_native")]
    fn default() -> Self {
        match std::fs::read_to_string("./assets/game_parameters.toml") {
            Ok(content) => match toml::from_str::<GameParameterValues>(&content) {
                Ok(values) => Self { values },
                Err(err) => {
                    error!("Error loading ./assets/game_parameters.toml: {err:?}");
                    Self {
                        values: GameParameterValues::default(),
                    }
                }
            },
            Err(err) => {
                error!("Error loading game parameters: {err:?}");

                Self {
                    values: GameParameterValues::default(),
                }
            }
        }
    }

    #[cfg(not(feature = "dev_native"))]
    fn default() -> Self {
        let content = include_str!("../assets/game_parameters.toml");

        match toml::from_str::<GameParameterValues>(&content) {
            Ok(values) => Self { values },
            Err(err) => {
                error!("Error loading ./assets/game_parameters.toml: {err:?}");
                Self {
                    values: GameParameterValues::default(),
                }
            }
        }
    }
}

pub fn register(mut app: &mut App) {
    app.init_resource::<GameParameters>();

    #[cfg(feature = "dev_native")]
    app.add_systems(Update, save_file);

    #[cfg(feature = "dev_native")]
    info!("Press 'ctrl+s' to save game parameters");
}

#[cfg(feature = "dev_native")]
fn save_file(keyboard: Res<ButtonInput<KeyCode>>, game_parameters: Res<GameParameters>) {
    if keyboard.all_pressed([KeyCode::ControlLeft, KeyCode::KeyS]) {
        info!("Save game parameters");
        match toml::to_string(game_parameters.values()) {
            Ok(s) => {
                if let Some(err) = std::fs::write("./assets/game_parameters.toml", &s).err() {
                    error!("Cannot save ./assets/game_parameters.toml");
                }
            }
            Err(err) => error!("Cannot save ./assets/game_parameters.toml : {err:?}"),
        }
    }
}
