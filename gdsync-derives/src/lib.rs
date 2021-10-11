mod derive_macro;
mod prelude;
mod syn_utils;
mod sync_actions;

#[proc_macro_derive(GodotSync, attributes(root_scene, get_node, find_node))]
pub fn godot_sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_macro::godot_sync_main(input)
}

// Here's a rough sketch of the API we want to achieve:

/*
#[derive(GodotSync)]
pub struct MyPlayer {
    /// The root scene node. A `.instance()` function will be generated for this struct, spawning the
    /// referenced scene and the node will be stored in `node`
    #[root_scene("res://scenes/Player.tscn")]
    pub node: Ref<Node>,

    /// You can fetch child nodes and store them as `Ref` variables inside this struct. The type is
    /// automatically inferred from the declared type, in this case `KinematicBody`
    #[get_child(".")]
    pub body: Ref<KinematicBody>,

    /// Just like in godot, `get_child("...")` accepts a path.
    #[get_child("PlayerModel/MeshInstance")]
    pub player_model: Ref<Spatial>,

    /// You can declare varaibles and define data mappings for them. Functions will be generated to extract
    /// the data from the referenced nodes at the start of the frame, and sync it back to scene nodes at the
    /// end of the frame. In this case, we're declaring a bidirectional sync for the transform property, which
    /// is read at the start of the frame from `body`, and set at the end of the frame to whatever value this
    /// `transform` variable currently has.
    #[sync_in_out(body, get = ".global_transform()", set = ".set_global_transform({})")]
    pub transform: Transform,

    /// Unidirectional syncs can also be done. These will generate read-only variables (hence, why not `pub`)
    /// that are set at the start of the frame. An immutable "getter" function will be generated.
    #[sync_in(transform, ".basis.forward")]
    forward: Vector3,

    #[find_child("SkeletonIK")]
    pub skeleton: Ref<SkeletonIK>,

    /// There can exist other types of sync, like `sync_once`, which reads once at the start of the frame. It
    /// is useful for caching results that will not change throughout the game's lifetime.
    #[sync_once(skeleton, ".find_bone(\"arm_L\")")]
    pub left_arm: String,


    #[find_child("AnimationPlayer")]
    pub player: Ref<AnimationPlayer>,

    /// Any complex sync logic that won't fit in a one-liner can be specified with external functions.
    #[sync_in_out_with("::get_current_anim_track", "::set_current_anim_track")]
    pub current_track: String,
}
*/
