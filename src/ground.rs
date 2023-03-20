use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct GroundPlugin;

const PLANE_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Ground;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(PLANE_SIZE).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Collider::cuboid(PLANE_SIZE / 2.0, 0.0, PLANE_SIZE / 2.0),
        Name::from("Ground"),
        Ground,
    ));
}
