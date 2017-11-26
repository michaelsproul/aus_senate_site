use iron::prelude::*;
use iron::status;

use self::Error::*;

#[derive(Clone, Copy, Debug, Error)]
pub enum Error {
    /// State or territory missing or invalid.
    InvalidState,
    /// Code bug or logic flaw.
    InternalErr,
}

impl From<Error> for IronError {
    fn from(app_err: Error) -> IronError {
        match app_err {
            InvalidState => IronError::new(app_err, status::BadRequest),
            InternalErr => IronError::new(app_err, status::InternalServerError),
        }
    }
}
