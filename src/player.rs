use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_tnua::prelude::*;


#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Player;

impl Plugin for Player
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player));
    }
}

// Create player object from bevy_skein, put camera on it, and attach player controller
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            }, //TODO: Fix collider similar to what is used to get world collider, but need to also fix rotations about all but yaw
        ))
        //Add camera as child for camera position
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ));
        });
}
