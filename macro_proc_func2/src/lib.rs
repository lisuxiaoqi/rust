use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Result, Token};

struct MulParam {
    left: Expr,
    right: Expr,
}

impl Parse for MulParam {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.parse::<Expr>()?;
        let _ = input
            .parse::<Token![,]>()
            .map_err(|_| input.error("Only [,] is qualified delimiter"))?;
        let r = input.parse::<Expr>()?;
        if !input.is_empty() {
            return Err(input.error("Expected exactly two params"));
        }
        Ok(MulParam { left: l, right: r })
    }
}

#[proc_macro]
pub fn mul(input: TokenStream) -> TokenStream {
    let MulParam { left, right } = parse_macro_input!(input as MulParam);
    quote! {
        #left * #right
    }
    .into()
}
