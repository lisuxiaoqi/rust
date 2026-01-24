use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Block, FnArg, ItemFn, LitStr, Pat, PatIdent};
mod recur;

#[proc_macro_attribute]
pub fn log_call(attr: TokenStream, input: TokenStream) -> TokenStream {
    eprintln!("attr:{}", &attr);
    eprintln!("input:{}", &input);

    //create parser for attr parse
    let attr_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("subattr1") {
            let value = meta.value()?;
            let s: LitStr = value.parse()?;
            eprintln!("\tsubattr1: value:{}", s.value());
            Ok(())
        } else if meta.path.is_ident("subattr2") {
            eprintln!("\tsubattr2: is set");
            Ok(())
        } else if meta.path.is_ident("subattr3") {
            meta.parse_nested_meta(|meta| {
                eprintln!(
                    "\tsubattr3: sub meta: {:}",
                    meta.path.get_ident().unwrap().to_string()
                );
                Ok(())
            })
            .unwrap();
            Ok(())
        } else {
            Err(meta.error("unsupported meta"))
        }
    });

    //use attr_parser
    parse_macro_input!(attr with attr_parser);

    //debug input information
    //parse sig and block
    let fn_input = parse_macro_input!(input as ItemFn);
    let sig = &fn_input.sig;
    let block: &Box<Block> = &fn_input.block;
    eprintln!("fn sig:{}", sig.to_token_stream().to_string());
    eprintln!("fn block:{}", block.to_token_stream().to_string());

    //debug sig, params
    let sig_ident = &sig.ident;
    eprintln!("sig_ident:{}, params len:{}", sig_ident, sig.inputs.len());

    sig.inputs.iter().for_each(|arg| {
        eprintln!("\t fnArg:{}", arg.to_token_stream().to_string());
        if let FnArg::Typed(pt) = arg {
            if let Pat::Ident(PatIdent { ref ident, .. }) = *pt.pat {
                let ty = &pt.ty;
                eprintln!(
                    "\t\t param:{}, Type:{}",
                    ident,
                    ty.to_token_stream().to_string()
                );
            }
        }
    });

    //generate param lists
    let param_lists: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pt) = arg {
                if let Pat::Ident(PatIdent { ref ident, .. }) = *pt.pat {
                    Some(quote! {
                        println!("param name:{}, value:{}", stringify!(#ident), #ident )
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    quote! {
        #sig{
            println!("[Log Start], enter fn: {}", stringify!(#sig_ident));
            #(#param_lists;)*
            let __result = (||#block)();
            println!("[Log End], exit fn: {}, result:{}", stringify!(#sig_ident), __result);
            __result
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn recur(attr: TokenStream, input: TokenStream) -> TokenStream {
    recur::recur_call(attr, input)
}
