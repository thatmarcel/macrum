use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::{parse, parse_macro_input, ItemFn, Lit};

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    apply_route_transformation(attr, item, "get")
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    apply_route_transformation(attr, item, "post")
}

fn apply_route_transformation(attr: TokenStream, item: TokenStream, method: &str) -> TokenStream {
    let Ok(Lit::Str(path_attr)) = parse::<Lit>(attr)
    else {
        return report_error("Missing path literal attribute");
    };
    let path_attr_string = path_attr.value();

    if !path_attr_string.starts_with("/") {
        return report_error("Path must start with a slash");
    }

    if path_attr_string.contains("/:") || path_attr_string.contains("/*") {
        return report_error("Path parameters must be in the form of /{name}, not /:name or /*name");
    }

    let mut parsed_item = parse_macro_input!(item as ItemFn);

    if parsed_item.sig.asyncness.is_none() {
        return report_error("Only async functions are supported");
    }

    if !parsed_item.sig.generics.params.is_empty() {
        return report_error("Generics are not supported as of now");
    }

    let parsed_item_original_ident = parsed_item.sig.ident.clone();

    let parsed_item_renamed_ident = Ident::new(
        &format!("_{}_route", parsed_item_original_ident),
        parsed_item.sig.ident.span()
    );
    parsed_item.sig.ident = parsed_item_renamed_ident.clone();

    let method_ident = Ident::new(method, Span::call_site());

    quote! {
        #parsed_item

        fn #parsed_item_original_ident() -> (String, axum::routing::MethodRouter<(), std::convert::Infallible>) {
            (#path_attr.to_string(), axum::routing::#method_ident(#parsed_item_renamed_ident))
        }
    }.into()
}

fn report_error(error_message: &str) -> TokenStream {
    let error_message = Literal::string(error_message);
    quote! {
        compile_error!(#error_message)
    }.into()
}