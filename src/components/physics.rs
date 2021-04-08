use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{
        storage::{DenseVecStorage, VecStorage},
        Component, Entity, WriteStorage,
    },
    Error,
};

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct RigidBody {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Gravitational(pub f32);
