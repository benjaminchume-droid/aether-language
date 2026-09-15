use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span { pub start: usize, pub end: usize }

impl Span { pub const fn new(start: usize, end: usize) -> Self { Self { start, end } } }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic { pub code: &'static str, pub message: String, pub span: Option<Span> }

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>, span: Option<Span>) -> Self { Self { code, message: message.into(), span } }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(s) => write!(f, "error[{}] {} ({}..{})", self.code, self.message, s.start, s.end),
            None => write!(f, "error[{}] {}", self.code, self.message),
        }
    }
}
