use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::color::palettes::css::{BLUE, GREEN, RED, WHITE};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_skein::SkeinPlugin;
use std::f32::consts::PI;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InspectorState {
    #[default]
    Visible,
    Hidden,
}

pub fn register(mut app: &mut App) {
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

    app.add_plugins((
        SkeinPlugin::default(),
        fps_overlay,
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        ResourceInspectorPlugin::<super::game_parameters::GameParameters>::default()
            .run_if(in_state(InspectorState::Visible)),
        WorldInspectorPlugin::default().run_if(in_state(InspectorState::Visible)),
        PhysicsDebugPlugin::default(),
    ))
    .insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..Default::default()
        },
    );

    app.init_state::<InspectorState>();

    app.add_systems(
        Update,
        (toggle, draw_grid.run_if(in_state(InspectorState::Visible))),
    );

    if super::LOG_GLTF_NODES {
        app.add_observer(
            |trigger: Trigger<SceneInstanceReady>, children: Query<&Children>, a: Query<&Name>| {
                for entity in children.iter_descendants(trigger.target()) {
                    if let Ok(e) = a.get(entity) {
                        info!("loaded from gltf = {e:?}");
                    };
                }
            },
        );
    }

    info!("Press 'F1' to toggle dev mode");
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
fn toggle(
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

        let gizmos = gizmo_store.config_mut::<PhysicsGizmos>();
        gizmos.0.enabled != gizmos.0.enabled;

        info!("Dev mode set: {}", fps_overlay.enabled);
    }
}
