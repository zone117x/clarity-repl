use crate::clarity::{
    diagnostic::{DiagnosableError, Level},
    representations::Span,
};

#[derive(Debug, PartialEq, Clone)]
pub enum LexerError {
    InvalidCharInt(char),
    InvalidCharUint(char),
    InvalidCharBuffer(char),
    InvalidCharIdent(char),
    WarningCharIdent(char),
    InvalidCharTraitIdent(char),
    InvalidCharPrincipal(char),
    InvalidBufferLength(usize),
    UnknownEscapeChar(char),
    IllegalCharString(char),
    IllegalCharUTF8Encoding(char),
    UnterminatedUTF8Encoding,
    ExpectedClosing(char),
    ExpectedSeparator,
    EmptyUTF8Encoding,
    InvalidUTF8Encoding,
    SingleSemiColon,
    UnknownSymbol(char),
    NoteToMatchThis(char),
}

#[derive(Debug)]
pub struct PlacedError {
    pub e: LexerError,
    pub span: Span,
}

impl DiagnosableError for LexerError {
    fn message(&self) -> String {
        use self::LexerError::*;
        match self {
            InvalidCharInt(c) => format!("invalid character, '{}', in int literal", c),
            InvalidCharUint(c) => format!("invalid character, '{}', in uint literal", c),
            InvalidCharBuffer(c) => format!("invalid character, '{}', in buffer", c),
            InvalidCharIdent(c) => format!("invalid character, '{}', in identifier", c),
            WarningCharIdent(c) => format!("identifiers containing a '{}' are bad for readability and may be illegal in a future version of Clarity", c),
            InvalidCharTraitIdent(c) => format!("invalid character, '{}', in trait identifier", c),
            InvalidCharPrincipal(c) => format!("invalid character, '{}', in principal literal", c),
            IllegalCharString(c) => format!("invalid character, '{}', in string literal", c),
            IllegalCharUTF8Encoding(c) => format!("invalid character, '{}', in UTF8 encoding", c),
            InvalidUTF8Encoding => "invalid UTF8 encoding".to_string(),
            EmptyUTF8Encoding => "empty UTF8 encoding".to_string(),
            UnterminatedUTF8Encoding => "unterminated UTF8 encoding, missing '}'".to_string(),
            InvalidBufferLength(size) => format!("invalid buffer length, {}", size),
            UnknownEscapeChar(c) => format!("unknown escape character, '{}'", c),
            ExpectedClosing(c) => format!("expected closing '{}'", c),
            ExpectedSeparator => "expected separator".to_string(),
            SingleSemiColon => "unexpected single ';' (comments begin with \";;\"".to_string(),
            UnknownSymbol(c) => format!("unknown symbol, '{}'", c),
            NoteToMatchThis(c) => format!("to match this '{}'", c),
        }
    }

    fn suggestion(&self) -> Option<String> {
        None
    }

    fn level(&self) -> crate::clarity::diagnostic::Level {
        use self::LexerError::*;
        match self {
            NoteToMatchThis(_) => Level::Note,
            WarningCharIdent(_) => Level::Warning,
            _ => Level::Error,
        }
    }
}
