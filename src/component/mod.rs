use mlua::AnyUserData;
use crate::game_object::GameObject;

pub mod camera;
pub mod transform;
pub mod lua_component_ref;

pub trait Component {
    fn parent(&self) -> GameObject;
    fn name(&self) -> String;
    fn cloned(&self) -> Self where Self: Sized;

    fn lua_call(&self, name: String, args: AnyUserData) -> Option<AnyUserData>;
}

pub trait ComponentInitializer: Component {
    type Inner;
    fn create(game_object: &GameObject) -> Self;
}

#[macro_export]
macro_rules! component_lua_parent_functions {
    (
        $methods:ident
    ) => {
        $methods.add_method("parent", |_, this, ()| {
            Ok(this.parent())
        });
    };
}