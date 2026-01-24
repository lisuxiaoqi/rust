/*
 * It's possible to recursive call macro self many times, unless exceed the max loop depth
 * */
use proc_macro::TokenStream;
use quote::quote;
pub fn recur_call(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //eprintln!("attr:{}", attr);
    //eprintln!("input:{}", input);

    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let name = input.ident.to_string();
    let result = match name.as_str() {
        "S" => quote! {#input #[recur]struct S1;},
        "S1" => quote! {#input #[recur]struct S2;},
        "S2" => quote! {#input #[recur]struct S3;},
        _ => quote! {},
    };
    result.into()
}
