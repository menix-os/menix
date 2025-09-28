use convert_case::{Case, Casing};
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
struct PciSubclassVariant {
    name: Ident,
    code: u8,
}

impl Parse for PciSubclassVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let code_lit: syn::LitInt = input.parse()?;
        let code = code_lit.base10_parse::<u8>()?;

        Ok(Self { name, code })
    }
}

#[derive(Debug)]
struct PciCollection<T: Parse> {
    name: Ident,
    code: u8,
    variants: Vec<T>,
}

impl<T: Parse> Parse for PciCollection<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let code_lit: syn::LitInt = input.parse()?;
        let code = code_lit.base10_parse::<u8>()?;

        let content;
        syn::braced!(content in input);

        let variants: Vec<_> = content
            .parse_terminated(T::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            name,
            code,
            variants,
        })
    }
}

struct PciVariants {
    classes: Vec<PciCollection<PciCollection<PciSubclassVariant>>>,
}

impl Parse for PciVariants {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let classes: Vec<_> = input
            .parse_terminated(PciCollection::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self { classes })
    }
}

#[proc_macro]
pub fn pci_variant_builders(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let PciVariants { classes } = syn::parse_macro_input!(input);

    let class_builders = classes.iter().map(|class| {
        let class_name = &class.name;
        let builder_name = Ident::new(&format!("Pci{}Builder", class_name), class_name.span());

        let subclass_builders = class.variants.iter().map(|subclass| {
            let subclass_name = &subclass.name;
            let builder_name = Ident::new(
                &format!("Pci{}{}Builder", class_name, subclass_name),
                subclass_name.span(),
            );

            let prog_if_fns = subclass.variants.iter().map(|prog_if| {
                let name = &prog_if.name;
                let code = prog_if.code;
                let fn_name = Ident::new(&name.to_string().to_case(Case::Snake), name.span());

                quote! {
                    pub const fn #fn_name(mut self) -> crate::system::pci::driver::PciVariant {
                        self.variant.prog_if = Some(#code);
                        self.variant
                    }
                }
            });

            quote! {
                pub struct #builder_name {
                    variant: crate::system::pci::driver::PciVariant,
                }

                impl #builder_name {
                    #(#prog_if_fns)*
                }
            }
        });

        let subclass_fns = class.variants.iter().map(|subclass| {
            let name = &subclass.name;
            let code = subclass.code;
            let fn_name = Ident::new(&name.to_string().to_case(Case::Snake), name.span());
            let subclass_name =
                Ident::new(&format!("Pci{}{}Builder", class_name, name), name.span());

            quote! {
                pub const fn #fn_name(mut self) -> #subclass_name {
                    self.variant.sub_class = Some(#code);
                    #subclass_name { variant: self.variant }
                }
            }
        });

        quote! {
            #(#subclass_builders)*

            pub struct #builder_name {
                variant: crate::system::pci::driver::PciVariant,
            }

            impl #builder_name {
                #(#subclass_fns)*
            }
        }
    });

    let builder_class_fns = classes.iter().map(|class| {
        let name = &class.name;
        let code = class.code;
        let fn_name = Ident::new(&name.to_string().to_case(Case::Snake), name.span());
        let class_name = Ident::new(&format!("Pci{}Builder", name), name.span());

        quote! {
            pub const fn #fn_name(mut self) -> #class_name {
                self.variant.class = Some(#code);
                #class_name { variant: self.variant }
            }
        }
    });

    let builder = quote! {
        pub struct PciVariantBuilder {
            variant: crate::system::pci::driver::PciVariant,
        }

        #(#class_builders)*

        impl PciVariantBuilder {
            pub const fn new() -> Self {
                Self { variant: crate::system::pci::driver::PciVariant::new() }
            }

            pub const fn id(mut self, vendor: u16, device: u16) -> Self {
                self.variant.vendor = Some(vendor);
                self.variant.device = Some(device);
                self
            }

            #(#builder_class_fns)*
        }
    };

    builder.into()
}
