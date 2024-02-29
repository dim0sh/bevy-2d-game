use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform::{self, commands}};

use crate::movement::Velocity;

const START_VEL: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Component,Debug)]
pub struct Enemie;

#[derive(Bundle)]
struct EnemieBundle {
    velocity: Velocity,
    model: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct EnemiePlugin;

impl Plugin for EnemiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemie)
            .add_systems(Update, enemie_movement);	
    }
}

fn spawn_enemie(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        EnemieBundle {
        velocity: Velocity { velocity: START_VEL},
        model: MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20.0, 20.0)).into(),
            material: materials.add(Color::rgb(1.0, 0.5, 0.5)).into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
            },
        },
        Enemie,
    ));
}

fn enemie_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform),With<Enemie>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}