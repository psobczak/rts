use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

pub struct GroundPlugin;

const PLANE_SIZE: f32 = 100.0;
const GRID_SIZE: f32 = 1.0;

#[derive(Resource)]
struct Grid {
    columns: usize,
    rows: usize,
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid {
            columns: (PLANE_SIZE / GRID_SIZE) as usize,
            rows: (PLANE_SIZE / GRID_SIZE) as usize,
        })
        .add_startup_system(spawn_ground);
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
    ));
}
