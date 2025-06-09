pub mod blender_helpers;
pub mod controls;
pub mod enemy;
pub mod factions;
pub mod interactions;
pub mod player;
pub mod weapons;
pub mod world;

use avian3d::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin, egui};
use bevy_seedling::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

// Bevy blender integration
use bevy::prelude::*;
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
            SeedlingPlugin::default(),
            (EguiPlugin {
                enable_multipass_for_primary_context: true,
            }),
            // Added code
            // world::World,
            player::PlayerPlugin,
            controls::Controller,
            weapons::WeaponsPlugin,
            interactions::InteractionsPlugin,
            enemy::BaseEnemyPlugin,
            // PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, play_music)
        .run();
}

fn play_music(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(SamplePlayer::new(server.load("Test.wav")).looping());
}
