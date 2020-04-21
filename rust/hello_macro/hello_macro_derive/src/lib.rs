// proc_macro is the crate the compiler's  API that allows us to read and
// maniuplate Rust code from our code.
extern crate proc_macro;

use crate::proc_macro::TokenStream;
// `quote` turns `syn` data structures back into Rust code.
use quote::quote;
// `syn` parses Rust code from a string into a data structure that we can
// perform operations on.
use syn;

// The code for hello_macro_derive is the same in almost any procedural
// macro crate.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    //
    // NB: we must panic if we fail since we have to return a `TokenStream`
    // and not a `Result` to conform to the procedural macro API.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // `name` is an `Ident` struct instance containing the identifier of the
    // annotated type.
    let name = &ast.ident;
    // `quote!` lets us define the Rust code we wish to return.
    let gen = quote! {
        // `#name` is templating provided by `quote!`.
        impl HelloMacro for #name {
            fn hello_macro() {
                // `stringify!` turns expression literals into a string literal at
                // compile time.
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    // We use `into` to convert the `quote!` into a `TokenStream`.
    gen.into()
}
