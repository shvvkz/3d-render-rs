use bevy::prelude::*;
use crate::movement::Rotates;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ROTATION_SPEED: f32 = 0.2;

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

#[derive(Resource)]
pub struct CubeProperties {
    pub color: Color,
    pub border_radius: f32,
}
fn spawn_cube(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, properties: Res<CubeProperties>) {
    commands.spawn(CubeBundle {
        rotates: Rotates {
            speed: ROTATION_SPEED,
        },
        pbr: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: properties.color,
                ..default()
            }),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}