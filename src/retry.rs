use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, PathArguments, ReturnType};

/// Takes the return type (a result) and replaces the error type with &str
fn replace_return_type(r: &ReturnType) -> Result<ReturnType, &'static str> {
    if let syn::ReturnType::Type(_, t) = r {
        if let syn::Type::Path(p) = t.as_ref() {
            let p = &p.path;
            let pairs = p.segments.pairs();
            let last = &pairs.last().unwrap();
            let value = &last.value();
            let arguments = &value.arguments;
            if let PathArguments::AngleBracketed(generics) = arguments {
                let ok_type = generics.args.first().unwrap();

                return syn::parse::<ReturnType>(quote! {-> Result<#ok_type, &'static str>}.into())
                    .map_err(|_| "Failed to generate return type");
            }
        }
    }
    Err("Could not replace return type")
}

/// Check if the return type of a function has the same name as `ty`.
/// This function does not check e.g. generics, only the identifier
fn returns_type(r: &ReturnType, ty: &str) -> bool {
    if let syn::ReturnType::Type(_, t) = r {
        if let syn::Type::Path(p) = t.as_ref() {
            let p = &p.path;
            let pairs = p.segments.pairs();
            let last = &pairs.last().unwrap();
            let value = &last.value();
            return &value.ident.to_string() == ty;
        }
    }
    false
}

/// Retry the function this macro is attached to if it fails
#[proc_macro_attribute]
pub fn retry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = syn::parse(item).unwrap();
    let retries: syn::Expr = syn::parse(attr).expect("Failed to parse number of retries");

    // Get the function signature and replace the error type with a &str
    let mut sig = input.sig.clone();
    let rt = replace_return_type(&sig.output).unwrap();
    sig.output = rt;

    // Extract the names of the arguments in order
    let fn_args = sig.inputs.clone().into_pairs().filter_map(|p| {
        if let FnArg::Typed(pt) = p.into_value() {
            Some(pt.pat)
        } else {
            None
        }
    });

    // the original function needs to be renamed
    let mut original_fun = input.clone();
    original_fun.sig.ident = format_ident!("_{}", sig.ident);
    let original_fun_ident = original_fun.sig.ident.clone();

    if returns_type(&original_fun.sig.output, "Result") {
        let tokens = quote! {
            #original_fun

            #sig {
                let mut tries = 0;
                while tries <= #retries {
                    if let Ok(v) = #original_fun_ident(#(#fn_args),*) {
                        return Ok(v);
                    }

                    tries += 1;
                }

                return Err("Exceeded number of tries")
            }
        };
        tokens.into()
    } else if returns_type(&original_fun.sig.output, "Option") {
        let tokens = quote! {
            #original_fun

            #sig {
                let mut tries = 0;
                while tries <= #retries {
                    if let Some(v) = #original_fun_ident(#(#fn_args),*) {
                        return Ok(v);
                    }

                    tries += 1;
                }

                return Err("Exceeded number of tries")
            }
        };
        tokens.into()
    } else {
        panic!("retry is only applicable to functions returning Result or Option")
    }
}
