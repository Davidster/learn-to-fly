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
    last_jump_time: Option<f32>,
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
                coefficient: 100.0,
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
            Some(KeyCode::Down) => {
                ball.is_backward_pressed = event.state.is_pressed();
            }
            Some(KeyCode::Right) => {
                ball.is_right_pressed = event.state.is_pressed();
            }
            Some(KeyCode::Left) => {
                ball.is_left_pressed = event.state.is_pressed();
            }
            Some(KeyCode::Space) => {
                ball.is_jump_pressed = event.state.is_pressed();
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

    let movement_vector = {
        let speed = 1.0;

        let forward_direction = Vec3::new(1.0, 0.0, 0.0);
        let up_direction = Vec3::new(0.0, 1.0, 0.0);
        let right_direction = forward_direction.cross(up_direction);

        let mut movement_vector: Option<Vec3> = None;
        let mut add_movement = |movement: Vec3| {
            movement_vector = match movement_vector {
                Some(res) => Some(res + movement),
                None => Some(movement),
            }
        };

        if ball.is_forward_pressed {
            add_movement(forward_direction);
        } else if ball.is_backward_pressed {
            add_movement(-forward_direction);
        }

        if ball.is_right_pressed {
            add_movement(right_direction);
        } else if ball.is_left_pressed {
            add_movement(-right_direction);
        }

        movement_vector
            .map(|movement_vector| movement_vector.normalize() * speed)
            .unwrap_or(Vec3::new(0.0, 0.0, 0.0))
    };

    ext_force.force = movement_vector;

    // if ball.is_forward_pressed {
    //     ext_force.force = Vec3::new(1.0, 0.0, 0.0);
    // } else {
    //     ext_force.force = Vec3::new(0.0, 0.0, 0.0);
    // }

    if ball.is_jump_pressed
        && (ball.last_jump_time.is_none()
            || time.elapsed().as_secs_f32() - ball.last_jump_time.unwrap() > 2.0)
    {
        ext_impuse.impulse = Vec3::new(0.0, 2.0, 0.0);
        ball.last_jump_time = Some(time.elapsed().as_secs_f32());
    } else {
        ext_impuse.impulse = Vec3::new(0.0, 0.0, 0.0);
    }
}
