use fetcher::FetcherError;
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error), uniffi(flat_error))]
#[cfg_attr(target_arch = "wasm32", derive(wasm::Error))]
pub enum Error {
  #[error("Couldn't login, check your credentials")]
  InvalidCredentials(),
  #[error(transparent)]
  FetcherError(#[from] FetcherError),
}
