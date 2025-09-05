use crate::component::Component;
use crate::game_object::GameObject;
use mlua::{AnyUserData, FromLua, MetaMethod, UserData, UserDataMethods};
use std::rc::Rc;

#[derive(FromLua)]
pub struct LuaComponentRef(pub(crate) Rc<Box<dyn Component>>);

impl Component for LuaComponentRef {
    fn parent(&self) -> GameObject {
        self.0.parent()
    }

    fn name(&self) -> String {
        self.0.name()
    }

    fn cloned(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }

    fn lua_call(&self, name: String, args: AnyUserData) {
        self.0.lua_call(name, args)
    }
}

impl Clone for LuaComponentRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl UserData for LuaComponentRef {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("parent", |_, this, ()| Ok(this.parent()));
        methods.add_meta_method(MetaMethod::Call, |_, this, (method_name, args): (String, AnyUserData)| {
            Ok(this.lua_call(method_name, args))
        });
    }
}
