use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform::{self, commands}};
use crate::movement::Velocity;
use crate::player::Player;
#[derive(Component,Debug)]
pub struct Projectile {
    pub range: f32,
    pub damage: f32,
}

#[derive(Bundle)]
struct ProjectileBundle {
    velocity: Velocity,
    model: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(spawn_projectile, projectile_movement));
    }
}

fn spawn_projectile(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &Velocity), With<Player>>
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = query.single().0;
    let player_velocity = query.single().1;
    commands.spawn((
        ProjectileBundle {
        velocity:  *player_velocity ,
        model: MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(20.0, 20.0)).into(),
            material: materials.add(Color::rgb(1.0, 1.0, 0.5)).into(),
            transform: *player_transform,
            ..Default::default()
            },
        },
        Projectile { range: 100.0, damage: 10.0 },
    ));
}

fn projectile_movement(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform),With<Projectile>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();
    }
}