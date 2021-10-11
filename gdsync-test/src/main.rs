use gdsync_derives::GodotSync;

#[derive(GodotSync)]
pub struct MyPlayer {
    #[root_scene("res://Player.tscn")]
    player: Option<Ref<Node>>,
}

pub fn main() {}
