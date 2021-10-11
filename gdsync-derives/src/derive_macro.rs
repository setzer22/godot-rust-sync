pub use crate::prelude::*;


pub(crate) fn godot_sync_main(input: TokenStream) -> TokenStream {
    let item_struct = match syn::parse::<syn::ItemStruct>(input) {
        Ok(str) => str,
        Err(err) => panic!("{}", err),
    };

    let ident = item_struct.ident;

    let mut godot_sync_actions: Vec<Box<dyn ToGodotSyncCode>> = vec![];

    match item_struct.fields {
        syn::Fields::Named(named) => {
            for field in named.named.iter() {
                for attr in field.attrs.iter() {
                    if attr.path.is_ident("root_scene") {
                        let ref_type = unwrap_option_ref(field.ty.clone())
                            .expect("Root scene should have Option<Ref<T>> as type");

                        let scene_path = attr
                            .parse_args::<syn::LitStr>()
                            .expect("root_scene attribute requires a single scene as argument")
                            .value();

                        godot_sync_actions.push(Box::new(RootScene {
                            field: field.ident.as_ref().expect("Only named fields supported").clone(),
                            ref_type,
                            scene_path,
                        }));
                    } else if attr.path.is_ident("get_node") || attr.path.is_ident("find_node") {
                        let ref_type = unwrap_option_ref(field.ty.clone())
                            .expect("Root scene should have Option<Ref<T>> as type");

                        let node_path = attr
                            .parse_args::<syn::LitStr>()
                            .expect("root_scene attribute requires a single scene as argument");

                        godot_sync_actions.push(Box::new(GetChild {
                            field: field.ident.as_ref().expect("Only named fields supported").clone(),
                            ref_type,
                            path: node_path,
                            get_kind: if attr.path.is_ident("get_node") {
                                GetChildKind::Get
                            } else {
                                GetChildKind::Find
                            },
                        }));
                    }
                }
            }
        }
        _ => todo!(),
    }

    let on_ready_code: Vec<TokenStream2> = godot_sync_actions.iter().map(|x| x.on_ready()).collect();
    let start_frame_code: Vec<TokenStream2> = godot_sync_actions.iter().map(|x| x.start_frame()).collect();
    let end_frame_code: Vec<TokenStream2> = godot_sync_actions.iter().map(|x| x.end_frame()).collect();

    let output = quote! {
        impl #ident {
            fn on_ready(&mut self) {
                #(#on_ready_code)*
            }

            fn start_frame(&mut self) {
                #(#start_frame_code)*
            }

            fn end_frame(&mut self) {
                #(#end_frame_code)*
            }
        }
    };

    output.into()
}