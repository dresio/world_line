use avian3d::prelude::*;
use bevy::{
    ecs::identifier::Identifier, gltf::GltfMeshExtras, prelude::*, render::camera::Viewport,
    scene::SceneInstanceReady, transform, ui::RelativeCursorPosition, winit::WinitSettings,
};
use bevy_tnua::{TnuaBasisContext, prelude::*};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Enemy {
    health: u32,
    faction: crate::factions::Factions,
}

// Gets enemy from
pub fn manage_enemy(mut query: Query<(&mut Enemy, &mut Transform)>) {}
