use avian3d::prelude::*;
use bevy::{gltf::GltfMeshExtras, prelude::*, scene::SceneInstanceReady};
use serde::{Deserialize, Serialize};
use serde_json;

pub fn load_blender_data(
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
            // error!("Issue with collider format on extra blender data");
            // dbg!(gltf_mesh_extras);
            continue;
        };

        let mut rb_property = RigidBody::Static;

        match data.rigidbody {
            Some(property) => match property {
                BRigidBody::Dynamic => rb_property = RigidBody::Dynamic,
                BRigidBody::Static => rb_property = RigidBody::Static,
            },
            None => {} //Do Nothing,
        };

        match data.collider {
            BCollider::TrimeshFromMesh => {
                commands
                    .entity(entity)
                    .insert((rb_property, ColliderConstructor::TrimeshFromMesh));
            }
        }

        match data.sensor {
            Some(is_sensor) => {
                if is_sensor {
                    commands.entity(entity).insert(Sensor);
                }
            }
            None => {} // Do nothing,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BMeshExtras {
    pub collider: BCollider,
    pub rigidbody: Option<BRigidBody>,
    pub sensor: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
enum BCollider {
    TrimeshFromMesh,
}

#[derive(Serialize, Deserialize, Debug)]
enum BRigidBody {
    Dynamic,
    Static,
}
