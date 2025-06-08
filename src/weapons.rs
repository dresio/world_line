use avian3d::{
    parry::{math::Rotation, transformation::utils::transform},
    prelude::{GravityScale, LinearVelocity, RigidBody},
};
use bevy::{
    math::ops::{cos, sin},
    prelude::*,
};
use std::f32::consts::PI;

pub struct BulletSpawnData {
    pub position: Vec3,
    pub yaw: f32,
    pub speed: f32,
    pub damage: f32,
    pub shot_from: crate::factions::Factions,
}

#[derive(Component, Debug)]
pub struct Bullet {
    damage: f32,
    shot_from: crate::factions::Factions,
}

pub fn shoot_bullet(mut commands: Commands, data: BulletSpawnData, asset_server: Res<AssetServer>) {
    dbg!(data.yaw);
    let vel = Vec3::new(data.speed * sin(data.yaw), 0.0, data.speed * cos(data.yaw));

    dbg!(vel);

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
    ));
}
