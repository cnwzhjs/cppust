use std::fmt::Debug;

use syn::{Fields, PathSegment, Type};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("config error: [{0}] {1}")]
    ConfigError(String, String),

    #[error("unknown data type {0:?}")]
    UnknownType(Type),

    #[error("unknown data type {0:?}")]
    UnknownFieldsType(Fields),

    #[error("invalid type path segment {0:?}")]
    InvalidTypePathSegment(PathSegment),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SynError(#[from] syn::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
