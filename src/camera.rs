use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

#[derive(Default, Resource)]
pub struct MouseState {
    pub pressed: bool,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseState::default())
           .add_systems(Startup, spawn_camera)
           .add_systems(Update, (update_camera, print_camera_position, mouse_button_input));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        OrbitCamera {
            yaw: 0.0,
            pitch: 0.0,
            distance: 5.0,
        },
        CameraController,
    ));
}

#[derive(Component)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

#[derive(Component)]
pub struct CameraController;

fn update_camera(
    time: Res<Time>,
    mouse_state: Res<MouseState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<CameraController>>,
) {
    let delta_time = time.delta_seconds();

    // Handle mouse motion
    if mouse_state.pressed {
        for event in mouse_motion_events.read() {
            let (mut transform, mut orbit_camera) = query.single_mut();
            let sensitivity = 0.4;

            orbit_camera.yaw += event.delta.x * sensitivity; // Inverted direction
            orbit_camera.pitch -= event.delta.y * sensitivity;

            orbit_camera.pitch = orbit_camera.pitch.clamp(-89.0, 89.0);

            let yaw_radians = orbit_camera.yaw.to_radians();
            let pitch_radians = orbit_camera.pitch.to_radians();

            let x = orbit_camera.distance * yaw_radians.cos() * pitch_radians.cos();
            let y = -(orbit_camera.distance * pitch_radians.sin());
            let z = orbit_camera.distance * yaw_radians.sin() * pitch_radians.cos();

            transform.translation = Vec3::new(x, y, z);
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }

    // Handle mouse wheel for zoom
    for event in mouse_wheel_events.read() {
        let (mut transform, mut orbit_camera) = query.single_mut();
        orbit_camera.distance -= event.y * delta_time * 10.0;
        orbit_camera.distance = orbit_camera.distance.clamp(2.0, 15.0);

        let yaw_radians = orbit_camera.yaw.to_radians();
        let pitch_radians = orbit_camera.pitch.to_radians();

        let x = orbit_camera.distance * yaw_radians.cos() * pitch_radians.cos();
        let y = -(orbit_camera.distance * pitch_radians.sin());
        let z = orbit_camera.distance * yaw_radians.sin() * pitch_radians.cos();

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn mouse_button_input(
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_state.pressed = true;
    } else if mouse_button_input.just_released(MouseButton::Left) {
        mouse_state.pressed = false;
    }
}

fn print_camera_position(query: Query<(&Transform, With<CameraController>)>) {
    for (transform, _) in query.iter() {
        info!("Camera is at position: {:?}", transform.translation);
    }
}
