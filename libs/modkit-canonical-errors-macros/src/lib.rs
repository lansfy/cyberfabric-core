use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

/// Generates a resource error type with constructors for all 16 canonical error categories.
///
/// For `ResourceInfo` categories (`not_found`, `already_exists`, `data_loss`), the generated
/// constructors take only a resource name and bake the GTS type into `ResourceInfo`.
/// For all other categories, constructors forward the context and tag with `resource_type`.
///
/// # Example
///
/// ```ignore
/// #[resource_error("gts.cf.core.tenants.tenant.v1~")]
/// struct TenantResourceError;
///
/// let err = TenantResourceError::not_found("tenant-123");
/// assert_eq!(err.resource_type(), Some("gts.cf.core.tenants.tenant.v1~"));
/// ```
#[proc_macro_attribute]
pub fn resource_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    let gts_type = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemStruct);

    let gts_value = gts_type.value();
    if let Err(e) = gts_id::validate_gts_id(&gts_value, false) {
        return syn::Error::new_spanned(&gts_type, format!("invalid GTS type: {e}"))
            .to_compile_error()
            .into();
    }

    let vis = &input.vis;
    let name = &input.ident;
    let attrs = &input.attrs;

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name;

        impl #name {
            // --- ResourceInfo categories: take only resource_name ---

            #vis fn not_found(resource_name: impl Into<String>) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::not_found(
                    ::cf_modkit_errors::NotFound::new(#gts_type, resource_name),
                ).with_resource_type(#gts_type)
            }

            #vis fn already_exists(resource_name: impl Into<String>) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::already_exists(
                    ::cf_modkit_errors::AlreadyExists::new(#gts_type, resource_name),
                ).with_resource_type(#gts_type)
            }

            #vis fn data_loss(resource_name: impl Into<String>) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::data_loss(
                    ::cf_modkit_errors::DataLoss::new(#gts_type, resource_name),
                ).with_resource_type(#gts_type)
            }

            // --- All other categories: forward context, tag with resource_type ---

            #vis fn invalid_argument(ctx: ::cf_modkit_errors::InvalidArgument) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::invalid_argument(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn permission_denied(ctx: ::cf_modkit_errors::PermissionDenied) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::permission_denied(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unauthenticated(ctx: ::cf_modkit_errors::Unauthenticated) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::unauthenticated(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn resource_exhausted(ctx: ::cf_modkit_errors::ResourceExhausted) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::resource_exhausted(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn failed_precondition(ctx: ::cf_modkit_errors::FailedPrecondition) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::failed_precondition(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn aborted(ctx: ::cf_modkit_errors::Aborted) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::aborted(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn out_of_range(ctx: ::cf_modkit_errors::OutOfRange) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::out_of_range(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unimplemented(ctx: ::cf_modkit_errors::Unimplemented) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::unimplemented(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn internal(ctx: ::cf_modkit_errors::Internal) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::internal(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unknown(ctx: ::cf_modkit_errors::Unknown) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::unknown(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn deadline_exceeded(ctx: ::cf_modkit_errors::DeadlineExceeded) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::deadline_exceeded(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn cancelled(ctx: ::cf_modkit_errors::Cancelled) -> ::cf_modkit_errors::CanonicalError {
                ::cf_modkit_errors::CanonicalError::cancelled(ctx)
                    .with_resource_type(#gts_type)
            }
        }
    };

    expanded.into()
}
