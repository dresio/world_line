use avian3d::prelude::{
    ColliderConstructor, CollisionEventsEnabled, GravityScale, LinearVelocity, RigidBody, Sensor,
};
use bevy::{
    math::ops::{cos, sin},
    prelude::*,
};
use std::time::Duration;

#[derive(Component, Debug)]
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, manage_bullets);
    }
}

pub struct BulletSpawnData {
    pub position: Vec3,
    pub yaw: f32,
    pub speed: f32,
    pub damage: f32,
    pub shot_from: crate::factions::Factions,
}

#[derive(Component, Debug)]
pub struct Bullet {
    pub damage: f32,
    pub shot_from: crate::factions::Factions,
    //Manages despawning
    pub lifetime: Timer,
}

#[derive(Component, Debug)]
pub struct Explosion {
    pub max_scale: f32,
    pub lifetime: Timer,
    pub duration: f32,
}

pub fn shoot_bullet(mut commands: Commands, data: BulletSpawnData, asset_server: Res<AssetServer>) {
    let vel = Vec3::new(data.speed * sin(data.yaw), 0.0, data.speed * cos(data.yaw));
    commands.spawn((
        SceneRoot(asset_server.load(
            // Change this to your exported gltf file
            GltfAssetLabel::Scene(0).from_asset("Weapons.glb"),
        )),
        Transform {
            rotation: Quat::from_euler(EulerRot::XYZ, 0.0, data.yaw, 0.0),
            translation: data.position,
            scale: Vec3::ONE,
        },
        RigidBody::Dynamic,
        GravityScale(0.0), //no gravity on bullets
        LinearVelocity(vel),
        Bullet {
            damage: data.damage,
            shot_from: data.shot_from,
            lifetime: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once),
        },
        ColliderConstructor::Sphere { radius: (1.0) },
        Sensor,
        CollisionEventsEnabled,
    ));
}

// Destroys bullets after timer elapsed to prevent memory issues during long runtimes
pub fn manage_bullets(
    bullet_query: Query<(&mut Bullet, Entity)>,
    fixed_time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for mut entity in bullet_query {
        //update timer
        entity
            .0
            .lifetime
            .tick(Duration::from_secs_f32(fixed_time.delta_secs()));

        // despawn if timer is finished
        if entity.0.lifetime.finished() {
            commands.entity(entity.1).despawn();
        }
    }
}
