mod ball;
mod camera_controller;
mod platform;

use ball::{BallBundle, BallPlugin};
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;
use camera_controller::CameraControllerPlugin;
use platform::{PlatformBundle, PlatformPlugin};

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

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
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, init)
        .add_systems(Update, (on_update, grab_mouse))
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Name("Elaina Proctor".to_string()));
    commands.spawn(Name("Renzo Hume".to_string()));
    commands.spawn(Name("Zayna Nieves".to_string()));

    /* Create the bouncing ball. */
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    commands.spawn(BallBundle::new(
        0.5,
        Vec3::new(0.0, 3.0, 0.0),
        &mut meshes,
        &mut materials,
    ));

    /* Create the ground. */

    // let ground_box_dims = (5.0, 0.1, 5.0);
    // let ground_position = Vec3::ZERO;
    commands.spawn(PlatformBundle::new(
        5.0,
        0.1,
        5.0,
        Vec3::ZERO,
        &mut meshes,
        &mut materials,
    ));
    // commands
    //     .spawn(Collider::cuboid(
    //         ground_box_dims.0 / 2.0,
    //         ground_box_dims.1 / 2.0,
    //         ground_box_dims.2 / 2.0,
    //     ))
    //     .insert(TransformBundle::from(Transform::from_translation(
    //         ground_position,
    //     )));
    // // plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(
    //         ground_box_dims.0,
    //         ground_box_dims.1,
    //         ground_box_dims.2,
    //     ))),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     transform: Transform::from_translation(ground_position),
    //     ..default()
    // });

    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // light
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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn on_update(time: Res<Time>, mut timer: ResMut<GreetTimer>, names_query: Query<&Name>) {
    if timer.0.tick(time.delta()).just_finished() {
        let current_time = time.elapsed();
        println!("Current time is: {current_time:?}");

        for name in &names_query {
            println!("hello {}!", name.0);
        }
    }
}

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
