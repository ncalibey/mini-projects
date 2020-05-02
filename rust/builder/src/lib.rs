extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::parenthesized;

use syn::parse::Result as SynResult;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse type to derive Builder for");

    impl_builder_macro(ast)
}

fn impl_builder_macro(ty: syn::DeriveInput) -> TokenStream {
    // The point of `parse_builder_information` is to use proc_macro2; the returning
    // `parse_builder_information::TokenStream` implements Into for a
    // `proc_macro::TokenStream`.
    match parse_builder_information(ty) {
        Ok(info) => info.into(),
        Err(e) => to_compile_errors(e).into(),
    }
}

// We use this to convert to the more standard style of error propagation, but making
// sure to convert it to a token stream so `proc_macro` can handle it correctly.
fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    // Since we have a vector of errors, we use this syntax to convert each element.
    // The `#compile_errors` is variable interpolation.
    quote! { #(#compile_errors)* }
}

// Type alias for `Result`.
type MultiResult<T> = std::result::Result<T, Vec<syn::Error>>;

// Wrapper struct to make working with `Vec<syn::Error>` easier.
#[derive(Debug, Default)]
struct SyntaxErrors {
    inner: Vec<syn::Error>,
}

impl SyntaxErrors {
    fn add<D, T>(&mut self, tts: T, description: D)
    where
        D: fmt::Display,
        T: quote::ToTokens,
    {
        // `new_spanned` is used to get info from the `tts` to tell the compiler
        // where to draw errors when printing the error out. A span is essentially
        // a region of source code in Rust.
        self.inner.push(syn::Error::new_spanned(tts, description));
    }

    fn extend(&mut self, errors: Vec<syn::Error>) {
        self.inner.extend(errors);
    }

    // This consumes the wrapper to return a `MultiResult`. This allows us to
    // use the `?` operator to report as many errors as you can diagnose at once.
    fn finish(self) -> MultiResult<()> {
        if self.inner.is_empty() {
            Ok(())
        } else {
            Err(self.inner)
        }
    }
}

// This contains all the info we need from the parsed AST to generate the needed code.
struct BuilderInfo {
    name: syn::Ident,
    generics: syn::Generics,
    fields: Vec<(Option<syn::Ident>, syn::Type, Vec<BuilderAttribute>)>,
}

fn parse_builder_information(ty: syn::DeriveInput) -> MultiResult<BuilderInfo> {
    use syn::spanned::Spanned;
    use syn::Data;

    let span = ty.span();
    // Destrcture into specific consitutent parts.
    let syn::DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = ty;

    match data {
        // we match against `Data::Struct` to make sure the item we are defined on
        // is a struct.
        Data::Struct(struct_) => parse_builder_struct(struct_, ident, generics, attrs, span),
        _ => Err(vec![syn::Error::new(
            span,
            "Can only derive `Builder` for a struct",
        )]),
    }
}

impl From<BuilderInfo> for TokenStream {
    fn from(other: BuilderInfo) -> TokenStream {
        other.generate_builder().into()
    }
}

impl BuilderInfo {
    fn generate_builder(self) -> proc_macro2::TokenStream {
        // `call_site` specifies that our macro hygenie is one that would conflict with an
        // existing name but that we have to use later.
        let gen_typ = syn::Ident::new("__Builder_T", proc_macro2::Span::call_site());

        // Generate setters for each field.
        let setters = self.fields.iter().map(|(n, t, _)| {
            quote! {
                fn #n<#gen_typ: Into<#t>>(mut self, val: #gen_typ) -> Self {
                    self.#n = Some(val.into());
                    self
                }
            }
        });

        // Create the fields for the builder struct. These are needed to build up the
        // larger structure.
        let builder_fields = self.fields.iter().map(|(n, t, _)| {
            quote! {
                #n: Option<#t>,
            }
        });

        // Create default fields for our struct, thus allowing us to implement the `Default`
        // trait.
        let builder_defaults = self.fields.iter().map(|(n, _, _)| {
            quote! {
                #n: None,
            }
        });

        let builder_build = self.fields.iter().map(|(n, _t, a)| {
            if a.is_empty() {
                quote! {
                    #n: self.#n.unwrap_or_else(Default::default),
                }
            } else {
                quote! {
                    #n: self.#n.unwrap(),
                }
            }
        });

