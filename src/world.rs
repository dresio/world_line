use std::f32::consts::FRAC_PI_4;

use avian3d::{math::PI, prelude::*};
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
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 400.0,
            color: Color::WHITE,
            affects_lightmapped_mesh_diffuse: true,
            shadow_depth_bias: 0.0199999996f32,
            shadow_normal_bias: 1.79999995f32,
        },
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_euler(EulerRot::ZYX, 0.0, -PI / 4.0, -PI / 4.0),
            scale: Vec3::ONE,
        },
    ));

    commands
        .spawn(SceneRoot(asset_server.load(
            // Change this to your exported gltf file
            GltfAssetLabel::Scene(1).from_asset("TestScene.glb"),
        )))
        .observe(crate::blender_helpers::load_blender_data);
}
