use std::fmt::{Debug, Formatter};

/// GeneralError marker Trait
pub trait GeneralError: std::error::Error {}

/// TradingErrorKind marker Trait
pub trait GeneralErrorKind: Copy + Debug {}

/// The general Error used when trading
///
/// This error struct provides all the functionality needed when an error occurs while trading
/// Usually the individual ErrorKinds provide more, domain specific, information and documentation.
#[derive(Clone, Debug)]
pub struct Error<K: GeneralErrorKind> {
    msg: String,
    kind: K,
}

impl<K: GeneralErrorKind> Error<K> {
    pub fn new(msg: String, kind: K) -> Self {
        Self {
            msg,
            kind,
        }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn kind(&self) -> K {
        self.kind
    }
}

impl<K: GeneralErrorKind> std::error::Error for Error<K> {}

impl<K: GeneralErrorKind> GeneralError for Error<K> {}

impl<K: GeneralErrorKind> std::fmt::Display for Error<K> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = self.msg
                      .trim()
                      .replace('\n', "\n\t");

        write!(
            formatter,
            "Error ({:?}):\n\
            \t{}",
            self.kind,
            msg
        )
    }
}

impl From<libloading::Error> for Error<ErrorKind> {
    fn from(error: libloading::Error) -> Self {
        Self {
            msg: format!("{:?}", error),
            kind: ErrorKind::LibLoading,
        }
    }
}

impl From<std::io::Error> for Error<ErrorKind> {
    fn from(err: std::io::Error) -> Self {
        Self {
            msg: err.to_string(),
            kind: ErrorKind::IO,
        }
    }
}

/// The basic ErrorKind enum
///
/// The ErrorKind enum has the purpose to cover a great range of different errors without
/// going into details about the error.
/// It's also not meant to provide any features or special functionality.
///
/// It should be used if there's no more detailed ErrorKind available or if the detailed
/// ErrorKind information is unimportant or undesired.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorKind {
    Broker,
    Bank,
    Trading,
    LibLoading,
    IO,
    MisMatchedVersion,
    Other,
}

impl GeneralErrorKind for ErrorKind {}

/// The TradingErrorKind
///
/// The TradingErrorKind has the purpose to go into greater detail about common trading errors.
///
/// It should be used for trading algorithms.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TradingErrorKind {}

impl GeneralErrorKind for TradingErrorKind {}
