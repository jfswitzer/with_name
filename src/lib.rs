
#[proc_macro_attribute]
pub fn with_name(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = input.ident.to_string();
    let const_decl = quote::quote! {
        const THIS_FN: &str = #fn_name;
    };

    input.block.stmts.insert(0, syn::parse(const_decl.into()).unwrap());

    let output = quote::quote! {
        #input
    };

    output.into()
}
