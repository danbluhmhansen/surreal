use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub trait SurrealCreate {
    fn create(&self) -> String;
}

pub fn impl_surreal_create(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;

    if let Data::Struct(s) = ast.data {
        if let Fields::Named(f) = s.fields {
            let mut first: Vec<String> = vec![];
            let mut second: Vec<String> = vec![];
            f.named
                .into_iter()
                .filter_map(|f| f.ident)
                .enumerate()
                .for_each(|(i, f)| {
                    first.push(format!("{f} = {{{i}}}"));
                    second.push(format!("self.{f}"));
                });
            let first = first.join(", ");
            let second = second.join(", ");
            let snake = name.to_string().to_case(Case::Snake);

            let body: TokenStream = format!(r#"format!("CREATE {snake} SET {first};", {second})"#)
                .parse()
                .unwrap();

            quote! {
                impl SurrealCreate for #name {
                    fn create(&self) -> String {
                        #body
                    }
                }
            }
        } else {
            quote! {
                impl SurrealCreate for #name {
                    fn create(&self) -> String {
                        format!("CREATE #name SET")
                    }
                }
            }
        }
    } else {
        quote! {
            impl SurrealCreate for #name {
                fn create(&self) -> String {
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::impl_surreal_create;
    use quote::quote;

    #[test]
    fn foo() {
        let ast = syn::parse2(quote! {
            struct FooBar {
                foo: i32,
                bar: String,
            }
        })
        .unwrap();
        assert_eq!(
            impl_surreal_create(ast).to_string(),
            quote! {
                impl SurrealCreate for FooBar {
                    fn create(&self) -> String {
                        format!("CREATE foo_bar SET foo = {0}, bar = {1};", self.foo, self.bar)
                    }
                }
            }
            .to_string()
        );
    }
}
