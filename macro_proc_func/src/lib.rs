use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, Token};

#[proc_macro]
pub fn add(input: TokenStream) -> TokenStream {
    let expr_lists = parse_macro_input!(input with Punctuated::<Expr,Token![,]>::parse_terminated);
    expr_lists.iter().for_each(|x| {
        eprintln!("parsed expr:{}", quote!(#x));
    });
    eprintln!("parsed expr end");

    let mut it = expr_lists.into_iter();
    let first = it.next().expect("At least one arg is required, boy!");
    let expanded = it.fold(quote! {#first}, |acc, item| {
        quote! {
            #acc + #item
        }
    });

    TokenStream::from(expanded)
}
