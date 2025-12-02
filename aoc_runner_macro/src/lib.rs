use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn resolver(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_body = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;

    // Extract the return type's inner type (Result<T>)
    let return_type = match &input_fn.sig.output {
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
        ReturnType::Default => {
            return syn::Error::new_spanned(
                &input_fn.sig,
                "resolver function must return Result<T>",
            )
            .to_compile_error()
            .into();
        }
    };

    let expanded = quote! {
        fn __solve(#fn_inputs) -> #return_type
            #fn_body

        fn main() -> ::aoc_runner::color_eyre::Result<()> {
            ::aoc_runner::run(__solve)
        }
    };

    expanded.into()
}
