use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use futures::future::BoxFuture;
use jsonrpc_core::{Call, Request, serde_json, Value};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use crate::caver::{Provider, RequestId, error::{Error}};
use crate::caver::error::{Result, ProviderError};

#[derive(Clone, Debug)]
pub struct Http {
  client: reqwest::Client,
  store: Arc<Store>
}

#[derive(Debug)]
struct Store {
  id: AtomicUsize,
  rpc_url: Url
}

impl Http {
  pub fn new(rawurl: &str) -> Self {
    let builder = reqwest::Client::builder();

    // TODO: error handling
    let client = builder
      .build()
      .unwrap();

    // TODO: error handling
    Http {
      client,
      store: Arc::new(Store {
        id: AtomicUsize::new(0),
        rpc_url: rawurl.parse().unwrap()
      })
    }
  }

  fn fetch_add(&self) -> RequestId {
    self.store.id.fetch_add(1, Ordering::AcqRel)
  }

  fn clone(&self) -> (Client, Url) {
    (self.client.clone(), self.store.rpc_url.clone())
  }
}

async fn execute_rpc<T: DeserializeOwned>(client: &Client, rpc_url: Url, request: &Request, id: RequestId) -> Result<T> {
  let res = client
    .post(rpc_url)
    .json(request)
    .send()
    .await
    .map_err(|err| Error::Provider(ProviderError::Message(format!("failed to send request: {}", err))))?;

  let status = res.status();
  let buf = res.bytes().await.map_err(|err| Error::Provider(ProviderError::Message(format!("failed to read response bytes: {}", err))))?;

  if !status.is_success() {
    return Err(Error::Provider(ProviderError::Code(status.as_u16())));
  }

  serde_json::from_slice(&buf).map_err(|err| {
    Error::Provider(ProviderError::Message(format!("{}", err)))
  })
}

type RpcResult = Result<Value>;

impl Provider for Http {
  type Out = BoxFuture<'static, RpcResult>;

  fn prepare(&self, method: &str, params: Vec<Value>) -> (RequestId, Call) {
    let id = self.fetch_add();
    let request = jsonrpc_core::Call::MethodCall(jsonrpc_core::MethodCall {
      id: jsonrpc_core::Id::Num(id as u64),
      jsonrpc: Some(jsonrpc_core::Version::V2),
      method: method.into(),
      params: jsonrpc_core::Params::Array(params),
    });

    (id, request)
  }

  fn send(&self, id: RequestId, request: Call) -> Self::Out {
    let (client, rpc_url) = self.clone();
    Box::pin(async move {
      let output: jsonrpc_core::Output = execute_rpc(&client, rpc_url, &jsonrpc_core::Request::Single(request), id).await?;
      match output {
        jsonrpc_core::Output::Success(success) => Ok(success.result),
        jsonrpc_core::Output::Failure(failure) => Err(Error::Rpc(failure.error)),
      }
    })
  }
}