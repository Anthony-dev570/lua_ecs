use crate::component::{Component, ComponentInitializer};
use crate::game_object::GameObject;
use lua_mathematics::quaternion::QuatF;
use lua_mathematics::vector::vec3::Vec3F;
use std::sync::{Arc, RwLock};
use mlua::AnyUserData;

pub struct TransformInner {
    parent: GameObject,

    position: Vec3F,
    rotation: QuatF,
    scale: Vec3F
}

pub struct Transform(Arc<RwLock<TransformInner>>);

impl Component for Transform {
    fn parent(&self) -> GameObject {
        self.0.read().unwrap().parent.clone()
    }

    fn name(&self) -> String {
        String::from("BI_TRANSFORM")
    }

    fn cloned(&self) -> Self {
        self.clone()
    }

    fn lua_call(&self, name: String, args: AnyUserData) {
        todo!()
    }
}

impl ComponentInitializer for Transform {
    type Inner = TransformInner;

    fn create(game_object: &GameObject) -> Self {
        Self(Arc::new(RwLock::new(TransformInner {
            parent: game_object.clone(),
            position: Vec3F::ZERO,
            rotation: QuatF::IDENTITY,
            scale: Vec3F::ONE,
        })))
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}