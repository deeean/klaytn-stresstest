use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use pin_project::pin_project;
use std::task::{Context, Poll};
use futures::ready;

pub mod api;
pub mod error;
pub mod types;
pub mod providers;
pub mod utils;

pub use self::{error::Result};

#[derive(Debug, Clone)]
pub struct Client<T: Provider> {
  provider: T
}

pub type RequestId = usize;

impl<T: Provider> Client<T> {
  pub fn new(provider: T) -> Self {
    Client {
      provider
    }
  }

  pub fn klay(&self) -> api::klay::Klay<T> {
    api::klay::Klay::new(self.provider.clone())
  }
}

pub trait Provider: std::fmt::Debug + Clone {
  type Out: futures::Future<Output = error::Result<jsonrpc_core::Value>>;

  fn prepare(&self, method: &str, params: Vec<jsonrpc_core::Value>) -> (RequestId, jsonrpc_core::Call);

  fn send(&self, id: RequestId, request: jsonrpc_core::Call) -> Self::Out;

  fn execute(&self, method: &str, params: Vec<jsonrpc_core::Value>) -> Self::Out {
    let (id, request) = self.prepare(method, params);
    self.send(id, request)
  }
}

pub fn decode<T: serde::de::DeserializeOwned>(value: jsonrpc_core::Value) -> error::Result<T> {
  serde_json::from_value(value).map_err(Into::into)
}

#[pin_project]
#[derive(Debug)]
pub struct CallFuture<T, F> {
  #[pin]
  store: F,
  _marker: PhantomData<T>,
}

impl<T, F> CallFuture<T, F> {
  pub fn new(store: F) -> Self {
    CallFuture {
      store,
      _marker: PhantomData,
    }
  }
}

impl<T, F> Future for CallFuture<T, F>
  where
    T: serde::de::DeserializeOwned,
    F: Future<Output = error::Result<jsonrpc_core::Value>>,
{
  type Output = error::Result<T>;

  fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
    let this = self.project();
    let x = ready!(this.store.poll(ctx));
    Poll::Ready(x.and_then(decode))
  }
}
