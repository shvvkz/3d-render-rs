use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Rotates {
    pub speed: f32,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, rotate_objects));
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn rotate_objects(mut query: Query<(&Rotates, &mut Transform)>, time: Res<Time>) {
    // for (rotates, mut transform) in query.iter_mut() {
    //     transform.rotate(Quat::from_rotation_y(rotates.speed * time.delta_seconds()));
    //     transform.rotate(Quat::from_rotation_x(rotates.speed * time.delta_seconds()));
    // }
}
