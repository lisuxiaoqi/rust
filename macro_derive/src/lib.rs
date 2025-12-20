use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Meta};

#[proc_macro_derive(Builder, attributes(skip, default))]
pub fn name_isnt_important(input: TokenStream) -> TokenStream {
    eprintln!("input:{}", input);
    let input = parse_macro_input!(input as DeriveInput);

    //get ident
    let ident = input.ident;
    eprintln!("input ident:{}", ident);

    //create builder name from ident
    let builder_name = Ident::new(&format!("{}Builder", ident), ident.span());
    eprintln!("builder name:{}", builder_name);

    //get fields
    let fields = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!("only FieldsNamed are supported"),
        },
        _ => unimplemented!("only structs are supported"),
    };

    //debug fields info
    fields.iter().for_each(|f| {
        eprintln!(
            "\tfiled=> ident:{:}, ty:{:}",
            f.ident.to_token_stream().to_string(),
            f.ty.to_token_stream().to_string(),
        );
        let attr = &f.attrs;
        attr.iter().for_each(|a| {
            eprintln!(
                "\t\tattr=>path:{} meta:{}",
                a.path().to_token_stream().to_string(),
                a.meta.to_token_stream().to_string()
            );
        });
    });

    let builder_idents = fields.iter().filter_map(|f| {
        let ident = &f.ident;
        let mut skip = false;

        for attr in &f.attrs {
            if attr.path().is_ident("skip") {
                skip = true;
            }
        }

        if skip {
            None
        } else {
            Some(quote! {
                #ident
            })
        }
    });

    let builder_fields: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            let mut skip = false;

            for attr in &f.attrs {
                if attr.path().is_ident("skip") {
                    skip = true;
                }
            }

            if skip {
                None
            } else {
                Some(quote! {
                    #ident: Option<#ty>
                })
            }
        })
        .collect();

    let builder_setters: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            let mut skip = false;

            for attr in &f.attrs {
                if attr.path().is_ident("skip") {
                    skip = true;
                }
            }

            if skip {
                None
            } else {
                Some(quote! {
                    pub fn #ident(mut self, #ident:#ty)->Self{
                        self.#ident = Some(#ident);
                        self
                    }
                })
            }
        })
        .collect();

    let built_fields: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let mut skip = false;
            let mut default = None;

            for attr in &f.attrs {
                if attr.path().is_ident("skip") {
                    skip = true;
                }

                if attr.path().is_ident("default") {
                    if let Meta::NameValue(ref nv) = attr.meta {
                        default = Some(&nv.value);
                    }
                }
            }

            if skip {
                quote! {
                    #ident: Default::default()
                }
            } else if let Some(expr) = default {
                quote! {
                    #ident: self.#ident.clone().unwrap_or_else(||::core::convert::Into::into(#expr))
                }
            } else {
                quote! {
                    #ident: self.#ident.clone().expect(concat!("Field ", stringify!(#ident), "is not set"))
                }
            }
        })
        .collect();

    quote! {
        #[derive(Debug)]
        struct #builder_name{
            #(#builder_fields,)*
        }

        impl #builder_name{
            //setter
            #(#builder_setters)*

            pub fn build(self)->#ident{
                #ident{
                    #(#built_fields,)*
                }
            }
        }

        impl #ident{
            pub fn builder()->#builder_name{
                #builder_name{
                    #(#builder_idents:None,)*
                }
            }
        }
    }
    .into()
}
