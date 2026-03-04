use std::fmt;

use gts::schema::GtsSchema;

use crate::context::{
    DebugInfo, DebugInfoV1, ErrorInfo, ErrorInfoV1, PreconditionFailure, PreconditionFailureV1,
    QuotaFailure, QuotaFailureV1, RequestInfo, RequestInfoV1, ResourceInfo, ResourceInfoV1,
    RetryInfo, RetryInfoV1, Validation,
};
use crate::kind::ErrorKind;

// ---------------------------------------------------------------------------
// CanonicalError Enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum CanonicalError {
    Cancelled {
        ctx: RequestInfo,
        message: String,
        resource_type: Option<String>,
    },
    Unknown {
        ctx: DebugInfo,
        message: String,
        resource_type: Option<String>,
    },
    InvalidArgument {
        ctx: Validation,
        message: String,
        resource_type: Option<String>,
    },
    DeadlineExceeded {
        ctx: RequestInfo,
        message: String,
        resource_type: Option<String>,
    },
    NotFound {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
    },
    AlreadyExists {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
    },
    PermissionDenied {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
    },
    ResourceExhausted {
        ctx: QuotaFailure,
        message: String,
        resource_type: Option<String>,
    },
    FailedPrecondition {
        ctx: PreconditionFailure,
        message: String,
        resource_type: Option<String>,
    },
    Aborted {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
    },
    OutOfRange {
        ctx: Validation,
        message: String,
        resource_type: Option<String>,
    },
    Unimplemented {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
    },
    Internal {
        ctx: DebugInfo,
        message: String,
        resource_type: Option<String>,
    },
    ServiceUnavailable {
        ctx: RetryInfo,
        message: String,
        resource_type: Option<String>,
    },
    DataLoss {
        ctx: ResourceInfo,
        message: String,
        resource_type: Option<String>,
    },
    Unauthenticated {
        ctx: ErrorInfo,
        message: String,
        resource_type: Option<String>,
    },
}

impl CanonicalError {
    // --- Ergonomic constructors (one per category) ---

    pub fn cancelled(ctx: RequestInfo) -> Self {
        Self::Cancelled {
            ctx,
            message: String::from("Operation cancelled by the client"),
            resource_type: None,
        }
    }

    pub fn unknown(detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let message = detail.clone();
        Self::Unknown {
            ctx: DebugInfo::new(detail),
            message,
            resource_type: None,
        }
    }

    pub fn invalid_argument(ctx: Validation) -> Self {
        let message = match &ctx {
            Validation::FieldViolations { .. } => String::from("Request validation failed"),
            Validation::Format { format } => format.clone(),
            Validation::Constraint { constraint } => constraint.clone(),
        };
        Self::InvalidArgument {
            ctx,
            message,
            resource_type: None,
        }
    }

    pub fn deadline_exceeded(ctx: RequestInfo) -> Self {
        Self::DeadlineExceeded {
            ctx,
            message: String::from("Operation did not complete within the allowed time"),
            resource_type: None,
        }
    }

    pub fn not_found(ctx: ResourceInfo) -> Self {
        Self::NotFound {
            ctx,
            message: String::from("Resource not found"),
            resource_type: None,
        }
    }

    pub fn already_exists(ctx: ResourceInfo) -> Self {
        let message = ctx.description.clone();
        Self::AlreadyExists {
            ctx,
            message,
            resource_type: None,
        }
    }

    pub fn permission_denied(ctx: ErrorInfo) -> Self {
        Self::PermissionDenied {
            ctx,
            message: String::from("You do not have permission to perform this operation"),
            resource_type: None,
        }
    }

    pub fn resource_exhausted(ctx: QuotaFailure) -> Self {
        Self::ResourceExhausted {
            ctx,
            message: String::from("Quota exceeded"),
            resource_type: None,
        }
    }

    pub fn failed_precondition(ctx: PreconditionFailure) -> Self {
        Self::FailedPrecondition {
            ctx,
            message: String::from("Operation precondition not met"),
            resource_type: None,
        }
    }

    pub fn aborted(ctx: ErrorInfo) -> Self {
        Self::Aborted {
            ctx,
            message: String::from("Operation aborted due to concurrency conflict"),
            resource_type: None,
        }
    }

