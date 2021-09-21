use super::method::{Method, MethodError};
use core::str::Utf8Error;
use std::convert::TryFrom;
use std::str;

pub struct Request<'a> {
    path: &'a str,
    query_string: Option<&'a str>,
    method: super::method::Method,
}

impl<'b> Request<'b> {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a > {
    type Error = ParseError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let result = str::from_utf8(value)?;
        let (method, request) = get_next_word(result).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(result).ok_or(ParseError::InvalidRequest)?;
        let (protocol, __) = get_next_word(result).ok_or(ParseError::InvalidRequest)?;

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find(("?")) {
            query_string = Some(&path[i..]);
            path = &path[..i + 1]
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(word: &str) -> Option<(&str, &str)> {
    for (i, c) in word.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&word[i..], &word[..i + 1]));
        }
    }

    None
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidRequest => "Invalid request",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}
