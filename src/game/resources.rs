use super::data_enums::*;
use super::data_structs::*;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::Deserialize;

// #[derive(Reflect, Resource, InspectorOptions, Deserialize, Asset)]
// #[reflect(Resource, InspectorOptions)]
#[derive(Deserialize, Asset, TypePath)]
pub struct GameParameters {
    pub camera_speed: f32,
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GameParametersHandle(pub Handle<GameParameters>);
