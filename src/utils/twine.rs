use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) enum Twine {
    Owned(Arc<String>),
    Borrowed(&'static str)
}

impl Twine {
    pub(crate) const fn from_str(s: &'static str) -> Self {
        Twine::Borrowed(s)
    }
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Twine::Owned(s) => s.as_str(),
            Twine::Borrowed(s) => s
        }
    }
}

impl From<String> for Twine {
    fn from(s: String) -> Self {
        Twine::Owned(Arc::new(s))
    }
}

impl From<&'static str> for Twine {
    fn from(s: &'static str) -> Self {
        Twine::Borrowed(s)
    }
}

impl From<Arc<String>> for Twine {
    fn from(s: Arc<String>) -> Self {
        Twine::Owned(s)
    }
}

impl From<&Arc<String>> for Twine {
    fn from(s: &Arc<String>) -> Self {
        Twine::Owned(s.clone())
    }
}

impl From<&String> for Twine {
    fn from(s: &String) -> Self {
        Twine::Owned(Arc::new(s.clone()))
    }
}

impl Display for Twine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq for Twine {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Twine {}

impl std::hash::Hash for Twine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl PartialOrd for Twine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Twine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}