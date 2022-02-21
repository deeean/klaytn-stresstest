#[macro_use]
extern crate serde_derive;

use std::i64;
use std::fmt::Error;
use std::num::ParseIntError;
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

const RPC_URL: &str = "http://localhost:8551";
const CONCURRENCY: i32 = 10;

#[derive(Deserialize, Debug)]
struct RpcPayload<T> {
  id: i32,
  jsonrpc: String,
  result: T,
}

#[derive(Deserialize, Debug)]
struct Block {
  blockscore: String,

  #[serde(rename = "extraData")]
  extra_data: String,

  #[serde(rename = "gasUsed")]
  gas_used: String,

  #[serde(rename = "governanceData")]
  governance_data: String,
  hash: String,

  #[serde(rename = "logsBloom")]
  logs_bloom: String,
  number: String,

  #[serde(rename = "parentHash")]
  parent_hash: String,

  #[serde(rename = "receiptsRoot")]
  receipts_root: String,
  reward: String,
  size: String,

  #[serde(rename = "stateRoot")]
  state_root: String,
  timestamp: String,

  #[serde(rename = "timestampFoS")]
  timestamp_fo_s: String,

  #[serde(rename = "totalBlockScore")]
  total_block_score: String,
  transactions: Vec<String>,

  #[serde(rename = "transactionsRoot")]
  transactions_root: String,

  #[serde(rename = "voteData")]
  vote_data: String,
}

struct Status {
  total: u128,
  healthy: i32,
  unhealthy: i32,
  latency: u128,
  latest_block_number: i64,
}

impl Status {
  pub const fn new() -> Self {
    return Status{
      total: 0,
      healthy: 0,
      unhealthy: 0,
      latency: 0,
      latest_block_number: 0,
    }
  }

  pub fn add(&mut self, is_healthy: bool, latency: u128, latest_block_number: i64) {
    self.total += 1;
    self.latency += latency;

    if is_healthy {
      self.healthy += 1;
      self.latest_block_number = latest_block_number;
    } else {
      self.unhealthy += 1;
    }

    if self.total % 1000 == 0 {
      self.print()
    }
  }

  pub fn print(&self) {
    println!("Total Request: {}", self.total);
    println!("Average Latency: {:.2}ms", self.latency as f64 / self.total as f64);
    println!("Healthy: {}", self.healthy);
    println!("Unhealthy: {}", self.unhealthy);
    println!("Block Number: {}", self.latest_block_number);
    println!();
  }
}

fn get_latest_block() -> Result<RpcPayload<Block>, reqwest::Error> {
  let client = reqwest::blocking::Client::new();
  let res = client.post(RPC_URL)
    .body("{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"klay_getBlockByNumber\",\"params\":[\"latest\",false]}")
    .header("Content-Type", "application/json")
    .send()?;

  let block = match res.json::<RpcPayload<Block>>() {
    Ok(block) => block,
    Err(err) => return Err(err)
  };

  Ok(block)
}

fn main() -> Result<(), Error>{
  let mut status = Arc::new(std::sync::Mutex::new(Status::new()));
  let mut handles = Vec::new();

  for _ in 0..CONCURRENCY {
    let cloned_status = status.clone();

    let h = thread::spawn(move || {
      loop {
        let now = Instant::now();

        match get_latest_block() {
          Ok(block) => {
            match i64::from_str_radix(block.result.number.trim_start_matches("0x"), 16) {
              Ok(res) => {
                cloned_status.lock().unwrap().add(true, now.elapsed().as_millis(), res);
              },
              _ => {
                cloned_status.lock().unwrap().add(false, now.elapsed().as_millis(), 0);
              }
            }
          }
          Err(err) => {
            cloned_status.lock().unwrap().add(false, now.elapsed().as_millis(), 0);
          }
        }

        sleep(Duration::from_millis(10));
      }
    });

    handles.push(h);
  }

  for h in handles {
    h.join().unwrap();
  }

  Ok(())
}
