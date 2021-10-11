use gdsync_derives::GodotSync;

#[derive(GodotSync)]
pub struct MyPlayer {
    #[root_scene("res://Player.tscn")]
    player: Option<Ref<Node>>,

    #[get_node("MyNode/Foo")]
    foo: Option<Ref<Foo>>,

    #[get_instance(MySuperType, "MyNode/MyInstance")]
    foo: Option<Ref<Bar>>,
}

pub fn main() {}
