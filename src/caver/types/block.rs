use ethereum_types::{U64, U256, H256};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use crate::caver::types::bytes::Bytes;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Block<TX> {
  pub blockscore: U256,

  #[serde(rename = "extraData")]
  pub extra_data: String,

  #[serde(rename = "gasUsed")]
  pub gas_used: U64,

  #[serde(rename = "governanceData")]
  pub governance_data: String,

  pub hash: H256,

  #[serde(rename = "logsBloom")]
  pub logs_bloom: String,

  pub number: U64,

  #[serde(rename = "parentHash")]
  pub parent_hash: H256,

  #[serde(rename = "receiptsRoot")]
  pub receipts_root: H256,

  pub reward: U256,

  pub size: U64,

  #[serde(rename = "stateRoot")]
  pub state_root: String,

  pub timestamp: U64,

  #[serde(rename = "timestampFoS")]
  pub timestamp_fo_s: U64,

  #[serde(rename = "totalBlockScore")]
  pub total_block_score: U256,

  pub transactions: Vec<TX>,

  #[serde(rename = "transactionsRoot")]
  pub transactions_root: H256,

  #[serde(rename = "voteData")]
  pub vote_data: String,
}

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
  where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
  let option = Option::deserialize(deserializer)?;
  Ok(option.unwrap_or_default())
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockNumber {
  Latest,
  Earliest,
  Pending,
  Number(U64),
}

impl<T: Into<U64>> From<T> for BlockNumber {
  fn from(num: T) -> Self {
    BlockNumber::Number(num.into())
  }
}

impl Serialize for BlockNumber {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
  {
    match *self {
      BlockNumber::Number(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
      BlockNumber::Latest => serializer.serialize_str("latest"),
      BlockNumber::Earliest => serializer.serialize_str("earliest"),
      BlockNumber::Pending => serializer.serialize_str("pending"),
    }
  }
}

impl<'a> Deserialize<'a> for BlockNumber {
  fn deserialize<D>(deserializer: D) -> Result<BlockNumber, D::Error>
    where
      D: Deserializer<'a>,
  {
    let value = String::deserialize(deserializer)?;
    match value.as_str() {
      "latest" => Ok(BlockNumber::Latest),
      "earliest" => Ok(BlockNumber::Earliest),
      "pending" => Ok(BlockNumber::Pending),
      _ if value.starts_with("0x") => U64::from_str_radix(&value[2..], 16)
        .map(BlockNumber::Number)
        .map_err(|e| D::Error::custom(format!("invalid block number: {}", e))),
      _ => Err(D::Error::custom("invalid block number: missing 0x prefix".to_string())),
    }
  }
}