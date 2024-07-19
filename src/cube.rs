use bevy::prelude::*;
use crate::movement::Rotates;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ROTATION_SPEED: f32 = 1.0;

#[derive(Bundle)]
struct CubeBundle {
    rotates: Rotates,
    pbr: PbrBundle,
}

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cube);
    }
}

fn spawn_cube(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(CubeBundle {
        rotates: Rotates {
            speed: ROTATION_SPEED,
        },
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.0),
                ..default()
            }),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
