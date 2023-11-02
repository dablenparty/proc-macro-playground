use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn clone_getter_proc_macro_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let syn::Data::Struct(struct_data) = data else {
        panic!("CloneGetters only supports structs");
    };
    let getters = struct_data
        .fields
        .iter()
        .map(|f| {
            let field_name = f.ident.clone().expect("Tuple structs are not supported");
            let field_type = &f.ty;
            let fn_name = format_ident!("{}_clone", field_name);
            quote! {
                pub fn #fn_name(&self) -> #field_type {
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
