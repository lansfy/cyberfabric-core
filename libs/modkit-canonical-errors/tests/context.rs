extern crate cf_modkit_canonical_errors as modkit_canonical_errors;

use modkit_canonical_errors::{
    DebugInfoV1, ErrorInfoV1, FieldViolation, FieldViolationV1, PreconditionFailureV1,
    PreconditionViolationV1, QuotaFailureV1, QuotaViolationV1, RequestInfoV1, ResourceInfoV1,
    RetryInfoV1, Validation,
};

#[test]
fn validation_field_violations_serialization() {
    let v = Validation::fields(vec![FieldViolation::new(
        "email",
        "must be valid",
        "INVALID_FORMAT",
    )]);
    let json = serde_json::to_value(&v).unwrap();
    assert!(json["field_violations"].is_array());
    assert_eq!(json["field_violations"][0]["field"], "email");
}

#[test]
fn validation_format_serialization() {
    let v = Validation::format("bad json");
    let json = serde_json::to_value(&v).unwrap();
    assert_eq!(json["format"], "bad json");
}

#[test]
fn validation_constraint_serialization() {
    let v = Validation::constraint("too many items");
    let json = serde_json::to_value(&v).unwrap();
    assert_eq!(json["constraint"], "too many items");
}

// =========================================================================
// GTS Schema tests — full JSON comparison for each context type
// =========================================================================

#[test]
fn schema_retry_info_v1() {
    use gts::schema::GtsSchema;
    let schema = RetryInfoV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.retry_info.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "retry_after_seconds"],
            "properties": {
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "retry_after_seconds": {
                    "format": "uint64",
                    "minimum": 0,
                    "type": "integer"
                }
            }
        })
    );
}

#[test]
fn schema_request_info_v1() {
    use gts::schema::GtsSchema;
    let schema = RequestInfoV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.request_info.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "request_id"],
            "properties": {
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "request_id": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_resource_info_v1() {
    use gts::schema::GtsSchema;
    let schema = ResourceInfoV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.resource_info.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "resource_type", "resource_name", "description"],
            "properties": {
                "description": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "resource_name": {
                    "type": "string"
                },
                "resource_type": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_error_info_v1() {
    use gts::schema::GtsSchema;
    let schema = ErrorInfoV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.error_info.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "reason", "domain", "metadata"],
            "properties": {
                "domain": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "metadata": {
                    "additionalProperties": {
                        "type": "string"
                    },
                    "type": "object"
                },
                "reason": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_field_violation_v1() {
    use gts::schema::GtsSchema;
    let schema = FieldViolationV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.field_violation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "field", "description", "reason"],
            "properties": {
                "description": {
                    "type": "string"
                },
                "field": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "reason": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_debug_info_v1() {
    use gts::schema::GtsSchema;
    let schema = DebugInfoV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.debug_info.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "detail", "stack_entries"],
            "properties": {
                "detail": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "stack_entries": {
                    "items": {
                        "type": "string"
                    },
                    "type": "array"
                }
            }
        })
    );
}

#[test]
fn schema_quota_violation_v1() {
    use gts::schema::GtsSchema;
    let schema = QuotaViolationV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.quota_violation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "subject", "description"],
            "properties": {
                "description": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "subject": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_quota_failure_v1() {
    use gts::schema::GtsSchema;
    let schema = QuotaFailureV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.quota_failure.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "violations"],
            "properties": {
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "violations": {
                    "items": {
                        "$ref": "#/$defs/QuotaViolationV1"
                    },
                    "type": "array"
                }
            }
        })
    );
}

#[test]
fn schema_precondition_violation_v1() {
    use gts::schema::GtsSchema;
    let schema = PreconditionViolationV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.precondition_violation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "type", "subject", "description"],
            "properties": {
                "description": {
                    "type": "string"
                },
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "subject": {
                    "type": "string"
                },
                "type": {
                    "type": "string"
                }
            }
        })
    );
}

#[test]
fn schema_precondition_failure_v1() {
    use gts::schema::GtsSchema;
    let schema = PreconditionFailureV1::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.precondition_failure.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "additionalProperties": false,
            "type": "object",
            "required": ["gts_type", "violations"],
            "properties": {
                "gts_type": {
                    "description": "GTS schema identifier",
                    "format": "gts-schema-id",
                    "title": "GTS Schema ID",
                    "type": "string",
                    "x-gts-ref": "gts.*"
                },
                "violations": {
                    "items": {
                        "$ref": "#/$defs/PreconditionViolationV1"
                    },
                    "type": "array"
                }
            }
        })
    );
}

#[test]
fn schema_validation() {
    use gts::schema::GtsSchema;
    let schema = Validation::gts_schema_with_refs();
    assert_eq!(
        schema,
        serde_json::json!({
            "$id": "gts://gts.cf.core.errors.validation.v1~",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "field_violations": {
                            "type": "array",
                            "items": {
                                "$ref": "gts://gts.cf.core.errors.field_violation.v1~"
                            }
                        }
                    },
                    "required": ["field_violations"]
                },
                {
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string"
                        }
                    },
                    "required": ["format"]
                },
                {
                    "type": "object",
                    "properties": {
                        "constraint": {
                            "type": "string"
                        }
                    },
                    "required": ["constraint"]
                }
            ]
        })
    );
}
