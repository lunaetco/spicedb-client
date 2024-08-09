use http::uri::InvalidUri;
use thiserror::Error;
use tonic::metadata::errors::InvalidMetadataValue;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidMetadataValue(#[from] InvalidMetadataValue),

    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),

    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),

    #[error(transparent)]
    TonicStatus(#[from] tonic::Status),
}
