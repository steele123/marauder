use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, token::Token, Abi, Attribute, AttributeArgs, FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token,
    Type,
};

#[proc_macro_attribute]
pub fn dll_main(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);

    if input.sig.ident != "main" {
        return syn::Error::new_spanned(&input.sig.ident, "dll_main must be named main")
            .to_compile_error()
            .into();
    }

    if !input.sig.inputs.is_empty() {
        return syn::Error::new_spanned(&input.sig.inputs, "dll_main can't accept any arguments")
            .to_compile_error()
            .into();
    }

    let is_async = input.sig.asyncness.take().is_some();

    create_main(input, args, is_async).unwrap_or_else(|e| e.to_compile_error().into())
}

fn create_main(mut input: ItemFn, args: AttributeArgs, is_async: bool) -> Result<TokenStream, syn::Error> {
    let original_body = &input.block;
    let brace_token = input.block.brace_token;
    if !is_async {
        input.block = syn::parse2(quote! {
            {
                match dw_reason {
                    1u32 => {
                        std::thread::spawn(move || {
                            #original_body
                        });
                    },
                    _ => {
                        return false
                    },
                };

                true
            }
        })
        .expect("Couldn't parse");
    } else {
        input.block = syn::parse2(quote! {
            {
                match dw_reason {
                    1u32 => {
                        let mut rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async move {
                            #original_body
                        });
                    },
                    _ => {
                        return false
                    },
                };

                true
            }
        })
        .expect("Couldn't parse");
    }
    input.block.brace_token = brace_token;

    // Only reason I decided to use quote! to parse this is because working with
    // input.sig.input is way more confusing than it should be
    // TODO: We probably want to make the type of the params of this function from
    //  our mem library's types
    input.sig =
        syn::parse2(quote! {extern "system" fn DllMain(module_handle: isize, dw_reason: std::os::raw::c_ulong, lp_reserved: *mut std::ffi::c_void) -> bool})
            .unwrap();

    // If we really cared I think we could just append a Attribute to input.attr for
    // no_mangle
    let result = quote! {
        #[no_mangle]
        #input
    };

    Ok(result.into())
}
