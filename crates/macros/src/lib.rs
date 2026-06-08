use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

/// Drop-in replacement for `#[server]` that injects auth + permission check before the body.
///
/// Generates a `{FnNamePascal}Controller` marker struct. Implement `AccessController` on it:
///
/// ```rust
/// #[secured_server]
/// pub async fn my_handler() -> Result<String, ServerFnError> { ... }
///
/// impl AccessController for MyHandlerController {
///     fn check_permission(user: &User) -> Result<(), AppError> { ... }
/// }
/// ```
#[proc_macro_attribute]
pub fn secured_server(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let base_name = fn_name.to_string().to_case(Case::Pascal);
    let controller_ident = format_ident!("{}Controller", base_name);

    let validation_code = quote! {
        let auth_session = crate::domain::auth::auth_context::auth()
            .await
            .map_err(|e| ::dioxus::prelude::ServerFnError::new(e.to_string()))?;
        let user = auth_session
            .current_user
            .ok_or_else(|| ::dioxus::prelude::ServerFnError::new("Not authenticated"))?;
        <#controller_ident as crate::domain::auth::access_controller::AccessController>::check_permission(&user)
            .map_err(|e| ::dioxus::prelude::ServerFnError::new(e.to_string()))?;
    };

    let original_body = input_fn.block.clone();
    *input_fn.block = syn::parse_quote! {
        {
            #validation_code
            #original_body
        }
    };

    let output = quote! {
        pub struct #controller_ident;

        #[::dioxus::prelude::server]
        #input_fn

        const _: () = {
            fn _enforce_access_controller<T: crate::domain::auth::access_controller::AccessController>() {}
            fn _check() { _enforce_access_controller::<#controller_ident>(); }
        };
    };
    output.into()
}
