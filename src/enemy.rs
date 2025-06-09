use std::time::Duration;

use avian3d::{math::PI, prelude::*};
use bevy::{
    math::ops::{cos, powf, sin, sqrt},
    prelude::*,
};
use bevy_tnua::{
    TnuaGravity,
    prelude::{TnuaBuiltinWalk, TnuaController},
};

#[derive(Component, Debug)]
pub struct BaseEnemyPlugin;

impl Plugin for BaseEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, runtime_enemy_gen)
            .add_systems(FixedUpdate, manage_enemy);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Enemy {
    health: u32,
    faction: crate::factions::Factions,
    speed: f32,
    turn_rate: f32,

    fire_timer: Timer,
}

// Gets enemy from
pub fn manage_enemy(
    mut query: Query<
        (&mut Enemy, &mut Transform, &mut TnuaController),
        Without<crate::player::Player>,
    >,
    player: Single<&Transform, With<crate::player::Player>>,
    fixed_time: Res<Time<Fixed>>,
    mut asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut enemy in query {
        enemy
            .0
            .fire_timer
            .tick(Duration::from_secs_f32(fixed_time.delta_secs()));

        // get range to player
        let range = calc_distance(
            vec2(enemy.1.translation.x, enemy.1.translation.z),
            vec2(player.translation.x, player.translation.y),
        );

        enemy.0.speed = (range - 70.0) / 10.0;
        if enemy.0.speed < 0.0 {
            enemy.0.speed = 0.0;
        }

        // Manage rotation
        let player_transform = player.translation;
        let this_transform = enemy.1.translation;
        let point_to =
            Transform::from_translation(this_transform).looking_at(player_transform, Vec3::Y);

        let desired_yaw = -point_to.rotation.to_euler(EulerRot::YXY).0;

        // Have to do some weird stuff to ge this axis properly...
        let mut current_yaw = enemy.1.rotation.to_euler(EulerRot::ZYZ).1 + f32::to_radians(180.0);
        let test_yaw = enemy.1.rotation.to_euler(EulerRot::XYZ).1;
        if test_yaw > 0.0 {
            current_yaw *= -1.0;
        }

        let mut delta = current_yaw - desired_yaw;
        //manage pi wrapping
        while delta > 2.0 * PI {
            delta -= 2.0 * PI;
        }
        while delta < -2.0 * PI {
            delta += 2.0 * PI;
        }

        enemy.1.rotate_y(delta);

        //move forward at speed
        let vel = Vec3::new(sin(-desired_yaw), 0.0, cos(-desired_yaw));
        enemy.2.basis(TnuaBuiltinWalk {
            desired_velocity: -vel * enemy.0.speed,
            float_height: 11.0,
            acceleration: 10.0,
            ..Default::default()
        });

        if enemy.0.fire_timer.finished() {
            //spawn bullet
            let bullet_data = crate::weapons::BulletSpawnData {
                position: enemy.1.translation,
                yaw: current_yaw,
                speed: 300.0,
                damage: 1.0,
                shot_from: crate::factions::Factions::Dominion,
            };

            crate::weapons::shoot_bullet(&mut commands, bullet_data, &mut asset_server);

            enemy.0.fire_timer.reset();
        }
    }
}

pub fn runtime_enemy_gen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<&Transform, With<crate::player::Player>>,
    enemies: Query<&Enemy>,
    time: Res<Time<Real>>,
) {
    let count = enemies.iter().count();
    let max_count = (time.elapsed().as_secs_f32()) as usize; //allow 1 new tank every second

    if count < max_count {
        let mut point = crate::world::sample_random_point() + vec3(0.0, 10.0, 0.0);

        while calc_distance(
            vec2(point.x, point.z),
            vec2(player.translation.x, player.translation.y),
        ) < 100.0
        {
            point = crate::world::sample_random_point(); //just check if it is within visual range
        }

        spawn_enemy(commands, asset_server, point);
    }
}

fn calc_distance(start: Vec2, end: Vec2) -> f32 {
    let delta = end - start;
    sqrt(powf(delta.x, 2.0) + powf(delta.y, 2.0))
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, location: Vec3) {
    //Spawns single enemy object
    commands.spawn((
        SceneRoot(asset_server.load(
            // Change this to your exported gltf file
            GltfAssetLabel::Scene(0).from_asset("TestScene.glb"),
        )),
        ColliderConstructor::Cylinder {
            radius: (10.0),
            height: (20.0),
        },
        Transform::from_translation(location),
        ColliderDensity(10.0),
        TnuaController::default(),
        TnuaGravity(Vec3::new(0.0, -50.0, 0.0)),
        RigidBody::Dynamic,
        LockedAxes::new().lock_rotation_y().lock_rotation_x(),
        Enemy {
            health: 10,
            faction: crate::factions::Factions::Dominion,
            speed: 5.0,
            turn_rate: f32::to_radians(50.0),
            fire_timer: Timer::new(Duration::from_secs_f32(30.0), TimerMode::Once),
        },
    ));
}
