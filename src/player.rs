use bevy::{ prelude::*, sprite::MaterialMesh2dBundle, transform::{self, commands}};

use crate::movement::Velocity;

const START_VEL: Vec2 = Vec2::new(50.0, 50.0);
const START_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component,Debug)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    velocity: Velocity,
    model: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);	
    }
}

fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        PlayerBundle {
        velocity: Velocity { velocity: START_VEL},
        model: MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20.0, 20.0)).into(),
            material: materials.add(Color::rgb(0.5, 0.5, 1.0)).into(),
            transform: Transform::from_translation(START_POS),
            ..Default::default()
            },
        },
        Player,
    ));
}

fn player_movement(time: Res<Time>,keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(& Velocity,&mut Transform),With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += velocity.velocity.y * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= velocity.velocity.y * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= velocity.velocity.x * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += velocity.velocity.x * time.delta_seconds();
        }
    }
}