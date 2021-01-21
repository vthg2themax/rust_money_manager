extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sql_load_accounts_with_balances() -> String {
    let bytes = include_bytes!("sql/load_accounts_with_balances.sql");
    String::from_utf8_lossy(bytes).to_string()
  
}