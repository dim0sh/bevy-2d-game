use bevy::prelude::*;

#[derive(Component,Debug,Copy,Clone)]
pub struct Velocity {
    pub velocity: Vec2,
}
pub struct MovementPlugin;

// impl Plugin for MovementPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, move_object);
//     }
// }

// fn move_object(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
//     for (velocity, mut transform) in query.iter_mut() {
//         transform.translation.x += velocity.velocity.x * time.delta_seconds();
//         transform.translation.y += velocity.velocity.y * time.delta_seconds();
//     }
// }