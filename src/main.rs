#[allow(unused)]

use crate::caver::{
  types::{Block,BlockNumber},
  api::{Klay}
};

mod caver;

const RPC_URL: &str = "http://localhost:8551";
const CONCURRENCY: i32 = 10;
const BATCH_SIZE: i32 = 100;
const DELAY: u64 = 0;

async fn too_many_blocks(thread_id: i32) -> caver::Result<()> {
  let provider = caver::providers::Http::new(RPC_URL);
  let caver = caver::Client::new(provider);
  let klay = caver.klay();

  if BATCH_SIZE == 0 {
    loop {
      println!("{}", thread_id);
      klay.get_block_by_number(BlockNumber::Latest).await;
      std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }
  } else {
    for j in 0..BATCH_SIZE {
      println!("{}, {}", thread_id, j);
      klay.get_block_by_number(BlockNumber::Latest).await;
      std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }
  }

  Ok(())
}

#[tokio::main]
pub async fn main() -> caver::Result<()> {
  let mut handles = Vec::new();

  for i in 0..CONCURRENCY {
    handles.push(tokio::spawn(async move { too_many_blocks(i).await }));
  }


  for handle in handles {
    tokio::join!(handle);
  }

  Ok(())
}
