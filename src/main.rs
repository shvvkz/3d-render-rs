mod camera;
mod debug;
mod movement;
mod cube;
mod ui;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use cube::{CubePlugin, CubeProperties};
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1.0,
        })
        .insert_resource(CubeProperties {
            color: Color::rgb(1.0, 0.0, 0.0),
            border_radius: 0.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CubePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)
        .run();
}
