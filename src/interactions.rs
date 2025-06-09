use std::time::Duration;

use bevy::{
    color::palettes::tailwind::RED_400,
    ecs::observer::TriggerTargets,
    pbr::ExtendedMaterial,
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
    text::cosmic_text::ttf_parser::Width,
    transform,
};

use avian3d::{
    parry::query,
    prelude::{ColliderConstructor, CollisionEventsEnabled, CollisionStarted, RigidBody, Sensor},
};
use bevy_seedling::{
    prelude::{Volume, VolumeNode},
    sample::SamplePlayer,
    sample_effects,
};

use crate::{
    enemy::Enemy,
    player::{self, Player},
    weapons::{Bullet, Explosion},
};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manage_collisions_bullets)
            .add_systems(FixedUpdate, (manage_explosion, manage_explosion_contacts));
    }
}

fn manage_collisions_bullets(
    mut collision_event_reader: EventReader<CollisionStarted>,
    query_enemies: Query<(Entity, &Transform), With<Enemy>>,
    query_bullets: Query<(Entity, &Bullet)>,
    mut player: Single<(Entity, &mut Player)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut server: Res<AssetServer>,
    mut command: Commands,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if query_enemies.contains(*entity1) && query_bullets.contains(*entity2) {
            let transform = query_enemies.get(*entity1).unwrap().1;
            let is_hit = query_bullets.get(*entity2).unwrap().1.shot_from
                == crate::factions::Factions::Player;

            if is_hit {
                //Despawn bullet and enemy
                command.entity(*entity1).despawn();
                command.entity(*entity2).despawn();

                create_explosion(
                    &mut command,
                    transform,
                    &mut meshes,
                    &mut materials,
                    &mut server,
                );
                player.1.score += 1;
            }
        }

        if player.0 == *entity1 && query_bullets.contains(*entity2) {
            let is_hit = query_bullets.get(*entity2).unwrap().1.shot_from
                == crate::factions::Factions::Dominion;

            if is_hit {
                //Despawn bullet and enemy
                command.entity(*entity2).despawn();
                player.1.health -= query_bullets.get(*entity2).unwrap().1.damage;
                command.spawn(SamplePlayer::new(server.load("DamageTaken.wav")));
            }
        }

        if player.0 == *entity1 && query_enemies.contains(*entity2) {
            player.1.health -= 5.0;
            command.spawn(SamplePlayer::new(server.load("BumpDamage.wav")));
        } else if player.0 == *entity2 && query_enemies.contains(*entity1) {
            player.1.health -= 5.0;
            command.spawn(SamplePlayer::new(server.load("BumpDamage.wav")));
        }
        {}
    }
}

fn manage_explosion_contacts(
    mut collision_event_reader: EventReader<CollisionStarted>,
    query_enemies: Query<(Entity, &Transform), With<Enemy>>,
    query_explosions: Query<Entity, With<Explosion>>,
    mut player: Single<&mut Player>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut server: Res<AssetServer>,
    mut command: Commands,
) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        if query_enemies.contains(*entity1) && query_explosions.contains(*entity2) {
            let transform = query_enemies.get(*entity1).unwrap().1;

            command.entity(*entity1).despawn();
            create_explosion(
                &mut command,
                transform,
                &mut meshes,
                &mut materials,
                &mut server,
            );
            player.score += 1;
        }
        if query_enemies.contains(*entity2) && query_explosions.contains(*entity1) {
            let transform = query_enemies.get(*entity2).unwrap().1;

            command.entity(*entity2).despawn();
            create_explosion(
                &mut command,
                transform,
                &mut meshes,
                &mut materials,
                &mut server,
            );
            player.score += 1;
        }
    }
}

fn manage_explosion(
    mut explosions: Query<(&mut Transform, &mut Explosion, Entity)>,
    fixed_time: Res<Time<Fixed>>,
    mut command: Commands,
) {
    for mut explosion in explosions {
        explosion
            .1
            .lifetime
            .tick(Duration::from_secs_f32(fixed_time.delta_secs())); //tick the timer

        //scale according to timer
        let mut proportion = explosion.1.lifetime.elapsed_secs() / explosion.1.duration;
        // dbg!(explosion.1.lifetime.duration().as_secs_f32());
        if proportion > 1.0 {
            proportion = 1.0;
        }

        let scalar = proportion * explosion.1.max_scale;
        let scale = Vec3::ONE * scalar;

        explosion.0.scale = scale;

        if explosion.1.lifetime.finished() {
            command.entity(explosion.2).despawn();
        }
    }
}

fn create_explosion(
    mut command: &mut Commands,
    position: &Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    server: &mut Res<AssetServer>,
) {
    command.spawn((
        Mesh3d(meshes.add(SphereMeshBuilder::new(
            5.0,
            SphereKind::Uv {
                sectors: 20,
                stacks: 20,
            },
        ))),
        Transform::from_translation(position.translation),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED_400.into(),
            ..default()
        })),
        ColliderConstructor::Sphere { radius: 5.0 },
        RigidBody::Dynamic,
        Sensor,
        CollisionEventsEnabled,
        crate::weapons::Explosion {
            max_scale: 10.0,
            lifetime: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            duration: 0.3,
        },
        SamplePlayer::new(server.load("Explosion.wav")),
        sample_effects![VolumeNode {
            volume: Volume::Decibels(-10.0)
        }],
    ));
}
