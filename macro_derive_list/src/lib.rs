use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(FieldHelper, attributes(add))]
pub fn gs_helper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    eprintln!("input ident:{}", ident);

    //get fields
    let fields = match input.data {
        Data::Struct(ref ds) => match ds.fields {
            Fields::Named(ref f) => &f.named,
            _ => unimplemented!("Only fieldsNamed are supported"),
        },
        _ => unimplemented!("Only structs are supported"),
    };

    //debug info
    fields.iter().for_each(|f| {
        let ident = f.ident.as_ref();
        let ty = &f.ty;
        let attrs = &f.attrs;

        eprintln!(
            "Field ident:{}, type:{}",
            ident.to_token_stream(),
            ty.to_token_stream()
        );

        for attr in attrs {
            let meta = &attr.meta;
            eprintln!("\tmeta:{}", meta.to_token_stream());
        }
    });

    //get helper functions vector
    let helper_fns: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let ident = f.ident.as_ref();
            let ty = &f.ty;
            let mut get_flag = false;
            let mut set_flag = false;
            let mut fns = Vec::new();

            for attr in &f.attrs {
                if attr.path().is_ident("add") {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("getter") {
                            get_flag = true;
                            return Ok(());
                        }

                        if meta.path.is_ident("setter") {
                            set_flag = true;
                            return Ok(());
                        }

                        Err(meta.error("unknow path"))
                    })
                    .unwrap();
                }
            }

            eprintln!(
                "field:{}, set_flag:{}, get_flag:{}",
                ident.to_token_stream(),
                set_flag,
                get_flag
            );

            if set_flag {
                let setter_name =
                    Ident::new(&format!("set_{}", ident.unwrap()), ident.unwrap().span());
                fns.push(quote! {
                    pub fn #setter_name(&mut self, #ident:#ty){
                        self.#ident=#ident;
                    }
                });
            }

            if get_flag {
                fns.push(quote! {
                    pub fn #ident(&self)->&#ty{
                        &self.#ident
                    }
                });
            }

            Some(quote! {
                #(#fns)*
            })
        })
        .collect();

    quote! {
        impl #ident{
            #(#helper_fns)*
        }
    }
    .into()
}
