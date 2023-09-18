use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn clone_getter_proc_macro_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let struct_data = if let syn::Data::Struct(struct_data) = data {
        struct_data
    } else {
        panic!("CloneGetters only supports structs");
    };
    let getters = struct_data
        .fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone().expect("Tuple structs are not supported");
            let field_type = &f.ty;
            quote! {
                pub fn #field_name(&self) -> #field_type {
                    self.#field_name.clone()
                }
            }
        })
        .collect::<Vec<_>>();
    let expanded = quote! {
        impl #ident {
            #(#getters)*
        }
    };
    expanded.into()
}
