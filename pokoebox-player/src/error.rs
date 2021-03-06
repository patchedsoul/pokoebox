use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Main application error structure.
#[derive(Debug)]
pub struct Error {
    description: String,
}

impl Error {
    /// Create a new application error instance.
    /// A brief description of the error must be passed to the `description`
    /// parameter.
    pub fn new<S>(description: S) -> Self
    where
        S: Into<String>,
    {
        Error {
            description: description.into(),
        }
    }

    /// Create a new application error instance, that is wrapped in an result object.
    /// This method always returns the `Err` type holding the error.
    /// A brief description of the error must be passed to the `description`
    /// parameter.
    pub fn new_err<T>(description: &'static str) -> Result<T, Self> {
        Err(Self::new(description))
    }
}

/// Implement the `Error` trait, to define this structure as error.
impl error::Error for Error {
    /// Get the error description.
    fn description(&self) -> &str {
        &self.description
    }
}

/// Implement the `Display` trait, required by the `Error` trait.
impl Display for Error {
    /// Format the error, to make it displayable in the console.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
