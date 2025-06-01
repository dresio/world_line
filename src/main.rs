use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};

use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_skein::SkeinPlugin;

fn main() {
    App::new()
        .register_type::<Character>()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            TnuaAvian3dPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
        ))
        .add_observer(
            // log the component from the gltf spawn
            |trigger: Trigger<SceneInstanceReady>,
             children: Query<&Children>,
             characters: Query<&Character>| {
                for entity in children.iter_descendants(trigger.target()) {
                    let Ok(character) = characters.get(entity) else {
                        continue;
                    };
                    info!(?character);
                }
            },
        )
        .add_systems(Startup, (startup, spawn_player))
        .add_systems(Update, player_input.in_set(TnuaUserControlsSystemSet))
        .run();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Character {
    name: String,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Basic light
    commands.spawn((DirectionalLight {
        shadows_enabled: true,
        ..default()
    },));

    commands.spawn(SceneRoot(asset_server.load(
        // Change this to your exported gltf file
        GltfAssetLabel::Scene(0).from_asset("TestScene.glb"),
    )));
}

// Create player object from bevy_skein, put camera on it, and attach player controller
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawns player object
    commands
        .spawn((
            SceneRoot(asset_server.load(
                // Change this to your exported gltf file
                GltfAssetLabel::Scene(1).from_asset("TestScene.glb"),
            )),
            Transform::from_xyz(0.0, 10.0, 0.0),
            TnuaController::default(),
            RigidBody::Dynamic,
            //TODO: Get collider
        ))
        //Add camera as child for camera position
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ));
        });
}

fn player_input(mut commands: Commands) {}
