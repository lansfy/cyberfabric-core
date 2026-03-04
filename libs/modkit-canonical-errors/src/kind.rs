// ---------------------------------------------------------------------------
// ErrorKind – per-category metadata without data fields
// ---------------------------------------------------------------------------

pub(crate) struct ErrorMeta {
    pub(crate) gts_type: &'static str,
    pub(crate) status_code: u16,
    pub(crate) title: &'static str,
    pub(crate) category_name: &'static str,
}

pub(crate) const ERROR_META: [ErrorMeta; 16] = [
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.cancelled.v1~",           status_code: 499, title: "Cancelled",           category_name: "cancelled" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~",             status_code: 500, title: "Unknown",              category_name: "unknown" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.invalid_argument.v1~",    status_code: 400, title: "Invalid Argument",     category_name: "invalid_argument" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.deadline_exceeded.v1~",   status_code: 504, title: "Deadline Exceeded",    category_name: "deadline_exceeded" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~",           status_code: 404, title: "Not Found",            category_name: "not_found" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.already_exists.v1~",      status_code: 409, title: "Already Exists",       category_name: "already_exists" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.permission_denied.v1~",   status_code: 403, title: "Permission Denied",    category_name: "permission_denied" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.resource_exhausted.v1~",  status_code: 429, title: "Resource Exhausted",   category_name: "resource_exhausted" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.failed_precondition.v1~", status_code: 400, title: "Failed Precondition",  category_name: "failed_precondition" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.aborted.v1~",             status_code: 409, title: "Aborted",              category_name: "aborted" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.out_of_range.v1~",        status_code: 400, title: "Out of Range",         category_name: "out_of_range" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.unimplemented.v1~",       status_code: 501, title: "Unimplemented",        category_name: "unimplemented" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~",            status_code: 500, title: "Internal",             category_name: "internal" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.service_unavailable.v1~", status_code: 503, title: "Unavailable",          category_name: "unavailable" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.data_loss.v1~",           status_code: 500, title: "Data Loss",            category_name: "data_loss" },
    ErrorMeta { gts_type: "gts.cf.core.errors.err.v1~cf.core.errors.unauthenticated.v1~",     status_code: 401, title: "Unauthenticated",      category_name: "unauthenticated" },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ErrorKind {
    Cancelled = 0,
    Unknown = 1,
    InvalidArgument = 2,
    DeadlineExceeded = 3,
    NotFound = 4,
    AlreadyExists = 5,
    PermissionDenied = 6,
    ResourceExhausted = 7,
    FailedPrecondition = 8,
    Aborted = 9,
    OutOfRange = 10,
    Unimplemented = 11,
    Internal = 12,
    ServiceUnavailable = 13,
    DataLoss = 14,
    Unauthenticated = 15,
}

impl ErrorKind {
    fn meta(&self) -> &'static ErrorMeta {
        &ERROR_META[*self as usize]
    }

    pub fn gts_type(&self) -> &'static str {
        self.meta().gts_type
    }

    pub fn status_code(&self) -> u16 {
        self.meta().status_code
    }

    pub fn title(&self) -> &'static str {
        self.meta().title
    }

    pub fn category_name(&self) -> &'static str {
        self.meta().category_name
    }
}
