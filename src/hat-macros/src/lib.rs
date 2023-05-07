#![feature(proc_macro_quote)]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ExprAssign};

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let heap_arg = parse_macro_input!(args as ExprAssign);

    if heap_arg.left.into_token_stream().to_string().to_lowercase() != "heap" {
        panic!("Heap size argument not found. Add `heap = <HEAP_SIZE>`");
    }

    format!(
        include_str!("../template/hat_main.rs"),
        heap_arg.right.into_token_stream().to_string(),
        item.to_string()
    )
    .parse::<TokenStream>()
    .unwrap()
}
