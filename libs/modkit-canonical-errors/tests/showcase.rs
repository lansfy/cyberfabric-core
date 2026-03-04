extern crate cf_modkit_canonical_errors as modkit_canonical_errors;

use modkit_canonical_errors::{
    resource_error, CanonicalError, DebugInfo, ErrorInfo, FieldViolation, PreconditionFailure,
    PreconditionViolation, Problem, QuotaFailure, QuotaViolation, RequestInfo, RetryInfo,
    Validation,
};

// =========================================================================
// Showcase tests — resource-scoped categories (macro-generated)
// =========================================================================

#[test]
fn showcase_not_found() {
    #[resource_error("gts.cf.core.users.user.v1")]
    struct UserResourceError;

    let err = UserResourceError::not_found("user-123");
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",
            "title": "Not Found",
            "status": 404,
            "detail": "Resource not found",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "resource_name": "user-123",
                "description": "Resource not found"
            }
        })
    );
}

#[test]
fn showcase_already_exists() {
    #[resource_error("gts.cf.core.users.user.v1")]
    struct UserResourceError;

    let err = UserResourceError::already_exists("alice@example.com");
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.already_exists.v1~",
            "title": "Already Exists",
            "status": 409,
            "detail": "Resource already exists",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "resource_name": "alice@example.com",
                "description": "Resource already exists"
            }
        })
    );
}

#[test]
fn showcase_data_loss() {
    #[resource_error("gts.cf.core.files.file.v1")]
    struct FileResourceError;

    let err = FileResourceError::data_loss("01JFILE-ABC");
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.data_loss.v1~",
            "title": "Data Loss",
            "status": 500,
            "detail": "Data loss detected",
            "context": {
                "resource_type": "gts.cf.core.files.file.v1",
                "resource_name": "01JFILE-ABC",
                "description": "Data loss detected"
            }
        })
    );
}

#[test]
fn showcase_invalid_argument() {
    #[resource_error("gts.cf.core.users.user.v1")]
    struct UserResourceError;

    // --- Simulated user input ---
    let email = "not-an-email";
    let age: u8 = 12;

    // --- Anticipated user code: validate fields, collect violations ---
    let mut violations = Vec::new();

    if !email.contains('@') {
        violations.push(FieldViolation::new(
            "email",
            "must be a valid email address",
            "INVALID_FORMAT",
        ));
    }
    if age < 18 {
        violations.push(FieldViolation::new(
            "age",
            "must be at least 18",
            "OUT_OF_RANGE",
        ));
    }

    assert!(!violations.is_empty());
    let err = UserResourceError::invalid_argument(Validation::fields(violations));

    // --- Wire format ---
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.invalid_argument.v1~",
            "title": "Invalid Argument",
            "status": 400,
            "detail": "Request validation failed",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "field_violations": [
                    {
                        "field": "email",
                        "description": "must be a valid email address",
                        "reason": "INVALID_FORMAT"
                    },
                    {
                        "field": "age",
                        "description": "must be at least 18",
                        "reason": "OUT_OF_RANGE"
                    }
                ]
            }
        })
    );
}

#[test]
fn showcase_out_of_range() {
    #[resource_error("gts.cf.core.users.user.v1")]
    struct UserResourceError;

    let err = UserResourceError::out_of_range(Validation::constraint(
        "Page 50 is beyond the last page (12)",
    ));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.out_of_range.v1~",
            "title": "Out of Range",
            "status": 400,
            "detail": "Page 50 is beyond the last page (12)",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "constraint": "Page 50 is beyond the last page (12)"
            }
        })
    );
}

#[test]
fn showcase_permission_denied() {
    #[resource_error("gts.cf.core.tenants.tenant.v1")]
    struct TenantResourceError;

    let err = TenantResourceError::permission_denied(ErrorInfo::new(
        "CROSS_TENANT_ACCESS",
        "auth.cyberfabric.io",
    ));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.permission_denied.v1~",
            "title": "Permission Denied",
            "status": 403,
            "detail": "You do not have permission to perform this operation",
            "context": {
                "resource_type": "gts.cf.core.tenants.tenant.v1",
                "reason": "CROSS_TENANT_ACCESS",
                "domain": "auth.cyberfabric.io",
                "metadata": {}
            }
        })
    );
}

#[test]
fn showcase_aborted() {
    #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
    struct UpstreamResourceError;

    let err = UpstreamResourceError::aborted(
        ErrorInfo::new("OPTIMISTIC_LOCK_FAILURE", "cf.oagw")
            .with_metadata("expected_version", "3")
            .with_metadata("actual_version", "5"),
    );
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.aborted.v1~",
            "title": "Aborted",
            "status": 409,
            "detail": "Operation aborted due to concurrency conflict",
            "context": {
                "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                "reason": "OPTIMISTIC_LOCK_FAILURE",
                "domain": "cf.oagw",
                "metadata": {
                    "expected_version": "3",
                    "actual_version": "5"
                }
            }
        })
    );
}

