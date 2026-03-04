extern crate cf_modkit_canonical_errors as modkit_canonical_errors;

use modkit_canonical_errors::{
    CanonicalError, DebugInfo, DebugInfoV1, ErrorInfo, ErrorInfoV1, FieldViolationV1,
    PreconditionFailure, PreconditionFailureV1, PreconditionViolationV1, Problem, QuotaFailure,
    QuotaFailureV1, QuotaViolationV1, RequestInfo, RequestInfoV1, ResourceInfo, ResourceInfoV1,
    RetryInfo, RetryInfoV1, Validation,
};

#[test]
fn not_found_gts_type() {
    let err =
        CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
    assert_eq!(
        err.gts_type(),
        "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~"
    );
}

#[test]
fn not_found_status_code() {
    let err =
        CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
    assert_eq!(err.status_code(), 404);
}

#[test]
fn not_found_title() {
    let err =
        CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"));
    assert_eq!(err.title(), "Not Found");
}

#[test]
fn display_includes_category_and_message() {
    let err =
        CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
            .with_message("User not found");
    assert_eq!(format!("{err}"), "not_found: User not found");
}

#[test]
fn with_message_overrides_default() {
    let err =
        CanonicalError::not_found(ResourceInfo::new("gts.cf.core.users.user.v1", "user-123"))
            .with_message("custom detail");
    assert_eq!(err.message(), "custom detail");
}

#[test]
fn all_16_categories_convert_to_problem() {
    let errors: Vec<CanonicalError> = vec![
        CanonicalError::cancelled(RequestInfo::new("req-1")),
        CanonicalError::unknown("unknown error"),
        CanonicalError::invalid_argument(Validation::format("bad")),
        CanonicalError::deadline_exceeded(RequestInfo::new("req-2")),
        CanonicalError::not_found(ResourceInfo::new("t", "n")),
        CanonicalError::already_exists(ResourceInfo::new("t", "n")),
        CanonicalError::permission_denied(ErrorInfo::new("R", "D")),
        CanonicalError::resource_exhausted(QuotaFailure::new(vec![])),
        CanonicalError::failed_precondition(PreconditionFailure::new(vec![])),
        CanonicalError::aborted(ErrorInfo::new("R", "D")),
        CanonicalError::out_of_range(Validation::constraint("x")),
        CanonicalError::unimplemented(ErrorInfo::new("R", "D")),
        CanonicalError::internal(DebugInfo::new("bug")),
        CanonicalError::service_unavailable(RetryInfo::after_seconds(10)),
        CanonicalError::data_loss(ResourceInfo::new("t", "n")),
        CanonicalError::unauthenticated(ErrorInfo::new("R", "D")),
    ];
    assert_eq!(errors.len(), 16);
    for err in errors {
        let problem = Problem::from(err);
        assert!(!problem.problem_type.is_empty());
        assert!(!problem.title.is_empty());
        assert!(problem.status > 0);
    }
}

// =========================================================================
// GTS Schema test — CanonicalError
// =========================================================================

#[test]
fn schema_canonical_error() {
    use gts::schema::GtsSchema;
    let schema = CanonicalError::gts_schema_with_refs();

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

    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.canonical_error.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                variant("cancelled",           "gts://gts.cf.core.errors.request_info.v1~"),
                variant("unknown",             "gts://gts.cf.core.errors.debug_info.v1~"),
                variant("invalid_argument",    "gts://gts.cf.core.errors.validation.v1~"),
                variant("deadline_exceeded",   "gts://gts.cf.core.errors.request_info.v1~"),
                variant("not_found",           "gts://gts.cf.core.errors.resource_info.v1~"),
                variant("already_exists",      "gts://gts.cf.core.errors.resource_info.v1~"),
                variant("permission_denied",   "gts://gts.cf.core.errors.error_info.v1~"),
                variant("resource_exhausted",  "gts://gts.cf.core.errors.quota_failure.v1~"),
                variant("failed_precondition", "gts://gts.cf.core.errors.precondition_failure.v1~"),
                variant("aborted",             "gts://gts.cf.core.errors.error_info.v1~"),
                variant("out_of_range",        "gts://gts.cf.core.errors.validation.v1~"),
                variant("unimplemented",       "gts://gts.cf.core.errors.error_info.v1~"),
                variant("internal",            "gts://gts.cf.core.errors.debug_info.v1~"),
                variant("unavailable",         "gts://gts.cf.core.errors.retry_info.v1~"),
                variant("data_loss",           "gts://gts.cf.core.errors.resource_info.v1~"),
                variant("unauthenticated",     "gts://gts.cf.core.errors.error_info.v1~")
            ]
        })
    );
}

// =========================================================================
// GTS ID validation — ensures all IDs in the crate are valid GTS identifiers
// =========================================================================

#[test]
fn validate_all_gts_ids() {
    use gts::schema::GtsSchema;

    // Validate all 16 category GTS type IDs
    let errors = vec![
        CanonicalError::cancelled(RequestInfo::new("r")),
        CanonicalError::unknown("e"),
        CanonicalError::invalid_argument(Validation::format("f")),
        CanonicalError::deadline_exceeded(RequestInfo::new("r")),
        CanonicalError::not_found(ResourceInfo::new("t", "n")),
        CanonicalError::already_exists(ResourceInfo::new("t", "n")),
        CanonicalError::permission_denied(ErrorInfo::new("R", "D")),
        CanonicalError::resource_exhausted(QuotaFailure::new(vec![])),
        CanonicalError::failed_precondition(PreconditionFailure::new(vec![])),
        CanonicalError::aborted(ErrorInfo::new("R", "D")),
        CanonicalError::out_of_range(Validation::constraint("c")),
        CanonicalError::unimplemented(ErrorInfo::new("R", "D")),
        CanonicalError::internal(DebugInfo::new("d")),
        CanonicalError::service_unavailable(RetryInfo::after_seconds(1)),
        CanonicalError::data_loss(ResourceInfo::new("t", "n")),
        CanonicalError::unauthenticated(ErrorInfo::new("R", "D")),
    ];
    for err in &errors {
        let id = err.gts_type();
        assert!(id.ends_with('~'), "GTS type ID must end with ~: {id}");
        gts_id::validate_gts_id(id, false)
            .unwrap_or_else(|e| panic!("Invalid GTS type ID '{id}': {e}"));
    }

    // Validate all 11 context type schema IDs
    let schema_ids = [
        RetryInfoV1::SCHEMA_ID,
        RequestInfoV1::SCHEMA_ID,
        ResourceInfoV1::SCHEMA_ID,
        ErrorInfoV1::SCHEMA_ID,
        FieldViolationV1::SCHEMA_ID,
        DebugInfoV1::SCHEMA_ID,
        QuotaViolationV1::SCHEMA_ID,
        QuotaFailureV1::SCHEMA_ID,
        PreconditionViolationV1::SCHEMA_ID,
        PreconditionFailureV1::SCHEMA_ID,
        Validation::SCHEMA_ID,
    ];
    for id in &schema_ids {
        assert!(id.ends_with('~'), "Schema ID must end with ~: {id}");
        gts_id::validate_gts_id(id, false)
            .unwrap_or_else(|e| panic!("Invalid schema ID '{id}': {e}"));
    }
}
