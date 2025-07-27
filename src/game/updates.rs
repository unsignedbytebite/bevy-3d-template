use super::components::*;
use super::data_enums::*;
use super::data_structs::*;
use super::events::*;
use super::resources::*;
use super::states::*;
use avian3d::prelude::*;
use bevy::input::keyboard;
use bevy::math::VectorSpace;
use bevy::prelude::*;

pub fn reset_cube(mut q_cube: Query<(&mut Transform, &mut LinearVelocity), With<FallingCube>>) {
    for (mut trans, mut vel) in q_cube {
        if trans.translation.y < -5.0 {
            trans.translation = Vec3::Y * 4.0;
            *vel = LinearVelocity::ZERO;
        }
    }
}

pub fn rotate_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    params: Res<Assets<GameParameters>>,
    params_handle: Res<GameParametersHandle>,
) {
    let delta = time.delta_secs();
    let s = params.get(params_handle.0.id()).unwrap().camera_speed;

    if input.pressed(KeyCode::KeyA) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(s * delta));
    } else if input.pressed(KeyCode::KeyD) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-s * delta));
    }
}
