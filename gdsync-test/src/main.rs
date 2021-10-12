use gdsync_derives::GodotSync;

#[derive(GodotSync)]
pub struct MyPlayer {
    #[root_scene("res://Player.tscn")]
    player: Option<Ref<Node>>,

    #[get_node("MyNode/Foo")]
    foo: Option<Ref<Foo>>,

    #[get_instance(MySuperType, "MyNode/MyInstance")]
    bar: Option<Instance<Bar>>,
}

#[derive(GodotSync)]
pub struct MyPlayer2 {
    #[root_node]
    node: Option<Ref<Node>>,

    #[get_node("MyNode/Foo")]
    foo: Option<Ref<Foo>>,

    #[get_instance(MySuperType, "MyNode/MyInstance")]
    bar: Option<Instance<Bar>>,
}

pub fn main() {}
