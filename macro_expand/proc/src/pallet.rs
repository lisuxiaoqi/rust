use proc_macro::TokenStream;
use quote::quote;

pub fn expand(_attr: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("--pallet expand, input:{}", input);
    quote! {
        use tt_call::{tt_call, tt_return};

        #[macro_export]
        macro_rules! pallet_a_produce{
            {
                $caller:tt
            }=>{
                tt_return!{
                    $caller
                    items = [{
                        ItemA1,
                        ItemA2,
                    }]
                }
            };
        }

    }
    .into()
}
