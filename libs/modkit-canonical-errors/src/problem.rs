use serde::Serialize;

use crate::error::CanonicalError;

// ---------------------------------------------------------------------------
// Problem (RFC 9457)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct Problem {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub context: serde_json::Value,
}

impl From<CanonicalError> for Problem {
    fn from(err: CanonicalError) -> Self {
        let problem_type = err.gts_type().to_string();
        let title = err.title().to_string();
        let status = err.status_code();
        let detail = err.message().to_string();
        let mut context = match &err {
            CanonicalError::Cancelled { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unknown { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::InvalidArgument { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::DeadlineExceeded { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::NotFound { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::AlreadyExists { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::PermissionDenied { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::ResourceExhausted { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::FailedPrecondition { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Aborted { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::OutOfRange { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unimplemented { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Internal { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::ServiceUnavailable { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::DataLoss { ctx, .. } => serde_json::to_value(ctx),
            CanonicalError::Unauthenticated { ctx, .. } => serde_json::to_value(ctx),
        }
        .expect("context serialization should not fail");

        if let Some(rt) = err.resource_type() {
            context["resource_type"] = serde_json::Value::String(rt.to_string());
        }

        Problem {
            problem_type,
            title,
            status,
            detail,
            instance: None,
            trace_id: None,
            context,
        }
    }
}
