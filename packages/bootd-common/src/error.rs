use alloc::string::String;
use alloc::format;


#[derive(Debug)]
pub struct UefiError {
    err: uefi::Error<()>,
    data: String,
}

impl<T: core::fmt::Debug> From<uefi::Error<T>> for UefiError {
    fn from(err: uefi::Error<T>) -> UefiError {
        UefiError {
            err: err.to_err_without_payload(),
            data: format!("{:?}", err.data()),
        }
    }
}


