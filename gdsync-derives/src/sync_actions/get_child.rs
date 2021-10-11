use crate::prelude::*;

pub enum GetChildKind {
    Get, Find,
}

pub struct GetChild {
    pub field: Ident,
    pub path: LitStr,
    pub ref_type: Type,
    pub get_kind: GetChildKind,
}

impl ToGodotSyncCode for GetChild {
    fn on_ready(&self) -> TokenStream2 {
        let field = &self.field;
        let field_name = &self.field.to_string();
        let path = &self.path;
        let ref_type = &self.ref_type;
        let type_name = quote! { #ref_type }.to_string();

        let get_fn = match self.get_kind {
            GetChildKind::Get => quote!{get_node},
            GetChildKind::Find => quote!{find_node},
        };

        quote! {
            unsafe {
                let node = self.node.assume_safe();
                self.#field = node.#get_fn(#path)
                    .expect(concat!("Child ", #field_name, " not found at ", #path))
                    .assume_safe()
                    .cast::<#ref_type>()
                    .expect(concat!("Child ", #field_name, " at ", #path, " could not be cast to ", #type_name))
                    .claim();
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
