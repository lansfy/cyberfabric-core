use std::collections::HashMap;

use gts::schema::GtsSchema;
use gts_macros::struct_to_gts_schema;
use serde::Serialize;

// ---------------------------------------------------------------------------
// Context Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.field_violation.v1~",
    description = "A single field validation violation",
    properties = "field,description,reason"
)]
pub struct FieldViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub field: String,
    pub description: String,
    pub reason: String,
}

pub type FieldViolation = FieldViolationV1;

impl FieldViolationV1 {
    pub fn new(
        field: impl Into<String>,
        description: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            field: field.into(),
            description: description.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Validation {
    FieldViolations {
        field_violations: Vec<FieldViolation>,
    },
    Format {
        format: String,
    },
    Constraint {
        constraint: String,
    },
}

impl GtsSchema for Validation {
    const SCHEMA_ID: &'static str = "gts.cf.core.errors.validation.v1~";

    fn gts_schema_with_refs() -> serde_json::Value {
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.validation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "field_violations": {
                            "type": "array",
                            "items": { "$ref": "gts://gts.cf.core.errors.field_violation.v1~" }
                        }
                    },
                    "required": ["field_violations"]
                },
                {
                    "type": "object",
                    "properties": {
                        "format": { "type": "string" }
                    },
                    "required": ["format"]
                },
                {
                    "type": "object",
                    "properties": {
                        "constraint": { "type": "string" }
                    },
                    "required": ["constraint"]
                }
            ]
        })
    }
}

impl Validation {
    pub fn fields(violations: impl Into<Vec<FieldViolation>>) -> Self {
        Self::FieldViolations {
            field_violations: violations.into(),
        }
    }

    pub fn format(msg: impl Into<String>) -> Self {
        Self::Format { format: msg.into() }
    }

    pub fn constraint(msg: impl Into<String>) -> Self {
        Self::Constraint {
            constraint: msg.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.resource_info.v1~",
    description = "Resource identification context for resource-scoped errors",
    properties = "resource_type,resource_name,description"
)]
pub struct ResourceInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub resource_type: String,
    pub resource_name: String,
    pub description: String,
}

pub type ResourceInfo = ResourceInfoV1;

impl ResourceInfoV1 {
    pub fn new(resource_type: impl Into<String>, resource_name: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            resource_type: resource_type.into(),
            resource_name: resource_name.into(),
            description: String::from("Resource not found"),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.error_info.v1~",
    description = "Error information with reason, domain, and metadata",
    properties = "reason,domain,metadata"
)]
pub struct ErrorInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub reason: String,
    pub domain: String,
    pub metadata: HashMap<String, String>,
}

pub type ErrorInfo = ErrorInfoV1;

impl ErrorInfoV1 {
    pub fn new(reason: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            reason: reason.into(),
            domain: domain.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.quota_violation.v1~",
    description = "A single quota violation entry",
    properties = "subject,description"
)]
pub struct QuotaViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub subject: String,
    pub description: String,
}

pub type QuotaViolation = QuotaViolationV1;

impl QuotaViolationV1 {
    pub fn new(subject: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            subject: subject.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.quota_failure.v1~",
    description = "Quota failure with one or more violations",
    properties = "violations"
)]
pub struct QuotaFailureV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub violations: Vec<QuotaViolation>,
}

pub type QuotaFailure = QuotaFailureV1;

impl QuotaFailureV1 {
    pub fn new(violations: impl Into<Vec<QuotaViolation>>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            violations: violations.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.precondition_violation.v1~",
    description = "A single precondition violation entry",
    properties = "precondition_type,subject,description"
)]
pub struct PreconditionViolationV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    #[serde(rename = "type")]
    pub precondition_type: String,
    pub subject: String,
    pub description: String,
}

pub type PreconditionViolation = PreconditionViolationV1;

impl PreconditionViolationV1 {
    pub fn new(
        precondition_type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            precondition_type: precondition_type.into(),
            subject: subject.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.precondition_failure.v1~",
    description = "Precondition failure with one or more violations",
    properties = "violations"
)]
pub struct PreconditionFailureV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub violations: Vec<PreconditionViolation>,
}

pub type PreconditionFailure = PreconditionFailureV1;

impl PreconditionFailureV1 {
    pub fn new(violations: impl Into<Vec<PreconditionViolation>>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            violations: violations.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.debug_info.v1~",
    description = "Debug information with detail and stack trace",
    properties = "detail,stack_entries"
)]
pub struct DebugInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub detail: String,
    pub stack_entries: Vec<String>,
}

pub type DebugInfo = DebugInfoV1;

impl DebugInfoV1 {
    pub fn new(detail: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            detail: detail.into(),
            stack_entries: Vec::new(),
        }
    }

    pub fn with_stack(mut self, entries: impl Into<Vec<String>>) -> Self {
        self.stack_entries = entries.into();
        self
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.retry_info.v1~",
    description = "Retry information for unavailable errors",
    properties = "retry_after_seconds"
)]
pub struct RetryInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub retry_after_seconds: u64,
}

pub type RetryInfo = RetryInfoV1;

impl RetryInfoV1 {
    pub fn after_seconds(seconds: u64) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            retry_after_seconds: seconds,
        }
    }
}

#[derive(Debug, Clone)]
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.cf.core.errors.request_info.v1~",
    description = "Request identification context",
    properties = "request_id"
)]
pub struct RequestInfoV1 {
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    gts_type: gts::GtsSchemaId,
    pub request_id: String,
}

pub type RequestInfo = RequestInfoV1;

impl RequestInfoV1 {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            gts_type: Self::gts_schema_id().clone(),
            request_id: request_id.into(),
        }
    }
}
