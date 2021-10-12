use crate::prelude::*;

pub(crate) struct RootNode {
    pub field: syn::Ident,
    pub ref_type: syn::Type,
}

impl ToGodotSyncCode for RootNode {
    fn on_ready(&self) -> TokenStream2 { TokenStream2::new() }

    fn start_frame(&self) -> TokenStream2 { TokenStream2::new() }

    fn end_frame(&self) -> TokenStream2 { TokenStream2::new() }

    fn standalone_fns(&self) -> TokenStream2 {
        let field = &self.field;
        let ref_type = &self.ref_type;

        quote! {
            pub fn from_root_node(root_node: TRef<#ref_type>) -> Self {
                let mut new = Self {
                    node: Some(root_node.claim()),
                    ..Default::default()
                };
                new.on_ready();
                new
            }
        }
    }
}
