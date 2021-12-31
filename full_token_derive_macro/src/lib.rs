use darling::{FromDeriveInput, FromField};
use macro_utils::{FieldData, GenericLoad};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(token))]
struct StructOps {
    name: Option<String>,
}

#[derive(FromField, Default)]
#[darling(default, attributes(token))]
struct FieldOps {
    name: Option<String>,
}

#[proc_macro_derive(FullToken, attributes(token))]
pub fn full_token_macro(input: TokenStream) -> TokenStream {
    let derive_token = parse_macro_input!(input as DeriveInput);

    let StructOps { name } = StructOps::from_derive_input(&derive_token).expect("Bad Struct Info");

    let DeriveInput {
        attrs: _a,
        vis: _v,
        ident,
        generics,
        data,
    } = derive_token;

    // 外部定义名称
    let name = if let Some(ident) = name {
        quote::quote! {#ident}
    } else {
        let name = stringify!(ident);
        quote::quote! {#name}
    };
    // 解析泛型
    let generics_loaded = macro_utils::load_generic(&generics);
    let field_loaded = if let Data::Struct(f) = data {
        macro_utils::load_fields(&f.fields, |t: FieldOps, n, _ty, _id| {
            if let Some(ident) = t.name {
                quote::quote! {#ident}
            } else {
                let ide=if let Some(ide)=n{
                    ide.clone()
                }else {
                    unreachable!("Only accept Named Struct")
                };
                 let ide = format!("{}",ide);
                quote::quote! {#ide}
            }
        })
    } else {
        unreachable!("Expect a Struct")
    };

    let name_impl = quote::quote! {
        fn name(&self) -> & 'static str{
            #name
        }
    };
    // 字段名称
    let all_names = field_loaded.iter().map(
        |FieldData {
             extra,
             ident: _,
             ftype: _,
         }| extra.clone(),
    );
    // 字段转换代码
    let get_data = field_loaded.iter().map(
        |FieldData {
             extra,
             ident,
             ftype,
         }| {
            let ident = if let Some(ident) = ident {
                ident.clone()
            } else {
                unreachable!();
            };
            quote::quote! {#extra =>  Some(<#ftype as crate::lexical::token_trait::IntoTokenMeta>::into_token_meta(self.#ident.clone())), }
        },
    );
    // 直段类型的约束
    let mut type_bounds = field_loaded
        .iter()
        .map(
            |FieldData {
                 extra: _,
                 ident: _,
                 ftype,
             }| {
                let ftype = ftype.clone();
                quote::quote! { #ftype:crate::lexical::token_trait::IntoTokenMeta+crate::lexical::token_trait::FromTokenMeta+Clone  }
            },
        )
        .collect::<Vec<_>>();

    let from_dyn=field_loaded.iter().map(|FieldData {
            extra,
            ident,
            ftype,
        }|
       { let ident = if let Some(ident) = ident {
            ident.clone()
        } else {
            unreachable!();
        };
        quote::quote! { #ident:<#ftype as crate::lexical::token_trait::FromTokenMeta>::from_token_meta(&src.get_data(#extra)?)? }
    }
    );

    // 泛型部分
    let GenericLoad {
        type_params,
        generic,
        mut where_clause,
    } = generics_loaded;

    let generics = generic
        .into_iter()
        .reduce(|l, r| quote::quote! {#l , #r })
        .and_then(|t| Some(quote::quote! {<#t> }))
        .unwrap_or(quote::quote! {});

    let type_params = type_params
        .into_iter()
        .map(|t| quote::quote! {#t:crate::lexical::token_trait::IntoTokenMeta+crate::lexical::token_trait::FromTokenMeta});

    where_clause.append(&mut type_bounds);
    where_clause.extend(type_params);

    let where_clause = where_clause
        .into_iter()
        .reduce(|l, r| quote::quote! { #l , #r })
        .and_then(|t| Some(quote::quote! {where #t}))
        .unwrap_or(quote::quote! {});

    quote::quote! {
        impl #generics FullToken for #ident #generics #where_clause {
           #name_impl

           fn get_data(&self, name: &str) -> Option<crate::lexical::token_trait::TokenMeta> {
               match name {
                   #(#get_data)*
                   _=>None
               }
            }

            fn get_all_name(&self) -> Vec<&'static str> {
                vec![
                    #(#all_names),*
                ]
            }
        }

        impl #generics crate::lexical::token_trait::FromToken for #ident #generics #where_clause{
            fn token_name() -> &'static str{
                #name
            }
            fn from_token(src: Box<dyn FullToken>) -> Option<Self>{
                if src.name()==Self::token_name(){
                   Some(
                       Self{
                           #(#from_dyn),*
                        }
                    )
                }else{
                    None
                }
            }
        }

    }
    .into()
}
