use std::fmt::{Display, Formatter, Result};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

fn error_code_to_message(e: &ValidationError) -> String {
    if let Some(message) = e.message.as_ref() {
        return message.as_ref().to_owned();
    }

    match e.code.as_ref() {
        "length" => match (
            e.params.get("min"),
            e.params.get("max"),
            e.params.get("equal"),
        ) {
            (Some(min), None, _) => format!("must be at least {} characters long", min),
            (None, Some(max), _) => {
                format!("must be less than or equal to {} characters long", max)
            }
            (Some(min), Some(max), _) => {
                format!("must be between {} and {} characters long", min, max)
            }
            (_, _, Some(equal)) => format!("must be equal to {} characters long", equal),
            _ => unreachable!(),
        },
        _ => format!("{} [{:?}]", e.code, e.params),
    }
}

fn errors_to_message(path: String, errors: &ValidationErrors) -> Option<String> {
    errors
        .errors()
        .iter()
        .next()
        .and_then(|(field, error)| match error {
            ValidationErrorsKind::Struct(e) => {
                errors_to_message(format!("{}.{}", path, field), e.as_ref())
            }
            ValidationErrorsKind::List(e) => e.iter().next().and_then(|(k, v)| {
                errors_to_message(format!("{}.{}[{}]", path, field, k), v.as_ref())
            }),
            ValidationErrorsKind::Field(field_error) => field_error
                .first()
                .map(|e| format!("The field {}.{} {}", path, field, error_code_to_message(e))),
        })
}

#[derive(Debug)]
pub enum Error {
    Unkown(String),
    ValidationErrors(ValidationErrors),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let err = match self {
            Error::ValidationErrors(errors) => {
                errors_to_message("".to_string(), errors).unwrap_or("".to_owned())
            }
            Error::Unkown(err) => err.to_owned(),
        };

        write!(f, "{}", err)
    }
}

impl std::error::Error for Error {}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Error::ValidationErrors(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Unkown(e.to_owned())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Unkown(e)
    }
}

impl From<couchbase::CouchbaseError> for Error {
    fn from(e: couchbase::CouchbaseError) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<eventstore::Error> for Error {
    fn from(e: eventstore::Error) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<eventstore::WrongExpectedVersion> for Error {
    fn from(e: eventstore::WrongExpectedVersion) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<mobc_redis::redis::RedisError> for Error {
    fn from(e: mobc_redis::redis::RedisError) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<mobc::Error<mobc_redis::redis::RedisError>> for Error {
    fn from(e: mobc::Error<mobc_redis::redis::RedisError>) -> Self {
        Error::Unkown(e.to_string())
    }
}

impl From<actix::MailboxError> for Error {
    fn from(e: actix::MailboxError) -> Self {
        Error::Unkown(e.to_string())
    }
}
