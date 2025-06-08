use bevy::{
    ecs::observer::TriggerTargets, prelude::*, text::cosmic_text::ttf_parser::Width, transform,
};

use avian3d::{parry::query, prelude::CollisionStarted};

use crate::{enemy::Enemy, weapons::Bullet};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_started_collisions);
    }
}

fn print_started_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    query_enemies: Query<(Entity, &Transform), With<Enemy>>,
    query_bullets: Query<Entity, With<Bullet>>,
    mut command: Commands,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if query_enemies.contains(*entity1) && query_bullets.contains(*entity2) {
            let transform = query_enemies.get(*entity1).unwrap().1;

            //Despawn bullet and enemy
            command.entity(*entity1).despawn();
            command.entity(*entity2).despawn();

            create_explosion(&mut command, transform);
        }
    }
}

fn create_explosion(mut command: &mut Commands, position: &Transform) {
    println!("BOOM");
}
