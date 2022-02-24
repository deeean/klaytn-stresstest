pub fn serialize<T: serde::Serialize>(t: &T) -> jsonrpc_core::Value {
  serde_json::to_value(t).expect("Types never fail to serialize.")
}