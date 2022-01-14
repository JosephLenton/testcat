use ::std::convert::From;
use ::std::fmt;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    TestDescriptionExpected,
    EmptyDescriptionGiven,
    CommaExpected,
    TestNameExpected,
    CodeBlockExpected,
    ExcessTokensFound,
    FmtError(fmt::Error),
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::FmtError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TestDescriptionExpected => write!(f, "Expected test description"),
            Error::EmptyDescriptionGiven => write!(f, "Empty test description given"),
            Error::CommaExpected => write!(f, "Expected comma seperator"),
            Error::TestNameExpected => write!(f, "Expected test name"),
            Error::CodeBlockExpected => write!(f, "Expected code block"),
            Error::ExcessTokensFound => write!(f, "Extra tokens found"),
            Error::FmtError(fmt) => write!(
                f,
                "Internal error; failed writing to string (this should never be visible), {}",
                fmt
            ),
        }
    }
}