        let name = self.name;
        let (impl_generics, ty_generics, maybe_where) = self.generics.split_for_impl();
        let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

        quote! {
            impl #impl_generics #name #ty_generics #maybe_where {
                fn builder() -> #builder_name #ty_generics {
                    #builder_name::new()
                }
            }

            impl #impl_generics Default for #builder_name #ty_generics #maybe_where {
                fn default() -> Self {
                    #builder_name {
                        #(#builder_defaults)*
                    }
                }
            }

            struct #builder_name #ty_generics #maybe_where {
                #(#builder_fields)*
            }

            impl #impl_generics #builder_name #ty_generics #maybe_where {
                fn new() -> Self {
                    Default::default()
                }

                #(#setters)*

                fn build(self) -> #name #ty_generics {
                    #name {
                        #(#builder_build)*
                    }
                }
            }
        }
    }
}

fn parse_builder_struct(
    struct_: syn::DataStruct,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    span: proc_macro2::Span,
) -> MultiResult<BuilderInfo> {
    // First check the attrs defined on the struct for any `builder`s.
    use syn::Fields;

    let mut errors = SyntaxErrors::default();

    for attr in attributes_from_syn(attrs)? {
        match attr {
            // `#[builder(required)]` cannot be used on the struct itself, only fields.
            BuilderAttribute::Required(tts) => {
                errors.add(tts, "required is only valid on a field");
            }
        }
    }

    let fields = match struct_.fields {
        // Only named fields can use `#required`, i.e. no tuple structs.
        Fields::Named(fields) => fields,
        _ => {
            errors.extend(vec![syn::Error::new(
                span,
                "only named fields are supported",
            )]);
            return Err(errors
                .finish()
                .expect_err("just added an error so there should be one"));
        }
    };

    let fields = fields
        .named
        .into_iter()
        .map(|f| match attributes_from_syn(f.attrs) {
            Ok(attrs) => (f.ident, f.ty, attrs),
            Err(e) => {
                errors.extend(e);
                (f.ident, f.ty, vec![])
            }
        })
        .collect();

    errors.finish()?;

    Ok(BuilderInfo {
        name,
        generics,
        fields,
    })
}

/// Converts a vector of `syn::Attribute` into a vector of `BuilderAttribute`.
fn attributes_from_syn(attrs: Vec<syn::Attribute>) -> MultiResult<Vec<BuilderAttribute>> {
    use syn::parse2;

    let mut ours = Vec::new();
    let mut errs = Vec::new();

    // `filter_map` filters values and returns them wrapped in a `Some` (doesn't return the `None`s).
    let parsed_attrs = attrs.into_iter().filter_map(|attr| {
        // Only keep `builder` attributes.
        if attr.path.is_ident("builder") {
            // Reply off our trait implementation to parse the `builder` trait and get at the
            // `Ok` variant.
            Some(parse2::<BuilderAttributeBody>(attr.tokens).map(|body| body.0))
        } else {
            None
        }
    });

    for attr in parsed_attrs {
        match attr {
            Ok(v) => ours.extend(v),
            Err(e) => errs.push(e),
        }
    }

    if errs.is_empty() {
        Ok(ours)
    } else {
        Err(errs)
    }
}

enum BuilderAttribute {
    Required(proc_macro2::TokenStream),
}

impl syn::parse::Parse for BuilderAttribute {
    fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
        use syn::Ident;

        let input_tts = input.cursor().token_stream();
        let name: Ident = input.parse()?;
        if name == "required" {
            Ok(BuilderAttribute::Required(input_tts))
        } else {
            Err(syn::Error::new(name.span(), "expected `required`"))
        }
    }
}

struct BuilderAttributeBody(Vec<BuilderAttribute>);

impl syn::parse::Parse for BuilderAttributeBody {
    fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
        use syn::punctuated::Punctuated;
        use syn::token::Comma;

        let inside;
        // Puts inner tokens between () for `#[builder(required)]` into `inside`.
        parenthesized!(inside in input);

        // Parse a sequence of `BuilderAttribute` types separated by commas (with opt. trailing comma).
        let parse_comma_list = Punctuated::<BuilderAttribute, Comma>::parse_terminated;
        // Extract the `BuilderAttribute` types from the the punctuated list.
        let list = parse_comma_list(&inside)?;

        Ok(BuilderAttributeBody(
            list.into_pairs().map(|p| p.into_value()).collect(),
        ))
    }
}
