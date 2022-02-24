#[allow(unused)]

use crate::caver::{
  types::{Block,BlockNumber},
  api::{Klay}
};
use std::sync::Arc;
use tokio::time::sleep;
use tokio::time::Instant;
use ethereum_types::U64;

mod caver;

const RPC_URL: &str = "http://localhost:8551";
const DELAY: u64 = 0;
const CONCURRENCY: i32 = 30;
const BATCH_SIZE: i32 = 0;

struct State {
  pub total: i32,
  pub healthy: i32,
  pub unhealthy: i32,
  pub total_latency: u128,
  pub latest_latency: u128,
  pub block_numbers: Vec<U64>
}

impl State {
  pub fn new() -> Self {
    Self{
      total: 0,
      healthy: 0,
      unhealthy: 0,
      total_latency: 0,
      latest_latency: 0,
      block_numbers: Vec::new()
    }
  }

  pub fn success(&mut self, block_number: U64, latency: u128) {
    self.total += 1;
    self.healthy += 1;
    self.total_latency += latency;
    self.latest_latency = latency;
    self.block_numbers.push(block_number);

    if self.block_numbers.len() > 10 {
      self.block_numbers.remove(0);
    }

    self.check_print_iteration();
  }

  pub fn failed(&mut self, latency: u128) {
    self.total += 1;
    self.unhealthy += 1;
    self.total_latency += latency;
    self.latest_latency = latency;

    self.check_print_iteration();
  }

  pub fn check_print_iteration(&self) {
    if self.total % 100 == 0 {
      self.print();
    }
  }

  pub fn print(&self) {
    println!("Total {}", self.total);
    println!("Average Latency: {:.2}ms", self.total_latency as f64 / self.total as f64);
    println!("Latest Latency: {:.2}ms", self.latest_latency);
    println!("Healthy {}", self.healthy);
    println!("Unhealthy {}", self.unhealthy);
    println!("BlockNumbers {:?}", self.block_numbers);
    println!();
  }
}

#[tokio::main]
pub async fn main() -> caver::Result<()> {
  let mut handles = Vec::new();
  let state = Arc::new(std::sync::Mutex::new(State::new()));

  for _ in 0..CONCURRENCY {
    let cloned_state = state.clone();

    handles.push(tokio::spawn(async move {
      let provider = caver::providers::Http::new(RPC_URL);
      let caver = caver::Client::new(provider);
      let klay = caver.klay();
      let mut count = 0;

      loop {
        let now = Instant::now();

        match klay.get_block_by_number(BlockNumber::Latest).await {
          Ok(it) => {
            match it {
              Some(block) => {
                cloned_state.lock().unwrap().success(block.number, now.elapsed().as_millis());
              },
              None => {}
            }
          }
          Err(e) => {
            cloned_state.lock().unwrap().failed(now.elapsed().as_millis())
          }
        }

        if DELAY != 0 {
          sleep(std::time::Duration::from_millis(DELAY)).await;
        }

        count += 1;

        if BATCH_SIZE != 0 && count >= BATCH_SIZE {
          break;
        }
      }
    }));
  }

  for handle in handles {
    tokio::join!(handle);
  }

  Ok(())
}
