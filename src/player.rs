use std::{sync::Arc, thread::current, time::Duration};

use avian3d::prelude::*;
use bevy::{ecs::system::command, prelude::*};
use bevy_egui::{EguiContextPass, EguiContexts, egui};
use bevy_tnua::prelude::*;
use std::f32::consts::PI;

use crate::world;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerTop {
    pub yaw: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerBottom {}

#[derive(Component, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, point_top)
            .add_systems(Update, point_bottom)
            .add_systems(EguiContextPass, player_ui);
    }
}

#[derive(Component, Debug)]
pub struct Player {
    // Movement params
    pub max_speed: f32,
    pub nominal_speed: f32,
    pub current_speed: f32,
    pub boost_timer: Timer,
    pub accel: f32,
    pub nominal_accel: f32,
    pub boost_accel: f32,

    // Firing params
    pub last_fired_gun: GunSide,
    pub fire_timer: Timer,

    pub health: f32,
    pub enemy_seed: f32,
    pub score: u128,
}

#[derive(Debug, PartialEq)]
pub enum GunSide {
    Left,
    Right,
}

// Create player object from bevy_skein, put camera on it, and attach player controller
fn spawn_player(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    world::startup(&mut commands, &mut asset_server);

    //Spawns player object
    commands
        .spawn((
            SceneRoot(asset_server.load(
                // Change this to your exported gltf file
                GltfAssetLabel::Scene(2).from_asset("TestScene.glb"),
            )),
            ColliderConstructor::Cylinder {
                radius: (15.0),
                height: (15.0),
            },
            Transform::from_xyz(0.0, 50.0, 0.0),
            TnuaController::default(),
            RigidBody::Dynamic,
            LockedAxes::new().lock_rotation_y(),
            Player {
                max_speed: 5000.0,
                nominal_speed: 300.0,
                current_speed: 300.0,
                accel: 500.0,
                nominal_accel: 500.0,
                boost_accel: 5000.0,
                boost_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                last_fired_gun: GunSide::Left,
                fire_timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once),
                health: 100.0,
                enemy_seed: 0.0,
                score: 0,
            },
            CollisionEventsEnabled,
        ))
        //Add camera as child for camera position
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(150.0, 300.0, 0.0)
                    .looking_at(Vec3::new(0.0, 15.0, 0.0), Vec3::Y),
            ));
        });
}

// System to direct the top of the mech to point at the mouse
fn point_top(mut top: Single<(&mut Transform, &mut PlayerTop)>, window: Single<&mut Window>) {
    let result = window.cursor_position();
    match result {
        Some(mouse_pos) => {
            let center = Vec2 {
                x: window.resolution.width() / 2.0,
                y: window.resolution.height() / 2.0,
            };

            //No clue why this function uses both the x and z...

            let angle = -(center - mouse_pos).to_angle();
            let player_angle: f32 = top.0.rotation.to_euler(EulerRot::YXY).0;

            top.1.yaw = angle;

            top.0
                .rotate_y((angle + f32::to_radians(15.0) + PI) - player_angle); //has a weird 15 deg offset?
        }

        None => { //Do nothing 
        }
    }
}

// System to direct the bottom of the mech to point at direction of travel
fn point_bottom(
    mut bot: Single<(&mut Transform, &PlayerBottom)>,
    query: Single<&LinearVelocity, With<Player>>,
) {
    let movement = Vec2 {
        x: query.0.z,
        y: query.0.x,
    };

    let speed = movement.length();

    if speed > 1e-2 {
        let angle = movement.to_angle();
        let current_angle = bot.0.rotation.to_euler(EulerRot::YXY).0; //using yxy because xyz is wrapping around pi/2 instead of pi
        let delta = angle - current_angle;

        bot.0.rotate_y(delta);
    }
}

fn debug_player_pos(query: Single<&Transform, With<Player>>) {
    dbg!(query.translation);
}

fn player_ui(mut contexts: EguiContexts, player: Single<&Player>) {
    let health = player.health;
    let boost_cooldown = 1.0 - player.boost_timer.elapsed_secs();
    let fire_cooldown = 5.0 - player.fire_timer.elapsed_secs();
    let score = player.score;
    egui::Window::new("UI").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Health: {health}"));
        ui.label(format!("Boost Cooldown: {:.2}", boost_cooldown));
        ui.label(format!("Reload Time Left: {:.2}", fire_cooldown));
        ui.label(format!("Score: {score}"));
    });
}
