use r#macro::impl_surreal_create;

#[proc_macro_derive(SurrealCreate)]
pub fn surreal_create_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let ast = syn::parse2(input).unwrap();
    let output: proc_macro2::TokenStream = impl_surreal_create(ast);
    proc_macro::TokenStream::from(output)
}
