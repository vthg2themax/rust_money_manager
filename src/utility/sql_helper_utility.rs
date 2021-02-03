
pub fn sql_load_accounts_with_balances() -> String {
    let bytes = include_bytes!("../sql/load_accounts_with_balances.sql");
    String::from_utf8_lossy(bytes).to_string()
  
}

pub fn sql_load_transactions_for_account() -> String {
    let bytes = include_bytes!("../sql/load_transactions_for_account.sql");
    String::from_utf8_lossy(bytes).to_string()
}