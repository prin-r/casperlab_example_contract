use casperlabs_types::ApiError;

#[repr(u16)]
pub enum Error {
    UnknownBridgeCallCommand = 1,      // 65537
    UnknownApiCommand = 2,             // 65538
    FailToDecodeProof = 3,             // 65539
    MissingArgument0 = 16,             // 65552
    MissingArgument1 = 17,             // 65553
    MissingArgument2 = 18,             // 65554
    MissingArgument3 = 19,             // 65555
    MissingArgument4 = 20,             // 65556
    MissingArgument5 = 21,             // 65557
    InvalidArgument0 = 22,             // 65558
    InvalidArgument1 = 23,             // 65559
    InvalidArgument2 = 24,             // 65560
    InvalidArgument3 = 25,             // 65561
    InvalidArgument4 = 26,             // 65562
    InvalidArgument5 = 27,             // 65563
    UnsupportedNumberOfArguments = 28, // 65564
    TestError = 999,
}

impl Error {
    pub fn missing_argument(i: u32) -> Error {
        match i {
            0 => Error::MissingArgument0,
            1 => Error::MissingArgument1,
            2 => Error::MissingArgument2,
            3 => Error::MissingArgument3,
            4 => Error::MissingArgument4,
            5 => Error::MissingArgument5,
            _ => Error::UnsupportedNumberOfArguments,
        }
    }

    pub fn invalid_argument(i: u32) -> Error {
        match i {
            0 => Error::InvalidArgument0,
            1 => Error::InvalidArgument1,
            2 => Error::InvalidArgument2,
            3 => Error::InvalidArgument3,
            4 => Error::InvalidArgument4,
            5 => Error::InvalidArgument5,
            _ => Error::UnsupportedNumberOfArguments,
        }
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
