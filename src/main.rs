pub mod blender_helpers;
pub mod controls;
pub mod player;
pub mod world;

use avian3d::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

// Bevy blender integration
use bevy::{prelude::*, scene::SceneInstanceReady, winit::WinitSettings};
use bevy_skein::SkeinPlugin;

fn main() {
    App::new()
        .register_type::<player::PlayerTop>()
        .register_type::<player::PlayerBottom>()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
            PhysicsPlugins::default(),
            TnuaAvian3dPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            // Added code
            world::World,
            player::Player,
            controls::Controller,
        ))
        .add_observer(
            // log the component from the gltf spawn
            |trigger: Trigger<SceneInstanceReady>,
             children: Query<&Children>,
             characters: Query<&player::Player>| {
                for entity in children.iter_descendants(trigger.target()) {
                    let Ok(character) = characters.get(entity) else {
                        continue;
                    };
                    info!(?character);
                }
            },
        )
        .run();
}
