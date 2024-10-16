use std::error::Error;
use std::fmt::{Debug, Display};

pub struct PenyuError {
    message: String,
    source: Option<Box<dyn Error>>
}

impl PenyuError {
    pub fn new(message: String, source: Option<Box<dyn Error>>) -> Self {
        Self { message, source }
    }
}

impl Display for PenyuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        let mut source = self.source();
        while let Some(err) = source {
            write!(f, ": {}", err)?;
            source = err.source();
        }
        Ok(())
    }
}

impl Debug for PenyuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for PenyuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl From<std::io::Error> for PenyuError {
    fn from(error: std::io::Error) -> Self {
        PenyuError::new("I/O error".to_string(), Some(Box::new(error)))
    }
}