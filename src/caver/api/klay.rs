use ethereum_types::H256;
use crate::caver::{CallFuture, Provider};
use crate::caver::types::{Block, BlockNumber};
use crate::caver::utils::serialize;

#[derive(Debug, Clone)]
pub struct Klay<T> {
  provider: T
}

impl<T: Provider> Klay<T> {
  pub fn new(provider: T) -> Self {
    Klay {
      provider
    }
  }

  pub fn get_block_by_number(&self, block_number: BlockNumber) -> CallFuture<Option<Block<H256>>, T::Out> {
    let include_transactions = serialize(&false);
    let value = serialize(&block_number);
    CallFuture::new(self.provider.execute("klay_getBlockByNumber", vec![value, include_transactions]))
  }
}