    pub fn out_of_range(ctx: Validation) -> Self {
        let message = match &ctx {
            Validation::FieldViolations { .. } => String::from("Value out of range"),
            Validation::Format { format } => format.clone(),
            Validation::Constraint { constraint } => constraint.clone(),
        };
        Self::OutOfRange {
            ctx,
            message,
            resource_type: None,
        }
    }

    pub fn unimplemented(ctx: ErrorInfo) -> Self {
        Self::Unimplemented {
            ctx,
            message: String::from("This operation is not implemented"),
            resource_type: None,
        }
    }

    pub fn internal(ctx: DebugInfo) -> Self {
        Self::Internal {
            ctx,
            message: String::from("An internal error occurred. Please retry later."),
            resource_type: None,
        }
    }

    pub fn service_unavailable(ctx: RetryInfo) -> Self {
        Self::ServiceUnavailable {
            ctx,
            message: String::from("Service temporarily unavailable"),
            resource_type: None,
        }
    }

    pub fn data_loss(ctx: ResourceInfo) -> Self {
        let message = ctx.description.clone();
        Self::DataLoss {
            ctx,
            message,
            resource_type: None,
        }
    }

    pub fn unauthenticated(ctx: ErrorInfo) -> Self {
        Self::Unauthenticated {
            ctx,
            message: String::from("Authentication required"),
            resource_type: None,
        }
    }

    // --- Builder methods ---

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        let msg = msg.into();
        match &mut self {
            Self::Cancelled { message, .. }
            | Self::Unknown { message, .. }
            | Self::InvalidArgument { message, .. }
            | Self::DeadlineExceeded { message, .. }
            | Self::NotFound { message, .. }
            | Self::AlreadyExists { message, .. }
            | Self::PermissionDenied { message, .. }
            | Self::ResourceExhausted { message, .. }
            | Self::FailedPrecondition { message, .. }
            | Self::Aborted { message, .. }
            | Self::OutOfRange { message, .. }
            | Self::Unimplemented { message, .. }
            | Self::Internal { message, .. }
            | Self::ServiceUnavailable { message, .. }
            | Self::DataLoss { message, .. }
            | Self::Unauthenticated { message, .. } => *message = msg,
        }
        self
    }

    pub fn with_resource_type(mut self, rt: impl Into<String>) -> Self {
        let rt = Some(rt.into());
        match &mut self {
            Self::Cancelled { resource_type, .. }
            | Self::Unknown { resource_type, .. }
            | Self::InvalidArgument { resource_type, .. }
            | Self::DeadlineExceeded { resource_type, .. }
            | Self::NotFound { resource_type, .. }
            | Self::AlreadyExists { resource_type, .. }
            | Self::PermissionDenied { resource_type, .. }
            | Self::ResourceExhausted { resource_type, .. }
            | Self::FailedPrecondition { resource_type, .. }
            | Self::Aborted { resource_type, .. }
            | Self::OutOfRange { resource_type, .. }
            | Self::Unimplemented { resource_type, .. }
            | Self::Internal { resource_type, .. }
            | Self::ServiceUnavailable { resource_type, .. }
            | Self::DataLoss { resource_type, .. }
            | Self::Unauthenticated { resource_type, .. } => *resource_type = rt,
        }
        self
    }

    // --- Accessors ---

    pub fn message(&self) -> &str {
        match self {
            Self::Cancelled { message, .. }
            | Self::Unknown { message, .. }
            | Self::InvalidArgument { message, .. }
            | Self::DeadlineExceeded { message, .. }
            | Self::NotFound { message, .. }
            | Self::AlreadyExists { message, .. }
            | Self::PermissionDenied { message, .. }
            | Self::ResourceExhausted { message, .. }
            | Self::FailedPrecondition { message, .. }
            | Self::Aborted { message, .. }
            | Self::OutOfRange { message, .. }
            | Self::Unimplemented { message, .. }
            | Self::Internal { message, .. }
            | Self::ServiceUnavailable { message, .. }
            | Self::DataLoss { message, .. }
            | Self::Unauthenticated { message, .. } => message,
        }
    }

    pub fn resource_type(&self) -> Option<&str> {
        match self {
            Self::Cancelled { resource_type, .. }
            | Self::Unknown { resource_type, .. }
            | Self::InvalidArgument { resource_type, .. }
            | Self::DeadlineExceeded { resource_type, .. }
            | Self::NotFound { resource_type, .. }
            | Self::AlreadyExists { resource_type, .. }
            | Self::PermissionDenied { resource_type, .. }
            | Self::ResourceExhausted { resource_type, .. }
            | Self::FailedPrecondition { resource_type, .. }
            | Self::Aborted { resource_type, .. }
            | Self::OutOfRange { resource_type, .. }
            | Self::Unimplemented { resource_type, .. }
            | Self::Internal { resource_type, .. }
            | Self::ServiceUnavailable { resource_type, .. }
            | Self::DataLoss { resource_type, .. }
            | Self::Unauthenticated { resource_type, .. } => resource_type.as_deref(),
        }
    }

    // --- ErrorKind & delegated metadata ---

    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Cancelled { .. } => ErrorKind::Cancelled,
            Self::Unknown { .. } => ErrorKind::Unknown,
            Self::InvalidArgument { .. } => ErrorKind::InvalidArgument,
            Self::DeadlineExceeded { .. } => ErrorKind::DeadlineExceeded,
            Self::NotFound { .. } => ErrorKind::NotFound,
            Self::AlreadyExists { .. } => ErrorKind::AlreadyExists,
            Self::PermissionDenied { .. } => ErrorKind::PermissionDenied,
            Self::ResourceExhausted { .. } => ErrorKind::ResourceExhausted,
            Self::FailedPrecondition { .. } => ErrorKind::FailedPrecondition,
            Self::Aborted { .. } => ErrorKind::Aborted,
            Self::OutOfRange { .. } => ErrorKind::OutOfRange,
            Self::Unimplemented { .. } => ErrorKind::Unimplemented,
            Self::Internal { .. } => ErrorKind::Internal,
            Self::ServiceUnavailable { .. } => ErrorKind::ServiceUnavailable,
            Self::DataLoss { .. } => ErrorKind::DataLoss,
            Self::Unauthenticated { .. } => ErrorKind::Unauthenticated,
        }
    }

    pub fn gts_type(&self) -> &'static str {
        self.kind().gts_type()
    }

    pub fn status_code(&self) -> u16 {
        self.kind().status_code()
    }

    pub fn title(&self) -> &'static str {
        self.kind().title()
    }

    fn category_name(&self) -> &'static str {
        self.kind().category_name()
    }
}

