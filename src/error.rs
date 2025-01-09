use fetcher::FetcherError;
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error), uniffi(flat_error))]
#[cfg_attr(target_arch = "wasm32", derive(wasm::Error))]
pub enum Error {
  #[error("session expired, you need to authenticate again")]
  ExpiredSession(),
  #[error("server replied with an error ({0})")]
  ServerError(String),
  #[error(transparent)]
  FetcherError(#[from] FetcherError),
}