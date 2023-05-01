#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let heap_size = args
        .to_string()
        .parse::<usize>()
        .expect("Cannot get heap size");

    format!(
        include_str!("../template/hat_main.rs"),
        heap_size,
        item.to_string()
    )
    .parse::<TokenStream>()
    .unwrap()
}
