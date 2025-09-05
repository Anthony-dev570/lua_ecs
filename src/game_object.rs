use std::fmt::Debug;
use std::rc::Rc;
use crate::boxed::Boxed;
use crate::component::{Component, ComponentInitializer};
use crate::hash_bag::HashBag;
use std::sync::{Arc, Mutex, RwLock};
use mlua::{FromLua, MetaMethod, UserData, UserDataMethods};
use crate::component::lua_component_ref::LuaComponentRef;

static mut GAME_OBJECT_ITR: Option<Arc<Mutex<u32>>> = None;

#[allow(dead_code)]
pub struct GameObjectInner {
    id: u32,
    name: String,
    components: HashBag<String, Rc<Box<dyn Component>>>
}

#[derive(FromLua)]
pub struct GameObject(Arc<RwLock<GameObjectInner>>);

impl UserData for GameObject {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_component", |_, this, name: String| {
            Ok(this.get_lua_component(name))
        });
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{this:?}")))
    }
}

impl GameObject {
    pub fn new() -> GameObject {
        let id = Self::inc();
        GameObject(Arc::new(RwLock::new(GameObjectInner {
            id,
            name: format!("GameObject#{}", id),
            components: Default::default(),
        })))
    }

    pub fn get_lua_component(&self, name: String) -> Option<LuaComponentRef> {
        let inner = self.0.read().unwrap();
        match inner.components.get(&name) {
            None => None,
            Some(c) => {
                match c.is_empty() {
                    true => None,
                    false => Some(LuaComponentRef(Rc::clone(&c[0])))
                }
            }
        }
    }

    pub fn add_component<C: Component + ComponentInitializer + Clone + 'static>(&self) -> C {
        let component = C::create(self);
        let name = component.name();
        let output = component.clone();

        self.0
            .write()
            .unwrap()
            .components
            .insert(name, Rc::new(component.boxed() as Box<dyn Component>));

        output
    }

    fn inc() -> u32 {
        unsafe {
            #[allow(static_mut_refs)]
            if GAME_OBJECT_ITR.is_none() {
                GAME_OBJECT_ITR = Some(Arc::new(Mutex::new(0)));
            }
            #[allow(static_mut_refs)]
            if let Some(itr) = GAME_OBJECT_ITR.as_mut() {
                let mut lock = itr.lock().unwrap();
                let id = *lock;
                *lock = *lock + 1;
                return id;
            }
            panic!("Not possible")
        }
    }
}

impl Clone for GameObject {
    fn clone(&self) -> Self {
        GameObject(self.0.clone())
    }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", self.0.read().unwrap().name))
    }
}