impl fmt::Display for CanonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.category_name(), self.message())
    }
}

impl std::error::Error for CanonicalError {}

impl GtsSchema for CanonicalError {
    const SCHEMA_ID: &'static str = "gts.cf.core.errors.canonical_error.v1~";

    fn gts_schema_with_refs() -> serde_json::Value {
        let variant = |name: &str, ctx_ref: &str| {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "category": { "const": name },
                    "message": { "type": "string" },
                    "resource_type": { "type": "string" },
                    "context": { "$ref": ctx_ref }
                },
                "required": ["category", "message", "context"]
            })
        };

        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.canonical_error.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                variant("cancelled",          &format!("gts://{}", RequestInfoV1::SCHEMA_ID)),
                variant("unknown",            &format!("gts://{}", DebugInfoV1::SCHEMA_ID)),
                variant("invalid_argument",   &format!("gts://{}", Validation::SCHEMA_ID)),
                variant("deadline_exceeded",   &format!("gts://{}", RequestInfoV1::SCHEMA_ID)),
                variant("not_found",          &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("already_exists",     &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("permission_denied",  &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("resource_exhausted", &format!("gts://{}", QuotaFailureV1::SCHEMA_ID)),
                variant("failed_precondition", &format!("gts://{}", PreconditionFailureV1::SCHEMA_ID)),
                variant("aborted",            &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("out_of_range",       &format!("gts://{}", Validation::SCHEMA_ID)),
                variant("unimplemented",      &format!("gts://{}", ErrorInfoV1::SCHEMA_ID)),
                variant("internal",           &format!("gts://{}", DebugInfoV1::SCHEMA_ID)),
                variant("unavailable",        &format!("gts://{}", RetryInfoV1::SCHEMA_ID)),
                variant("data_loss",          &format!("gts://{}", ResourceInfoV1::SCHEMA_ID)),
                variant("unauthenticated",    &format!("gts://{}", ErrorInfoV1::SCHEMA_ID))
            ]
        })
    }
}
