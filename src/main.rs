#![allow(warnings)]

mod game;

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    math::VectorSpace,
    prelude::*,
    scene::SceneInstanceReady,
    text::FontSmoothing,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_skein::SkeinPlugin;
use std::f32::consts::PI;

const CLEAR_COLOUR: Color = Color::hsv(219.0, 0.58, 0.93);
const WINDOW_RESOLUTION: (u16, u16) = (1280, 720);
const TITLE: &str = "{{project-name}}";
const APP_NAME: &str = "{{project-name}}";
const RESIZABLE: bool = true;
const MOUSE_VISIBLE: bool = true;
const PHYSICS_DEBUG_RENDER: bool = true;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InspectorState {
    #[default]
    Visible,
    Hidden,
}

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
    let fps_overlay = FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 18.0,
                ..Default::default()
            },
            text_color: Color::WHITE,
            enabled: true,
            ..Default::default()
        },
    };

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(windows_settings()).set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }),
        PhysicsPlugins::default(),
        SkeinPlugin::default(),
        fps_overlay,
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::default().run_if(in_state(InspectorState::Visible)),
        game::plugin::Game::new(),
    ));

    if PHYSICS_DEBUG_RENDER {
        app.add_plugins(PhysicsDebugPlugin::default());
    }

    app.init_state::<InspectorState>();

    app.add_systems(
        Update,
        (
            exit,
            toggle_inspector,
            draw_grid.run_if(in_state(InspectorState::Visible)),
        ),
    );

    // Log the components from gltf spawn
    app.add_observer(
        |trigger: Trigger<SceneInstanceReady>, children: Query<&Children>, a: Query<&Name>| {
            for entity in children.iter_descendants(trigger.target()) {
                if let Ok(e) = a.get(entity) {
                    info!("loaded from gltf = {e:?}");
                };
            }
        },
    );

    app.insert_resource(ClearColor(CLEAR_COLOUR));

    info!("Press 'F1' to toggle dev mode");
    app.run();
}

/// A quick exit
fn exit(keyboard: Res<ButtonInput<KeyCode>>, mut ev_exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        ev_exit.write(AppExit::Success);
    }
}

/// Draw the world grid
fn draw_grid(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X * 10.0, RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 10.0, GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 10.0, BLUE);

    gizmos.grid(
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(20),
        Vec2::splat(1.0),
        LinearRgba::gray(0.65),
    );
}

/// Toggle the dev mode
fn toggle_inspector(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<InspectorState>>,
    mut next_state: ResMut<NextState<InspectorState>>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
    mut gizmo_store: ResMut<GizmoConfigStore>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        fps_overlay.enabled = !fps_overlay.enabled;
        match state.get() {
            InspectorState::Hidden => next_state.set(InspectorState::Visible),
            InspectorState::Visible => next_state.set(InspectorState::Hidden),
        }
        info!("Dev mode set: {}", fps_overlay.enabled);
    }
}
