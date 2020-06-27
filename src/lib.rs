use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn testaun_case(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = input.sig.ident;
    let block = input.block;

    let fn_decl = quote! {
        fn #fn_name() {
            testaun_before();
            let result = std::panic::catch_unwind(|| #block);
            testaun_after();
            assert!(result.is_ok());
        }
    };

    let output = quote! {
        #fn_decl
    };
    // println!("output = {}", output);

    output.into()
}
