extern crate self as modkit_canonical_errors;

pub mod context;
pub mod error;
pub mod kind;
pub mod problem;

pub use modkit_canonical_errors_macros::resource_error;

pub use context::{
    DebugInfo, DebugInfoV1, ErrorInfo, ErrorInfoV1, FieldViolation, FieldViolationV1,
    PreconditionFailure, PreconditionFailureV1, PreconditionViolation, PreconditionViolationV1,
    QuotaFailure, QuotaFailureV1, QuotaViolation, QuotaViolationV1, RequestInfo, RequestInfoV1,
    ResourceInfo, ResourceInfoV1, RetryInfo, RetryInfoV1, Validation,
};
pub use error::CanonicalError;
pub use kind::ErrorKind;
pub use problem::Problem;

