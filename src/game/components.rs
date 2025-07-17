use super::data_enums::*;
use super::data_structs::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
pub struct FallingCube {}
