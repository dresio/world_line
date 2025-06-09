use std::time::Duration;

use bevy::prelude::*;
use bevy_seedling::sample::SamplePlayer;
use bevy_tnua::prelude::*;

use crate::player::PlayerTop;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Controller;

impl Plugin for Controller {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement)
            .add_systems(Update, manage_speed)
            .add_systems(Update, fire_control)
            .add_systems(FixedUpdate, manage_boost_time)
            .add_systems(FixedUpdate, manage_fire_timer);
    }
}

fn apply_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Single<(&mut TnuaController, &crate::player::Player)>,
) {
    //pressed
    //just_pressed
    //just_released

    let mut direction = Vec3::ZERO;
    let speed = query.1.current_speed;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.x -= 1.0;
    }

    query.0.basis(TnuaBuiltinWalk {
        desired_velocity: direction * speed,
        float_height: 11.0,
        acceleration: 500.0,
        ..Default::default()
    });
}

// System to manage boost timings
fn manage_boost_time(mut player: Single<&mut crate::player::Player>, fixed_time: Res<Time<Fixed>>) {
    // Tick boost timer
    player
        .boost_timer
        .tick(Duration::from_secs_f32(fixed_time.delta_secs()));

    // Mange decreasing speed after boost is used
    if player.current_speed > player.nominal_speed {
        //should revert to old speed within 0.5 seconds-ish
        let delta = player.max_speed - player.nominal_speed;
        let delta_update = delta * 2.0 * fixed_time.delta_secs();
        let delta_accel = player.boost_accel - player.nominal_accel;
        let delta_accel_update = delta_accel * 2.0 * fixed_time.delta_secs();
        player.current_speed -= delta_update;
        player.accel -= delta_accel_update;
    }
}

// Manages speed boost
fn manage_speed(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut crate::player::Player>,
) {
    // If button is pressed and timer is ready, adjust max speed and reset timer
    if player.boost_timer.finished() && keyboard_input.just_pressed(KeyCode::Space) {
        player.current_speed = player.max_speed;
        player.accel = player.boost_accel;
        player.boost_timer.reset();
    }
}

fn manage_fire_timer(mut player: Single<&mut crate::player::Player>, fixed_time: Res<Time<Fixed>>) {
    player
        .fire_timer
        .tick(Duration::from_secs_f32(fixed_time.delta_secs()));
}

fn fire_control(
    mut player: Single<(&mut crate::player::Player, &Transform)>,
    top: Single<(&PlayerTop, &Transform)>,
    keyboard_input: Res<ButtonInput<MouseButton>>,
    mut asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard_input.pressed(MouseButton::Left) && player.0.fire_timer.finished() {
        let offset = Vec3::new(0.0, 5.0, 0.0);

        if player.0.last_fired_gun == crate::player::GunSide::Left {
            //will shoot from right gun
        } else {
            //will shoot from left gun
        }

        let bullet_data = crate::weapons::BulletSpawnData {
            position: player.1.translation + offset,
            yaw: top.0.yaw,
            speed: 1000.0,
            damage: 10.0,
            shot_from: crate::factions::Factions::Player,
        };

        crate::weapons::shoot_bullet(&mut commands, bullet_data, &mut asset_server);

        player.0.fire_timer.reset();
        commands.spawn(SamplePlayer::new(asset_server.load("MechBullet.wav")));
    }
}
