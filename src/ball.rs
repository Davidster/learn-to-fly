use bevy::{input::keyboard::KeyboardInput, prelude::*, utils::Instant};
use bevy_rapier3d::prelude::*;

pub struct BallPlugin;

#[derive(Bundle)]
pub struct BallBundle {
    pbr_material: PbrBundle,
    collider: Collider,
    rigid_body: RigidBody,
    restitution: Restitution,
    force: ExternalForce,
    impulse: ExternalImpulse,
    friction: Friction,
    ball: Ball,
    // velocity: Velocity,
    // sleeping: Sleeping,
    // gravity_scale: GravityScale,
    // ccd: Ccd,
}

#[derive(Component)]
pub struct Ball {
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_jump_pressed: bool,
    last_jump_time: Option<Instant>,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (process_inputs, update_ball));
    }
}

impl BallBundle {
    pub fn new(
        radius: f32,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_material: PbrBundle {
                mesh: meshes.add(
                    Mesh::try_from(shape::Icosphere {
                        radius,
                        subdivisions: 5,
                    })
                    .unwrap(),
                ),
                material: materials.add(Color::rgb(0.1, 0.2, 0.9).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            collider: Collider::ball(radius),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.7),
            force: Default::default(),
            impulse: Default::default(),
            friction: Friction {
                coefficient: 10.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            ball: Ball {
                is_forward_pressed: false,
                is_backward_pressed: false,
                is_left_pressed: false,
                is_right_pressed: false,
                is_jump_pressed: false,
                last_jump_time: None,
            },
        }
    }
}

fn process_inputs(
    mut query: Query<&mut Ball>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    let mut ball = query.single_mut();

    for event in keyboard_input_events.iter() {
        match event.key_code {
            Some(KeyCode::Up) => {
                ball.is_forward_pressed = event.state.is_pressed();
            }
            _ => {}
        }
    }
}

fn update_ball(
    mut query: Query<(&mut Ball, &mut ExternalForce, &mut ExternalImpulse)>,
    time: Res<Time>,
) {
    let (mut ball, mut ext_force, mut ext_impuse) = query.single_mut();

    if ball.is_forward_pressed {
        ext_force.force = Vec3::new(1.0, 0.0, 0.0);
    } else {
        ext_force.force = Vec3::new(0.0, 0.0, 0.0);
    }

    // if ball.is_jump_pressed && ball.last_jump_time.is_none()
    //     || time
    //         .elapsed()
    //         .duration_since(ball.last_jump_time.unwrap())
    //         .as_secs_f32()
    // {}
}
