use std::fmt;

use mupdf_sys::*;

#[derive(Debug, Clone)]
pub struct MuPdfError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for MuPdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MuPDF error, code: {}, message: {}",
            self.code, &self.message
        )
    }
}

impl std::error::Error for MuPdfError {}

pub unsafe fn ffi_error() -> Option<MuPdfError> {
    use std::ffi::CStr;

    let err = mupdf_error();
    if err.is_null() {
        return None;
    }
    let code = (*err).type_;
    let c_msg = (*err).message;
    let c_str = CStr::from_ptr(c_msg);
    let message = format!("{}", c_str.to_string_lossy());
    Some(MuPdfError { code, message })
}

macro_rules! ffi_try {
    ($func:ident($($arg:expr),+)) => ({
        let res = $func($($arg),+);
        if let Some(err) = $crate::ffi_error() {
            return Err(err.into());
        }
        res
    });
    ($func:ident()) => ({
        let res = $func();
        if let Some(err) = $crate::ffi_error() {
            return Err(err.into());
        }
        res
    })
}

#[derive(Debug)]
pub enum Error {
    MuPdf(MuPdfError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::MuPdf(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<MuPdfError> for Error {
    fn from(err: MuPdfError) -> Self {
        Self::MuPdf(err)
    }
}