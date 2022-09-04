use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn;
use syn::{AttributeArgs, FnArg, Ident, ItemFn, Lit, LitStr, NestedMeta, parse_macro_input, Pat, PathArguments, PathSegment, PatType, ReturnType, Type};
use syn::__private::TokenStream2;
use syn::punctuated::{Pair, Pairs};
use syn::token::Comma;

#[feature(reqwest)]
#[proc_macro_attribute]
pub fn reqwest_fn(attr: TokenStream, item:TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    // Build the trait implementation
    impl_reqwest_fn(&attr,&item)
}
struct ParseSignature<'a>{
    name:&'a Ident,
    args:Vec<TokenStream2>,
    body:BodyVariant,
    headers:Vec<TokenStream2>,
    func_generics:Vec<TokenStream2>,
    method:LitStr,
}
fn parse_input_pairs(pairs:Pairs<FnArg,Comma>) -> Vec<(Ident,PathSegment)> {
        let mut args = Vec::new();
        for pair in pairs{
            match pair {
                Pair::Punctuated(arg, _) => {
                    match arg {
                        FnArg::Receiver(_) => {}
                        FnArg::Typed(pat) => {
                            args.push(
                                (
                                    match &*pat.pat {
                                        Pat::Ident(iden) => {
                                            iden.ident.clone()
                                        },
                                        Pat::Struct(pat) => {
                                            format_ident!("{}",
                                    pat.path.segments.last().unwrap().ident.to_string().to_ascii_lowercase()
                                        )
                                        },
                                        Pat::TupleStruct(pat) => {
                                            format_ident!("{}",
                                    pat.path.segments.last().unwrap().ident.to_string().to_ascii_lowercase()
                                        )
                                        }
                                        Pat::TupleStruct(pat) => {
                                            match pat.pat.elems.first().unwrap() {
                                                Pat::Ident(iden) => {
                                                    iden.ident.clone()
                                                },
                                                _ => panic!("not supported pattern in variable position of fn args"),

                                            }
                                        }
                                        _ => panic!("not supported pattern in variable position of fn args"),
                                    },
                                    *pat.ty.clone())
                            )
                        },
                    }
                },
                Pair::End(arg) => {
                    match arg {
                        FnArg::Receiver(_) => {}
                        FnArg::Typed(pat) => {
                            args.push(
                                (
                                    match &*pat.pat {
                                        Pat::Ident(iden) => {
                                            iden.ident.clone()
                                        }
                                        _ => panic!("Expected variable names as fn arg patterns only.")
                                    },
                                    *pat.ty.clone())
                            )
                        },
                    }
                }
            }
        }
        args.into_iter()
            .map(|(var,ty)|{
                (var, match ty {
                    Type::Path(ty_path) => {
                        ty_path.path.segments
                            .pairs()
                            .into_iter()
                            .last()
                            .unwrap()
                            .into_value().clone()
                    },
                    _ => panic!("types in fn args must be simple path segmented types")
                })
            }).collect::<Vec<(Ident,PathSegment)>>()
}

fn parse_signature<'a>(attr:&'a AttributeArgs,item:&'a ItemFn) -> ParseSignature<'a>{
    let name = &item.sig.ident;
    let mut args_old = parse_input_pairs(item.sig.inputs.pairs().clone());
    let (
        args,
        headers,
        body,
        mut func_generics
    ) = {
        let mut args = Vec::new();
        let mut headers = Vec::new();
        let mut body = BodyVariant::None;
        let mut func_generics = Vec::new();
        func_generics.push(quote!(U:reqwest::IntoUrl));
        for (var,ty) in args_old.clone() {
            let var = var.to_token_stream();
            let ty_ident = ty.ident.clone();
            let generic = match ty.arguments {
                PathArguments::None => None,
                PathArguments::AngleBracketed(inner) => Some(
                    inner.args.last().unwrap().clone()
                ),
                _ => panic!("expecting generic args")
            };
            let ty_token = ty_ident.to_token_stream();
            if ty_ident == format_ident!("TypedHeader") {
                args.push(quote!(#var : #generic));
                headers.push(quote!(.headers({
                    use  axum::headers::HeaderMapExt;
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.typed_insert(#var);
                    headers
                    })
                ))
            } else if ty_ident == format_ident!("String") {
                args.push(quote!(#var : String));
                body = BodyVariant::String(var.clone());
            } else if ty_ident == format_ident!("Json") {
                args.push(quote!(#var : &#generic));
                body = BodyVariant::Json(var.clone());
            } else if ty_ident == format_ident!("Request") {
                args.push(quote!(#var : B));
                body = BodyVariant::Request(var.clone());
                func_generics.push(quote!(B:Into<reqwest::Body>))
            } else if ty_ident == format_ident!("Bytes") {
                args.push(quote!(#var : bytes::Bytes));
                body = BodyVariant::Bytes(var.clone());
            } else {

            }
        }
        (
            args,
            headers,
            body,
            func_generics
        )
    };
    let method = match attr[0].clone() {
        NestedMeta::Meta(_) => panic!("Only str args are supported: i.e \"get\" instead of get, \
          you get it?"),
        NestedMeta::Lit(lit) => match lit {
            Lit::Str(lit) => lit,
            _ => panic!("unsuported func args"),
        }
    };

    ParseSignature{
        method,name,args,headers,func_generics,body
    }
}

fn impl_reqwest_fn(attr:&AttributeArgs,item:&ItemFn) -> TokenStream {
    let ParseSignature{
        name,
        args,
        body,
        headers,
        mut func_generics,
        method } = parse_signature(attr, item);
    let body = {
        match body {
            BodyVariant::Json(var) => {
                quote!(.json(#var))
            }
            BodyVariant::String(var) => {
                quote!(.body(#var))
            }
            BodyVariant::Bytes(var) => {
                quote!(.body(#var))
            }
            BodyVariant::Request(var) => {
                quote!(.body(#var))
            }
            BodyVariant::None => {
                quote!()
            }
        }
    };
    let method = match method.value().as_str() {
                    "get" => {
                        quote!(.get(route))
                    },
                    "post" => {
                        quote!(.post(route))
                    },
                    "put" => {
                        quote!(.put(route))
                    },
                    "delete" => {
                        quote!(.delete(route))
                    },
                    "patch" => {
                        quote!(.patch(route))
                    },
                    "head" => {
                        quote!(.head(route))
                    },
                    _ => panic!("unsupported method in first arg")
    };
    let func_generics = {
        let mut list = Vec::new();
        let last = func_generics.pop().unwrap();
        for other in func_generics {
            list.push(quote!(#other,))
        }
        list.push(quote!(#last));
        list
    };

    let mod_name = format_ident!("client_derive_reqwest_fn_{}",name);
    let gen = quote! {
        pub mod #mod_name {
            use super::*;
            pub async fn #name<#(#func_generics)*>(
                client:&reqwest::Client,
                route:U,
                #(#args),*)
            -> Result<reqwest::Response,reqwest::Error> {
                client
                    #method
                    #(#headers)*
                    #body
                    .send()
                    .await
            }
        }
        #item
    };
    gen.into()
}

#[derive(Debug)]
enum BodyVariant{
    Json(TokenStream2),
    String(TokenStream2),
    Bytes(TokenStream2),
    Request(TokenStream2),
    None,
}