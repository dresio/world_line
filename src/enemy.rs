use avian3d::{math::PI, prelude::*};
use bevy::{
    math::ops::{cos, sin},
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
        app.add_systems(Startup, spawn_enemy)
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
}

// Gets enemy from
pub fn manage_enemy(
    mut query: Query<
        (&mut Enemy, &mut Transform, &mut TnuaController),
        Without<crate::player::Player>,
    >,
    player: Single<&Transform, With<crate::player::Player>>,
    fixed_time: Res<Time<Fixed>>,
) {
    for mut enemy in query {
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
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Transform::from_xyz(30.0, 20.0, 0.0),
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
        },
    ));
}
