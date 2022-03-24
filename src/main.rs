use std::collections::HashMap;
use std::time::{Duration, Instant};
use reqwest::StatusCode;
use threadpool::ThreadPool;

const CONCURRENCY: usize = 10;
const REQUEST: usize = 1000;
const RPC_URL: &str = "http://localhost:8551/";

struct Response {
  status: u16,
  size: u64,
  latency: u64,
}

#[derive(Default, Debug)]
struct Report {
  statuses: HashMap<u16, u32>,
  total_size: u64,
  latencies: Vec<u64>,
}

fn get_latest_block() -> Option<Response> {
  let client = reqwest::blocking::Client::new();
  let now = Instant::now();
  let try_resp = client.post(RPC_URL)
    .header("content-type", "application/json")
    .body("{\"id\":0,\"method\":\"klay_getBlockByNumber\",\"params\":[\"latest\", false]}")
    .send();

  if let Some(resp) = try_resp.ok() {
    return Some(Response {
      status: resp.status().as_u16(),
      size: resp.bytes().unwrap().len() as u64,
      latency: now.elapsed().as_nanos() as u64,
    })
  }

  None
}

fn main() {
  let pool = ThreadPool::new(CONCURRENCY);
  let (tx, rx) = std::sync::mpsc::channel();

  for _ in 0..REQUEST {
    let tx = tx.clone();

    pool.execute(move || {
      tx.send(get_latest_block());
    });
  }

  let mut report = Report::default();
  let mut success: u64 = 0;

  for resp in rx.iter().take(REQUEST) {
    match resp {
      Some(resp) => {
        if let Some(s) = report.statuses.get(&resp.status) {
          report.statuses.insert(resp.status, s + 1);
        } else {
          report.statuses.insert(resp.status, 1);
        }

        report.total_size += resp.size;
        report.latencies.push(resp.latency);
        success += 1;
      },
      None => {

      }
    }
  }

  let min_latency = report.latencies.iter().min().unwrap();
  let max_latency = report.latencies.iter().max().unwrap();
  let total_latency = report.latencies.iter().sum::<u64>();

  println!("Status: {:?}", report.statuses);
  println!("Total Bytes: {:?}", report.total_size);
  println!("Avg Latency: {:?}", Duration::from_nanos(total_latency / success));
  println!("Min Latency: {:?}", Duration::from_nanos(*min_latency));
  println!("Max Latency: {:?}", Duration::from_nanos(*max_latency));
}