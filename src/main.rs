use std::f64::consts::E;

use bevy::prelude::*;

mod player;
mod movement;
mod enemy;
mod projectile;

use player::PlayerPlugin;
use movement::MovementPlugin;
use enemy::EnemiePlugin;
use projectile::ProjectilePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProjectilePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}
