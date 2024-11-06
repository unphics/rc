use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::Item;
use quote::quote;

mod builder;

#[proc_macro_attribute]
pub fn mytest_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("{:#?}",parse_macro_input!(attr as AttributeArgs));
    let body_ast = parse_macro_input!(item as Item);
    eprintln!("{:#?}", body_ast);
    return quote!(#body_ast).into();
}

/**
 * 过程宏其实是在原struct后追加了一个struct定义, 被过程宏修饰后原宏定义仍然存在
 */
#[proc_macro_derive(Builder)] // 宏的名字叫Builder
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let ret: TokenStream = match builder::do_extend(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into()
    };
    // let output: syn::File = syn::parse(ret.clone()).unwrap();
    // let _ = quote!(eprintln!("proc tokend: {:#?}", #output));
    return ret;
}