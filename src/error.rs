#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    CliError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Other(Box::new(err))
    }
}

pub fn map_error<E: std::error::Error + 'static>(err: E) -> Error {
    Error::Other(Box::new(err))
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Other(err) => write!(f, "some error: {:#?}", err),
            Error::CliError(msg) => write!(f, "{:#?}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
