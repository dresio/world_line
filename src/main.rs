use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};

// Bevy blender integration
use bevy::{gltf::GltfMeshExtras, prelude::*, scene::SceneInstanceReady};
use bevy_skein::SkeinPlugin;
use serde::{Deserialize, Serialize};
use serde_json;

use bevy_simple_subsecond_system::prelude::*;

fn main() {
    App::new()
        .register_type::<Character>()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            // TnuaAvian3dPlugin::new(FixedUpdate),
            // TnuaControllerPlugin::new(FixedUpdate),
            SimpleSubsecondPlugin::default(),
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

    commands
        .spawn(SceneRoot(asset_server.load(
            // Change this to your exported gltf file
            GltfAssetLabel::Scene(0).from_asset("TestScene.glb"),
        )))
        .observe(load_blender_data);
}

// Create player object from bevy_skein, put camera on it, and attach player controller
#[hot]
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
            ColliderConstructor::Cuboid {
                x_length: 1.0,
                y_length: 1.0,
                z_length: 1.0,
            }, //TODO: Get collider
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

fn load_blender_data(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    extras: Query<&GltfMeshExtras>,
) {
    for entity in children.iter_descendants(trigger.target()) {
        let Ok(gltf_mesh_extras) = extras.get(entity) else {
            continue;
        };

        let Ok(data) = serde_json::from_str::<BMeshExtras>(&gltf_mesh_extras.value) else {
            error!("Issue with collider format on extra blender data");
            continue;
        };
        match data.collider {
            BCollider::TrimeshFromMesh => {
                commands
                    .entity(entity)
                    .insert((RigidBody::Static, ColliderConstructor::TrimeshFromMesh));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BMeshExtras {
    pub collider: BCollider,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BCollider {
    TrimeshFromMesh,
}
