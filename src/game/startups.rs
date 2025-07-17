use super::components::*;
use super::data_enums::*;
use super::data_structs::*;
use super::events::*;
use super::resources::*;
use super::states::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

/// Create a default camera off center
pub fn create_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Main camera"),
        Camera3d::default(),
        Transform::from_xyz(2.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn create_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        Name::new("Floor"),
        RigidBody::Static,
        Collider::cuboid(4.0, 0.1, 4.0),
        Restitution::new(1.0),
        Mesh3d(meshes.add(Cuboid::new(4.0, 0.1, 4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // Light
    commands.spawn((
        Name::new("Point Light"),
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

pub fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Bouncy cube"),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Restitution::new(1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
        FallingCube::default(),
    ));
}

pub fn load_parameters(mut commands: Commands, asset_server: Res<AssetServer>) {
    let params = GameParametersHandle(asset_server.load("game_parameters.yaml"));
    commands.insert_resource(params);
}