#[test]
fn showcase_unimplemented() {
    #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
    struct UpstreamResourceError;

    let err = UpstreamResourceError::unimplemented(ErrorInfo::new("GRPC_ROUTING", "cf.oagw"));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.unimplemented.v1~",
            "title": "Unimplemented",
            "status": 501,
            "detail": "This operation is not implemented",
            "context": {
                "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                "reason": "GRPC_ROUTING",
                "domain": "cf.oagw",
                "metadata": {}
            }
        })
    );
}

#[test]
fn showcase_failed_precondition() {
    #[resource_error("gts.cf.core.tenants.tenant.v1")]
    struct TenantResourceError;

    let err = TenantResourceError::failed_precondition(PreconditionFailure::new(vec![
        PreconditionViolation::new(
            "STATE",
            "tenant.users",
            "Tenant must have zero active users before deletion",
        ),
    ]));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.failed_precondition.v1~",
            "title": "Failed Precondition",
            "status": 400,
            "detail": "Operation precondition not met",
            "context": {
                "resource_type": "gts.cf.core.tenants.tenant.v1",
                "violations": [
                    {
                        "type": "STATE",
                        "subject": "tenant.users",
                        "description": "Tenant must have zero active users before deletion"
                    }
                ]
            }
        })
    );
}

#[test]
fn showcase_internal() {
    #[resource_error("gts.cf.core.tenants.tenant.v1")]
    struct TenantResourceError;

    let err = TenantResourceError::internal(DebugInfo::new(
        "An internal error occurred. Please retry later.",
    ));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",
            "title": "Internal",
            "status": 500,
            "detail": "An internal error occurred. Please retry later.",
            "context": {
                "resource_type": "gts.cf.core.tenants.tenant.v1",
                "detail": "An internal error occurred. Please retry later.",
                "stack_entries": []
            }
        })
    );
}

#[test]
fn showcase_deadline_exceeded() {
    #[resource_error("gts.cf.core.users.user.v1")]
    struct UserResourceError;

    let err = UserResourceError::deadline_exceeded(RequestInfo::new("01JREQ-ABC"));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.deadline_exceeded.v1~",
            "title": "Deadline Exceeded",
            "status": 504,
            "detail": "Operation did not complete within the allowed time",
            "context": {
                "resource_type": "gts.cf.core.users.user.v1",
                "request_id": "01JREQ-ABC"
            }
        })
    );
}

#[test]
fn showcase_cancelled() {
    #[resource_error("gts.cf.oagw.upstreams.upstream.v1")]
    struct UpstreamResourceError;

    let err = UpstreamResourceError::cancelled(RequestInfo::new("01JREQ-DEF"));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.cancelled.v1~",
            "title": "Cancelled",
            "status": 499,
            "detail": "Operation cancelled by the client",
            "context": {
                "resource_type": "gts.cf.oagw.upstreams.upstream.v1",
                "request_id": "01JREQ-DEF"
            }
        })
    );
}

// =========================================================================
// Showcase tests — system-level categories (direct constructors)
// =========================================================================

#[test]
fn showcase_unauthenticated() {
    let err = CanonicalError::unauthenticated(
        ErrorInfo::new("TOKEN_EXPIRED", "auth.cyberfabric.io")
            .with_metadata("expires_at", "2026-02-25T10:00:00Z"),
    );
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.unauthenticated.v1~",
            "title": "Unauthenticated",
            "status": 401,
            "detail": "Authentication required",
            "context": {
                "reason": "TOKEN_EXPIRED",
                "domain": "auth.cyberfabric.io",
                "metadata": {
                    "expires_at": "2026-02-25T10:00:00Z"
                }
            }
        })
    );
}

#[test]
fn showcase_resource_exhausted() {
    let err = CanonicalError::resource_exhausted(QuotaFailure::new(vec![QuotaViolation::new(
        "requests_per_minute",
        "Limit of 100 requests per minute exceeded",
    )]));
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.resource_exhausted.v1~",
            "title": "Resource Exhausted",
            "status": 429,
            "detail": "Quota exceeded",
            "context": {
                "violations": [
                    {
                        "subject": "requests_per_minute",
                        "description": "Limit of 100 requests per minute exceeded"
                    }
                ]
            }
        })
    );
}

#[test]
fn showcase_unavailable() {
    let err = CanonicalError::service_unavailable(RetryInfo::after_seconds(30));

    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.service_unavailable.v1~",
            "title": "Unavailable",
            "status": 503,
            "detail": "Service temporarily unavailable",
            "context": {
                "retry_after_seconds": 30
            }
        })
    );
}

#[test]
fn showcase_unknown() {
    let err = CanonicalError::unknown("Unexpected response from payment provider");
    let problem = Problem::from(err);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "type": "gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",
            "title": "Unknown",
            "status": 500,
            "detail": "Unexpected response from payment provider",
            "context": {
                "detail": "Unexpected response from payment provider",
                "stack_entries": []
            }
        })
    );
}
