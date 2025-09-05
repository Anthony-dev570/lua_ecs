use crate::component::{Component, ComponentInitializer};
use crate::game_object::GameObject;
use lua_mathematics::angle::{Angle, AngleF};
use lua_mathematics::matrix::matrix4x4::Matrix4x4F;
use mlua::{AnyUserData, Lua, MetaMethod};
use rlua::{FromLua, UserData, UserDataMethods};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex, RwLock};
use crate::boxed::Boxed;

static mut MAIN: Option<Arc<Mutex<Camera>>> = None;

pub struct CameraInner {
    parent: GameObject,

    fov: AngleF,
    near: f32,
    far: f32,

    perspective: Matrix4x4F,
}

#[derive(FromLua)]
pub struct Camera(Arc<RwLock<CameraInner>>);

impl Camera {
    pub fn main() -> Option<Camera> {
        unsafe {
            #[allow(static_mut_refs)]
            if let Some(t) = &MAIN {
                let camera = t.lock().unwrap();
                return Some(camera.clone());
            }
            None
        }
    }

    pub fn load_lua(lua: &Lua) -> rlua::Result<()> {
        let main = lua.create_function(|_, ()| Ok(Camera::main()))?;
        lua.globals().set("camera_main", main)?;
        Ok(())
    }

    pub fn set_fov(&self, fov: AngleF) {
        self.0.write().unwrap().fov = fov;
    }

    pub fn set_near(&self, near: f32) {
        self.0.write().unwrap().near = near;
    }

    pub fn set_far(&self, far: f32) {
        self.0.write().unwrap().far = far;
    }
}
impl UserData for Camera {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        crate::component_lua_parent_functions!(methods);

        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{this:?}")));

        methods.add_method("fov", |_, this, fov: Option<AngleF>| {
            if let Some(fov) = fov {
                this.set_fov(fov);
            }

            Ok(this.0.read().unwrap().fov)
        });

        methods.add_method("near", |_, this, fov: Option<f32>| {
            if let Some(f) = fov {
                this.set_near(f);
            }

            Ok(this.0.read().unwrap().near)
        });
        methods.add_method("far", |_, this, fov: Option<f32>| {
            if let Some(f) = fov {
                this.set_far(f);
            }

            Ok(this.0.read().unwrap().near)
        });
    }
}
impl Component for Camera {
    fn parent(&self) -> GameObject {
        self.0.read().unwrap().parent.clone()
    }

    fn name(&self) -> String {
        String::from("BI_CAMERA")
    }

    fn cloned(&self) -> Self {
        self.clone()
    }

    fn lua_call(&self, name: String, args: AnyUserData) -> Option<AnyUserData> {
        match &*name {
            "fov" => {
                if let Ok(fov) = args.borrow::<AngleF>() {
                    self.set_fov(*fov);
                }
            },
            &_ => {}
        }
        None
    }
}
impl ComponentInitializer for Camera {
    type Inner = CameraInner;

    fn create(game_object: &GameObject) -> Self {
        let out = Self(Arc::new(RwLock::new(Self::Inner {
            parent: game_object.clone(),
            perspective: Matrix4x4F::ZERO,
            fov: Angle::Degrees(90f32),
            near: 0.01f32,
            far: 1000f32
        })));
        unsafe {
            #[allow(static_mut_refs)]
            if MAIN.is_none() {
                MAIN = Some(Arc::new(Mutex::new(out.clone())));
            }
        }
        out
    }
}
impl Clone for Camera {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Debug for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.read().unwrap()))
    }
}
impl Debug for CameraInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Camera")
            .field("perspective", &self.perspective)
            .field("fov", &self.fov)
            .field("near", &self.near)
            .field("far", &self.far)
            .finish()
    }
}