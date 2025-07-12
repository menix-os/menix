use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn syscall(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let vis = item.vis;
    let sig = item.sig;
    let block = item.block;

    let ident = sig.ident;
    let inputs = sig.inputs.iter();
    let output = sig.output;

    let impl_call_args = inputs
        .clone()
        .enumerate()
        .map(|(i, _)| format_ident!("a{i}"))
        .map(|ident| quote! { #ident.into() });

    let result = quote! {
        #vis fn #ident(
            a0: usize,
            a1: usize,
            a2: usize,
            a3: usize,
            a4: usize,
            a5: usize,
        ) -> EResult<usize> {
            fn syscall_impl(#(#inputs),*) #output { #block }

            syscall_impl(#(#impl_call_args),*).map(|x| x.into())
        }
    };

    result.into()
}
