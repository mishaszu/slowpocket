pub const NOT_NULL_VIOLATION: &str = "23502";
pub const FOREIGN_KEY_VIOLATION: &str = "23503";
pub const UNIQUE_VIOLATION: &str = "23505";
pub const CHECK_VIOLATION: &str = "23514";

pub const INSUFFICIENT_PRIVILEGE: &str = "42501";

pub enum ErrorKindExt {
    UniqueViolation,
    ForeignKeyViolation,
    NotNullViolation,
    CheckViolation,
    InsufficientPrivilege,
    Other,
}

pub trait ErrorExt {
    fn kind_ext(&self) -> ErrorKindExt;
}

impl ErrorExt for sqlx::Error {
    fn kind_ext(&self) -> ErrorKindExt {
        let db_err = match self {
            sqlx::Error::Database(value) => value,
            _ => return ErrorKindExt::Other,
        };
        let code = match db_err.code() {
            Some(value) => value,
            _ => return ErrorKindExt::Other,
        };

        match code.as_ref() {
            NOT_NULL_VIOLATION => ErrorKindExt::NotNullViolation,
            FOREIGN_KEY_VIOLATION => ErrorKindExt::ForeignKeyViolation,
            UNIQUE_VIOLATION => ErrorKindExt::UniqueViolation,
            CHECK_VIOLATION => ErrorKindExt::CheckViolation,
            INSUFFICIENT_PRIVILEGE => ErrorKindExt::InsufficientPrivilege,
            _ => ErrorKindExt::Other,
        }
    }
}
