use bevy::{gltf::GltfMeshExtras, prelude::*, scene::SceneInstanceReady};
use serde::{Deserialize, Serialize};
use serde_json;
use avian3d::prelude::*;

pub struct World;

impl Plugin for World
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
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
struct BMeshExtras {
    pub collider: BCollider,
}

#[derive(Serialize, Deserialize, Debug)]
enum BCollider {
    TrimeshFromMesh,
}
