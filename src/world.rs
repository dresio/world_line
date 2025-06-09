use avian3d::math::PI;
use bevy::prelude::*;
use rand::Rng;

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

pub fn sample_random_point() -> Vec3 {
    let x_range = (-800.0, 800.0);
    let y_range = (-800.0, 800.0);

    Vec3 {
        x: rand::rng().random_range(x_range.0..x_range.1),
        y: 0.0,
        z: rand::rng().random_range(y_range.0..y_range.1),
    }
}
