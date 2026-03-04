use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

/// Generates a resource error type with constructors for all 16 canonical error categories.
///
/// For ResourceInfo categories (not_found, already_exists, data_loss), the generated
/// constructors take only a resource name and bake the GTS type into ResourceInfo.
/// For all other categories, constructors forward the context and tag with resource_type.
///
/// # Example
///
/// ```ignore
/// #[resource_error("gts.cf.core.tenants.tenant.v1")]
/// struct TenantResourceError;
///
/// let err = TenantResourceError::not_found("tenant-123");
/// assert_eq!(err.resource_type(), Some("gts.cf.core.tenants.tenant.v1"));
/// ```
#[proc_macro_attribute]
pub fn resource_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    let gts_type = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemStruct);
    let vis = &input.vis;
    let name = &input.ident;
    let attrs = &input.attrs;

    let expanded = quote! {
        #(#attrs)*
        #vis struct #name;

        impl #name {
            // --- ResourceInfo categories: take only resource_name ---

            #vis fn not_found(resource_name: impl Into<String>) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::not_found(
                    ::modkit_canonical_errors::ResourceInfo::new(#gts_type, resource_name),
                ).with_resource_type(#gts_type)
            }

            #vis fn already_exists(resource_name: impl Into<String>) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::already_exists(
                    ::modkit_canonical_errors::ResourceInfo::new(#gts_type, resource_name)
                        .with_description("Resource already exists"),
                ).with_resource_type(#gts_type)
            }

            #vis fn data_loss(resource_name: impl Into<String>) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::data_loss(
                    ::modkit_canonical_errors::ResourceInfo::new(#gts_type, resource_name)
                        .with_description("Data loss detected"),
                ).with_resource_type(#gts_type)
            }

            // --- All other categories: forward context, tag with resource_type ---

            #vis fn invalid_argument(ctx: ::modkit_canonical_errors::Validation) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::invalid_argument(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn permission_denied(ctx: ::modkit_canonical_errors::ErrorInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::permission_denied(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unauthenticated(ctx: ::modkit_canonical_errors::ErrorInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::unauthenticated(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn resource_exhausted(ctx: ::modkit_canonical_errors::QuotaFailure) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::resource_exhausted(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn failed_precondition(ctx: ::modkit_canonical_errors::PreconditionFailure) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::failed_precondition(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn aborted(ctx: ::modkit_canonical_errors::ErrorInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::aborted(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn out_of_range(ctx: ::modkit_canonical_errors::Validation) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::out_of_range(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unimplemented(ctx: ::modkit_canonical_errors::ErrorInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::unimplemented(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn internal(ctx: ::modkit_canonical_errors::DebugInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::internal(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn unknown(detail: impl Into<String>) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::unknown(detail)
                    .with_resource_type(#gts_type)
            }

            #vis fn deadline_exceeded(ctx: ::modkit_canonical_errors::RequestInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::deadline_exceeded(ctx)
                    .with_resource_type(#gts_type)
            }

            #vis fn cancelled(ctx: ::modkit_canonical_errors::RequestInfo) -> ::modkit_canonical_errors::CanonicalError {
                ::modkit_canonical_errors::CanonicalError::cancelled(ctx)
                    .with_resource_type(#gts_type)
            }
        }
    };

    expanded.into()
}
