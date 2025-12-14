#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextcodeError {
    Io,
}

impl std::error::Error for TextcodeError {}

impl std::fmt::Display for TextcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextcodeError::Io => write!(f, "I/O Error occurred"),
        }
    }
}
