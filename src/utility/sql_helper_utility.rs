
pub fn sql_load_accounts_with_balances() -> String {
    let bytes = include_bytes!("../sql/load_accounts_with_balances.sql");
    String::from_utf8_lossy(bytes).to_string()
  
}

pub fn sql_load_transactions_for_account() -> String {
    let bytes = include_bytes!("../sql/load_transactions_for_account.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// sql_load_account_with_balanace loads the account with balance based on the date the account
/// was opened until the given date, for the account with given guid.
pub fn sql_load_acount_with_balance_for_date_and_guid() -> String {
    let bytes = include_bytes!("../sql/load_account_with_balance_for_date_and_guid.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// sql_load_transaction_for_account_between_dates loads the transactions based on the beginning
/// dates give. You should have 4 guids for the account, the beginning date, then the end after it.
pub fn sql_load_transactions_for_account_between_dates() -> String {
    let bytes = include_bytes!("../sql/load_transactions_for_account_between_dates.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// sql_load_all_accounts_except_root_and_template loads all the accounts >100 of them except the root
/// and the template ones.
pub fn sql_load_all_accounts_except_root_and_template() -> String {
    let bytes = include_bytes!("../sql/load_all_accounts_except_root_and_template.sql");
    String::from_utf8_lossy(bytes).to_string()
}
