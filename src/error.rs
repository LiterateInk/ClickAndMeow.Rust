use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Couldn't login, check your credentials")]
  InvalidCredentials(),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  HeaderToStr(#[from] reqwest::header::ToStrError),
  #[error(transparent)]
  UrlParse(#[from] url::ParseError),
  #[error(transparent)]
  InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
