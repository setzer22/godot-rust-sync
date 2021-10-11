use crate::prelude::*;

pub(crate) struct RootScene {
    pub field: syn::Ident,
    pub scene_path: String,
    pub ref_type: syn::Type,
}

impl ToGodotSyncCode for RootScene {
    fn on_ready(&self) -> TokenStream2 {
        let path = &self.scene_path;
        let field = &self.field;
        let ref_type = &self.ref_type;
        let ref_type_name = quote! {#ref_type}.to_string();

        quote! {
            //println!("Loading scene from {} and storing it in {}, while casting to {}", #path, #field, #ref_type);
            unsafe {
                let resource_loader = ResourceLoader::godot_singleton();
                let root_scene_packed = resource_loader.load(#path, "PackedScene", false)
                    .expect(concat!("Scene not found at path: ", #path));
                let root_scene_packed = root_scene_packed // Avoid dropped temporary
                    .assume_safe()
                    .cast::<PackedScene>()
                    .expect(concat!("Resource at ", #path, " could not be cast to PackedScene"));

                let root_scene_inst = root_scene_packed
                    .instance(0)
                    .expect(concat!("Scene at ", #path, " could not be instanced"))
                    .assume_safe();

                let root_node = root_scene_inst.cast::<#ref_type>()
                    .expect(concat!("Root node of scene at ", #path, "could not be cast to ", #ref_type_name));

                self.#field = Some(root_node.claim());
            }
        }
    }

    fn start_frame(&self) -> TokenStream2 { TokenStream2::new() }

    fn end_frame(&self) -> TokenStream2 { TokenStream2::new() }
}
