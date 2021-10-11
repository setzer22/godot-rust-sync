use crate::prelude::*;

pub struct GetInstance {
    pub field: Ident,
    pub path: LitStr,
    pub owner_type: syn::Ident,
    pub instance_type: Type,
    pub get_kind: super::get_child::GetChildKind,
}

impl ToGodotSyncCode for GetInstance {
    fn on_ready(&self) -> TokenStream2 {
        let field = &self.field;
        let field_name = &self.field.to_string();
        let path = &self.path;
        let instance_type = &self.instance_type;
        let instance_type_name = quote! { #instance_type }.to_string();
        let owner_type = &self.owner_type;
        let owner_type_name = quote! { #owner_type }.to_string();

        let get_fn = match self.get_kind {
            GetChildKind::Get => quote! {get_node},
            GetChildKind::Find => quote! {find_node},
        };

        quote! {
            unsafe {
                // TODO: `node` name is hardcoded. Use the detected #[root_scene]!!
                let node = self.node.expect("Root scene node instanced").assume_safe();
                self.#field = Some(node.#get_fn(#path)
                    .expect(concat!("Child ", #field_name, " not found at ", #path))
                    .assume_safe()
                    .cast::<#owner_type>()
                    .expect(concat!("Child ", #field_name, " at ", #path, " could not be cast to ", #owner_type_name))
                    .cast_instance::<#instance_type>()
                    .expect(concat!("Child ", #field_name, " at ", #path, " could not be cast to instance type ", #instance_type_name))
                    .claim());
            }
        }
    }

    fn start_frame(&self) -> TokenStream2 {
        quote! {}
    }

    fn end_frame(&self) -> TokenStream2 {
        quote! {}
    }
}
