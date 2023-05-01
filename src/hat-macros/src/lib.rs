#![feature(proc_macro_quote)]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let _system_clock = "16_000_000";

    format!(include_str!("../template/hat_main.rs"), item.to_string())
        .parse::<TokenStream>()
        .unwrap()
}
