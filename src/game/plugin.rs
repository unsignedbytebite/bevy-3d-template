use super::{
    components, events,
    resources::{self, GameParameters},
    startups, states, updates,
};
use bevy::{ecs::schedule::BoxedCondition, prelude::*};
use bevy_common_assets::yaml::YamlAssetPlugin;
use std::sync::Mutex;

#[derive(Default)]
pub struct Game {
    condition: Mutex<Option<BoxedCondition>>,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_if<M>(mut self, condition: impl Condition<M>) -> Self {
        let condition_system = IntoSystem::into_system(condition);
        self.condition = Mutex::new(Some(Box::new(condition_system) as BoxedCondition));
        self
    }
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(YamlAssetPlugin::<GameParameters>::new(&[
            "game_parameters.yaml",
        ]));

        // Registers
        app.register_type::<components::FallingCube>();

        // States
        // none

        // Events
        // none

        // Resources
        // none

        // Setups
        app.add_systems(PreStartup, startups::load_parameters);
        app.add_systems(
            Startup,
            (
                startups::create_camera,
                startups::create_world,
                startups::create_player,
            ),
        );

        // Updates
        app.add_systems(Update, (updates::reset_cube, updates::rotate_camera));
    }
}
