use avian3d::prelude::*;
use bevy::{gltf::GltfMeshExtras, prelude::*, scene::SceneInstanceReady};
use serde::{Deserialize, Serialize};
use serde_json;

pub struct World;

impl Plugin for World {
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
        .observe(crate::blender_helpers::load_blender_data);
}
