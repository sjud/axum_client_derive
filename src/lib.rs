use proc_macro::TokenStream;
use syn::parse_macro_input;
use crate::reqwest_fn::impl_reqwest_fn;
use syn::{AttributeArgs, ItemFn};

mod reqwest_fn;


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

