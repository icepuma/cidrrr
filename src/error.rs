use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    InvalidCidr {
        input: String,
        source: ipnet::AddrParseError,
    },
    Io(std::io::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCidr { input, source } => {
                write!(f, "'{input}' is an invalid CIDR: {source}")
            }
            Self::Io(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCidr { source, .. } => Some(source),
            Self::Io(source) => Some(source),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        Self::Io(source)
    }
}
