use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlatformPlugin;

#[derive(Bundle)]
pub struct PlatformBundle {
    pbr_material: PbrBundle,
    collider: Collider,
    platform: Platform,
}

#[derive(Component)]
pub struct Platform;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_platforms);
    }
}

impl PlatformBundle {
    pub fn new(
        hx: f32,
        hy: f32,
        hz: f32,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_material: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(hx * 2.0, hy * 2.0, hz * 2.0))),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            collider: Collider::cuboid(hx, hy, hz),
            platform: Platform,
        }
    }
}

fn update_platforms(mut platforms_query: Query<&mut Transform, With<Platform>>, time: Res<Time>) {
    platforms_query.for_each_mut(|mut platform| {
        platform.rotate(Quat::from_axis_angle(
            Vec3::new(0.0, 1.0, 0.0),
            time.delta().as_secs_f32(),
        ));
        // dbg!(platform.rotation);
    });
}
