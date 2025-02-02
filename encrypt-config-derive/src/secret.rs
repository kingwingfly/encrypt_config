use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr};

pub(crate) fn derive_secret_source(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut path_or_name: Option<Expr> = None;
    let mut keyring_entry: Option<Expr> = None;

    if let Some(attr) = input
        .attrs
        .iter()
        .find(|&attr| attr.path().is_ident("source"))
    {
        attr.parse_nested_meta(|meta| {
            match &meta.path {
                #[cfg(not(feature = "default_config_dir"))]
                path if path.is_ident("path") => {
                    let value = meta.value()?; // this parses the `=`
                    path_or_name = value.parse().ok();
                }
                #[cfg(feature = "default_config_dir")]
                path if path.is_ident("name") => {
                    let value = meta.value()?; // this parses the `=`
                    path_or_name = value.parse().ok();
                }
                path if path.is_ident("keyring_entry") => {
                    let value = meta.value()?; // this parses the `=`
                    keyring_entry = value.parse().ok();
                }
                _ => Err(meta.error("unsupported attribute"))?,
            }
            Ok(())
        })
        .unwrap();
    };

    if path_or_name.is_none() {
        #[cfg(feature = "default_config_dir")]
        panic!("`#[source(name = \"...\")]` is required.");
        #[cfg(not(feature = "default_config_dir"))]
        panic!("`#[source(path = \"...\")]` is required.");
    }

    if keyring_entry.is_none() {
        panic!("`#[source(keyring_entry = \"...\")]` is required.");
    }

    #[cfg(not(feature = "default_config_dir"))]
    let secret_source_impl = quote! {
        impl #impl_generics ::encrypt_config::source::SecretSource for #name #ty_generics #where_clause {
            const PATH: &'static str = #path_or_name;
            const KEYRING_ENTRY: &'static str = #keyring_entry;
        }
    };

    #[cfg(feature = "default_config_dir")]
    let secret_source_impl = quote! {
        impl #impl_generics ::encrypt_config::source::SecretSource for #name #ty_generics #where_clause {
            const NAME: &'static str = #path_or_name;
            const KEYRING_ENTRY: &'static str = #keyring_entry;
        }
    };

    let expanded = quote! {
        #secret_source_impl

        impl #impl_generics ::encrypt_config::source::Cacheable for #name #ty_generics #where_clause {
            fn load() -> ::std::io::Result<Self>
            where
                Self: Sized,
            {
                <Self as ::encrypt_config::SecretSource>::load()
            }

            fn store(&self) -> ::std::io::Result<()> {
                <Self as ::encrypt_config::SecretSource>::store(self)
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}
