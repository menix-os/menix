use proc_macro2::Span;
use quote::quote;
use syn::{
    ItemFn, ReturnType, Signature, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

// Make sure the given path starts with a given identifier
fn path_starts_with(path: &syn::Path, ident: &str) -> bool {
    path.segments
        .first()
        .map(|seg| seg.ident == ident)
        .unwrap_or(false)
}

fn is_return_type_empty(type_: &Type) -> bool {
    matches!(type_, Type::Tuple(tuple) if tuple.elems.is_empty())
}

#[derive(Clone, PartialEq, Eq)]
enum AttributeKind {
    Name(String),
    Depends(Vec<syn::Path>),
    Entails(Vec<syn::Path>),
}

#[derive(Clone)]
struct Attribute {
    span: Span,
    kind: AttributeKind,
}

#[derive(Clone)]
struct AttributeList {
    attributes: Vec<Attribute>,
}

impl Attribute {
    fn parse_name_attribute(span: Span, input: ParseStream) -> syn::Result<Self> {
        let _: syn::Token![=] = input.parse()?;
        let name_lit: syn::LitStr = input.parse()?;

        Ok(Self {
            span: span.join(name_lit.span()).unwrap(),
            kind: AttributeKind::Name(name_lit.value()),
        })
    }

    fn parse_depends_entails_attribute(
        input: ParseStream,
    ) -> syn::Result<Punctuated<syn::Path, syn::Token![,]>> {
        let _: syn::Token![=] = input.parse()?;

        let content;
        let _ = syn::bracketed!(content in input);

        Punctuated::<syn::Path, syn::Token![,]>::parse_terminated(&content)
    }
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name_ident: syn::Ident = input.parse()?;

        if name_ident == "name" {
            Self::parse_name_attribute(name_ident.span(), input)
        } else if name_ident == "depends" {
            Self::parse_depends_entails_attribute(input).map(|x| Self {
                span: name_ident.span().join(x.span()).unwrap(),
                kind: AttributeKind::Depends(x.iter().cloned().collect()),
            })
        } else if name_ident == "entails" {
            Self::parse_depends_entails_attribute(input).map(|x| Self {
                span: name_ident.span().join(x.span()).unwrap(),
                kind: AttributeKind::Entails(x.iter().cloned().collect()),
            })
        } else {
            Err(syn::Error::new_spanned(
                name_ident,
                "expected \"name\", \"depends\" or \"entails\"",
            ))
        }
    }
}

impl Parse for AttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: Punctuated::<Attribute, syn::Token![,]>::parse_terminated(input)?
                .iter()
                .cloned()
                .collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn task(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let AttributeList { mut attributes } = parse_macro_input!(attr);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
        ..
    } = parse_macro_input!(item);

    let Signature {
        ident,
        generics,
        inputs,
        output,
        ..
    } = sig;

    if !generics.params.is_empty() {
        return syn::Error::new_spanned(
            generics.params,
            "initgraph task must not have any generic parameters",
        )
        .to_compile_error()
        .into();
    }

    if !inputs.is_empty() {
        return syn::Error::new_spanned(
            inputs,
            "initgraph task must not have any input parameters",
        )
        .to_compile_error()
        .into();
    }

    if !matches!(&output, ReturnType::Default)
        && !matches!(&output, ReturnType::Type(_, type_) if is_return_type_empty(type_))
    {
        return syn::Error::new_spanned(output, "initgraph task must not have a return type")
            .to_compile_error()
            .into();
    }

    let mut other_attrs = vec![];

    for attr in attrs.iter() {
        let path = attr.meta.path();

        if path_starts_with(path, "initgraph") {
            if path.segments.len() != 2 || path.segments[1].ident != "task" {
                return syn::Error::new_spanned(attr, "unknown initgraph attribute")
                    .to_compile_error()
                    .into();
            }

            match attr.parse_args::<AttributeList>() {
                Ok(parsed) => attributes.extend_from_slice(&parsed.attributes),
                Err(err) => {
                    return err.to_compile_error().into();
                }
            }
        } else {
            other_attrs.push(attr);
        }
    }

    let display_name = {
        let mut name_attrs = attributes
            .iter()
            .filter(|attr| matches!(attr.kind, AttributeKind::Name(_)));

        if let Some(name_attr) = name_attrs.next() {
            if let Some(attr) = name_attrs.next() {
                return syn::Error::new(attr.span, "only a single name attribute is allowed")
                    .to_compile_error()
                    .into();
            }

            let AttributeKind::Name(name) = &name_attr.kind else {
                unreachable!()
            };

            name
        } else {
            return syn::Error::new_spanned(ident, "missing a name attribute")
                .to_compile_error()
                .into();
        }
    };

    let edges = attributes
        .iter()
        .filter(|attr| {
            matches!(
                attr.kind,
                AttributeKind::Depends(_) | AttributeKind::Entails(_)
            )
        })
        .flat_map(|attr| match &attr.kind {
            AttributeKind::Name(_) => unreachable!(),
            AttributeKind::Depends(paths) => paths
                .iter()
                .map(|path| quote! { ::initgraph::Edge::new(&#path, &#ident) })
                .collect::<Vec<_>>(),
            AttributeKind::Entails(paths) => paths
                .iter()
                .map(|path| quote! { ::initgraph::Edge::new(&#ident, &#path) })
                .collect::<Vec<_>>(),
        })
        .map(|edge| {
            quote! {
                const _: () = {
                    #[used]
                    #[doc(hidden)]
                    static __EDGE: ::initgraph::Edge = #edge;

                    #[used]
                    #[doc(hidden)]
                    #[unsafe(link_section = ".initgraph.ctors")]
                    static __EDGE_CTOR: fn() = || __EDGE.register();
                };
            }
        });

    quote! {
        #[used]
        #[doc(hidden)]
        #[unsafe(link_section = ".initgraph.nodes")]
        #(#other_attrs)*
        #vis static #ident: ::initgraph::Node = ::initgraph::Node::new(
            #display_name,
            ::initgraph::Action::Callback(|| #block),
        );

        #(#edges)*
    }
    .into()
}
