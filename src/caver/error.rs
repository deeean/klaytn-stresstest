use derive_more::{Display, From};
use serde_json::Error as SerdeError;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Display, Debug, Clone, PartialEq)]
pub enum ProviderError {
  /// Transport-specific error code.
  #[display(fmt = "code {}", _0)]
  Code(u16),
  /// Arbitrary, developer-readable description of the occurred error.
  #[display(fmt = "{}", _0)]
  Message(String),
}

#[derive(Debug, Display, From)]
pub enum Error {
  #[display(fmt = "Decoder error: {}", _0)]
  Decoder(String),

  #[display(fmt = "Provider error: {}", _0)]
  #[from(ignore)]
  Provider(ProviderError),

  #[display(fmt = "RPC error: {:?}", _0)]
  Rpc(jsonrpc_core::Error),
}

impl From<SerdeError> for Error {
  fn from(err: SerdeError) -> Self {
    Error::Decoder(format!("{:?}", err))
  }
}