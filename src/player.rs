use avian3d::prelude::*;
use bevy::{
    ecs::identifier::Identifier, gltf::GltfMeshExtras, prelude::*, render::camera::Viewport,
    scene::SceneInstanceReady, transform, ui::RelativeCursorPosition, winit::WinitSettings,
};
use bevy_tnua::{TnuaBasisContext, prelude::*};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerTop {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerBottom {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, point_top)
            .add_systems(Update, point_bottom);
    }
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
            Transform::from_xyz(0.0, 20.0, 0.0),
            TnuaController::default(),
            RigidBody::Dynamic,
            LockedAxes::new().lock_rotation_y(),
            ColliderConstructor::Cylinder {
                radius: (10.0),
                height: (6.0),
            },
        ))
        //Add camera as child for camera position
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(100.0, 100.0, 0.0)
                    .looking_at(Vec3::new(0.0, 15.0, 0.0), Vec3::Y),
            ));
        });
}

fn point_top(mut top: Single<(&mut Transform, &PlayerTop)>, window: Single<&mut Window>) {
    let result = window.cursor_position();
    match result {
        Some(mouse_pos) => {
            let center = Vec2 {
                x: window.resolution.width() / 2.0,
                y: window.resolution.height() / 2.0,
            };

            //No clue why this function uses both the x and z...

            let angle = -(center - mouse_pos).to_angle();
            let player_angle = top.0.rotation.to_euler(EulerRot::XYZ).2;

            top.0.rotate_y(angle - player_angle);
        }

        None => { //Do nothing 
        }
    }
}

fn point_bottom(
    mut bot: Single<(&mut Transform, &PlayerBottom)>,
    mut query: Query<&mut LinearVelocity>,
) {
    for (mut linear_velocity) in &mut query {
        // dbg!(linear_velocity);
    }
}
