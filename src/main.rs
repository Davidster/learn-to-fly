mod ball;
mod camera_controller;
mod platform;

use ball::{BallBundle, BallPlugin};
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;
use camera_controller::CameraControllerPlugin;
use platform::{PlatformBundle, PlatformPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraControllerPlugin,
            PlatformPlugin,
            BallPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, init)
        .add_systems(Update, (on_update, grab_mouse))
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(BallBundle::new(
        0.5,
        Vec3::new(0.0, 3.0, 0.0),
        &mut meshes,
        &mut materials,
    ));

    commands.spawn(PlatformBundle::new(
        5.0,
        0.1,
        5.0,
        Vec3::ZERO,
        &mut meshes,
        &mut materials,
    ));

    commands.spawn(PlatformBundle::new(
        2.5,
        0.1,
        2.5,
        Vec3::new(9.0, 0.0, 0.0),
        &mut meshes,
        &mut materials,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        // transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn on_update(time: Res<Time>) {}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
