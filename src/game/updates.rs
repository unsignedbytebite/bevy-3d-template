use super::components::*;
use super::data_enums::*;
use super::data_structs::*;
use super::events::*;
use super::resources::*;
use super::states::*;
use crate::game_parameters::GameParameters;
use avian3d::prelude::*;
use bevy::input::keyboard;
use bevy::math::VectorSpace;
use bevy::prelude::*;

pub fn reset_cube(
    params: Res<GameParameters>,
    mut q_cube: Query<(&mut Transform, &mut LinearVelocity), With<FallingCube>>,
) {
    for (mut trans, mut vel) in q_cube {
        if trans.translation.y < params.values().cube_reset_z {
            trans.translation = Vec3::Y * 4.0;
            *vel = LinearVelocity::ZERO;
        }
    }
}

pub fn rotate_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    params: Res<GameParameters>,
) {
    let delta = time.delta_secs();
    let cam_speed = params.values().camera_speed;

    if input.pressed(KeyCode::KeyA) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-cam_speed * delta));
    } else if input.pressed(KeyCode::KeyD) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(cam_speed * delta));
    }
}
