use crate::prelude::*;

// Given a type like Option<T>, returns T, None otherwise
pub fn unwrap_generic(ty: Type, wrapper_type_is: &str) -> Option<Type> {
    if let Type::Path(ref typepath) = ty {
        let path = &typepath.path;


        let first = path.segments.iter().next()?;

        if !(first.ident.to_string() == wrapper_type_is) {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref generic_args) = first.arguments {
            let inner_ty = generic_args.args.iter().next()?;
            if let syn::GenericArgument::Type(ref inner_ty) = inner_ty {
                return Some(inner_ty.clone());
            }
        }
    }
    return None;
}

pub fn unwrap_option_ref(ty: Type) -> Option<Type> {
    Some(Some(ty).and_then(|ty| unwrap_generic(ty, "Option")).and_then(|ty| unwrap_generic(ty, "Ref"))?)
}

/// Returns the identifier for a plain type, e.g. `Node`, or None otherwise
pub fn type_ident(ty: Type) -> Option<syn::Ident> {
    if let Type::Path(ref typepath) = ty {
        let seg = typepath.path.segments.iter().next()?;
        return Some(seg.ident.clone());
    }

    return None;
}

#[test]
fn test_unwrap_type() {
    let ty = syn::parse2::<syn::Type>(quote! { Option<Ref<Node>> }).expect("Could parse");

    let inner = Some(ty)
        .and_then(|ty| unwrap_generic(ty, "Option"))
        .and_then(|ty| unwrap_generic(ty, "Ref"))
        .and_then(type_ident)
        .expect("Could unwrap");

    assert_eq!(inner.to_string(), "Node")
}
