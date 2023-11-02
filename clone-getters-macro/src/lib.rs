#![warn(clippy::all, clippy::pedantic)]
use proc_macro::{self, TokenStream};

mod getters;

#[proc_macro_derive(CloneGetters)]
pub fn derive_macro_clone_getters(input: TokenStream) -> TokenStream {
    getters::clone_getter_proc_macro_impl(input)
}
