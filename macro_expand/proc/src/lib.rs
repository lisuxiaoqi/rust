use proc_macro::TokenStream;
mod pallet;
mod runtime;

#[proc_macro_attribute]
pub fn runtime(attr: TokenStream, input: TokenStream) -> TokenStream {
    runtime::expand(attr, input)
}

#[proc_macro_attribute]
pub fn pallet(attr: TokenStream, input: TokenStream) -> TokenStream {
    pallet::expand(attr, input)
}
