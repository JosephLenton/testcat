use ::std::convert::From;
use ::std::fmt;

pub type Result<N> = ::std::result::Result<N, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    TestDescriptionExpected,
    EmptyDescriptionGiven,
    CommaExpected,
    CodeBlockExpected,
    ExcessTokensFound,
    SynError(::syn::Error),
    FmtError(fmt::Error),
}

impl From<::syn::Error> for Error {
    fn from(err: ::syn::Error) -> Self {
        Error::SynError(err)
    }
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
            Error::CodeBlockExpected => write!(f, "Expected code block"),
            Error::ExcessTokensFound => write!(f, "Extra tokens found"),
            Error::SynError(syn_error) => write!(f, "{}", syn_error,),
            Error::FmtError(fmt_error) => write!(
                f,
                "Internal error; failed writing to string (this should never be visible), {}",
                fmt_error,
            ),
        }
    }
}
