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
            error!("Issue with collider format on extra blender data");
            continue;
        };
        match data.collider {
            BCollider::TrimeshFromMesh => {
                commands
                    .entity(entity)
                    .insert((RigidBody::Static, ColliderConstructor::TrimeshFromMesh));
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BMeshExtras {
    pub collider: BCollider,
}

#[derive(Serialize, Deserialize, Debug)]
enum BCollider {
    TrimeshFromMesh,
}
