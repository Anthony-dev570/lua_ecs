pub mod component;
pub mod game_object;
pub mod hash_bag;
pub mod boxed;

#[cfg(test)]
mod tests {
    use crate::component::camera::Camera;
    use crate::game_object::GameObject;
    use lua_mathematics::angle::AngleF;
    use rlua::Lua;

    #[test]
    fn it_works() {
        let go = GameObject::new();
        let _camera = go.add_component::<Camera>();

        let lua = Lua::new();

        AngleF::load_lua(&lua).unwrap();
        Camera::load_lua(&lua).unwrap();

        lua.load(include_str!("../lua_tests/camera_test.lua")).exec().unwrap();
    }
}
