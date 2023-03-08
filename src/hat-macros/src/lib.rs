mod entry_macro;

use crate::entry_macro::{entry_parse, entry_analyze, entry_codegen};

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = entry_parse(args.into(), item.into());
    let model = entry_analyze(ast);
    entry_codegen(model)
}
