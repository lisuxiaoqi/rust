use proc_macro::TokenStream;
use quote::quote;

pub fn expand(_attr: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("--runtime expand, input:{}", input);
    quote! {
        use tt_call::{tt_call, tt_return};

        macro_rules! mixer{
            {
                $caller:tt
                parts = [{ }]
                pre = [{ $($pre:tt,)* }]
                items = [{ $($new:tt,)* }]
            }=>{
                #[derive(PartialEq, Debug)]
                enum Runtime{
                    $($pre,)*
                    $($new,)*
                }
            };

            {
                $caller:tt
                parts = [{ $first:tt $($rest:tt)* }]
                pre = [{ $($pre:tt,)* }]
                items = [{ $($new:tt,)* }]
            }=>{
                tt_call!{
                    macro = [{ $first }]
                    ~~>mixer!{
                        $caller
                        parts = [{ $($rest)* }]
                        pre = [{
                            $($pre,)*
                            $($new,)*
                        }]
                    }
                }
            };
        }

        tt_call! {
            macro = [{ mixer }]
            parts = [{pallet_a_produce}]
            pre = [{}]
            items = [{}]
        }
    }
    .into()
}
