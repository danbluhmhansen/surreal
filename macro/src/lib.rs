use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub trait SurrealCreate {
    fn create(&self) -> String;
}

pub fn impl_surreal_create(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;

    if let Data::Struct(s) = ast.data {
        if let Fields::Named(f) = s.fields {
            let mut first: Vec<String> = vec![];
            let mut second: Vec<String> = vec![];
            f.named.into_iter().enumerate().for_each(|(i, f)| {
                let ident = f.ident.unwrap();
                match f.ty {
                    Type::Path(p) => {
                        if let Some(s) = p.path.get_ident() {
                            if s.eq("String") {
                                first.push(format!("{0} = '{{{1}}}'", ident, i));
                            } else {
                                first.push(format!("{0} = {{{1}}}", ident, i));
                            }
                        }
                    }
                    _ => {}
                };
                second.push(format!("self.{0}", ident));
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
                        format!("CREATE foo_bar SET foo = {0}, bar = '{1}';", self.foo, self.bar)
                    }
                }
            }
            .to_string()
        );
    }
}
