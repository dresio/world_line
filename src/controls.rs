use bevy::prelude::*;
use bevy_tnua::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Controller;

impl Plugin for Controller {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement);
    }
}

fn apply_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    //pressed
    //just_pressed
    //just_released

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.x -= 1.0;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * 80.0,
        float_height: 11.0,
        ..Default::default()
    });
}

fn fire_laser() {}
