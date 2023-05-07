#![feature(proc_macro_quote)]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ExprAssign, ItemFn};

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let item2 = item.clone();
    let main_task = parse_macro_input!(item2 as ItemFn);
    let heap_arg = parse_macro_input!(args as ExprAssign);

    if heap_arg.left.into_token_stream().to_string().to_lowercase() != "heap" {
        panic!("Heap size argument not found. Add `heap = <HEAP_SIZE>`");
    }

    if main_task.sig.asyncness.is_none() {
        panic!("The main task must be an async function");
    }

    let main_task_name = main_task.sig.ident.to_string();

    format!(
        include_str!("../template/cortex_m.rs"),
        main_task_name,
        heap_arg.right.into_token_stream().to_string(),
        item.to_string()
    )
    .parse::<TokenStream>()
    .unwrap()
}
