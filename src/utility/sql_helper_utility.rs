///sql_helper_utility is all the sql files in use by the program.
/// There should only be complex ones here, because simple ones don't need their own file.

/// load_splits_for_last_30_day_report loads the last 30 days of splits.
/// The 3 parameters are start_date, end_date, and account_type.
pub fn load_splits_for_last_30_day_report() -> String {
    let bytes = include_bytes!("../sql/load_splits_for_last_30_day_report.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_slots_for_name gives you all the slots with a given name. 
pub fn load_slots_for_name() -> String {
    let bytes = include_bytes!("../sql/load_slots_for_name.sql");
    String::from_utf8_lossy(bytes).to_string()
}

pub fn load_slots_for_name_and_string_val() -> String {
    let bytes = include_bytes!("../sql/load_slots_for_name_and_string_val.sql");
    String::from_utf8_lossy(bytes).to_string()
}

pub fn load_accounts_with_balances() -> String {
    let bytes = include_bytes!("../sql/load_accounts_with_balances.sql");
    String::from_utf8_lossy(bytes).to_string()
  
}

/// load_transactions_for_account loads all the transactions for an account.
/// You will need to pass the account_guid 4 times.
pub fn load_transactions_for_account() -> String {
    let bytes = include_bytes!("../sql/load_transactions_for_account.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_commodity_for_guid() loads a commodity for a given guid.
pub fn load_commodity_for_guid() -> String {
    let bytes = include_bytes!("../sql/load_commodity_for_guid.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_account_with_balance_for_date_and_guid loads the account with balance based on 
/// the date the account was opened until the given date, for the account with given guid.
pub fn load_account_with_balance_for_date_and_guid() -> String {
    let bytes = include_bytes!("../sql/load_account_with_balance_for_date_and_guid.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_account_with_balance_for_guid loads the account with balance for the account with
/// the given guid.
pub fn load_account_with_balance_for_guid() -> String {
    let bytes = include_bytes!("../sql/load_account_with_balance_for_guid.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_transaction_for_account_between_dates loads the transactions based on the beginning
/// dates give. You should have 4 guids for the account, the beginning date, then the end after it.
pub fn load_transactions_for_account_between_dates() -> String {
    let bytes = include_bytes!("../sql/load_transactions_for_account_between_dates.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_all_accounts_except_root_and_template loads all the accounts >100 of them except the root
/// and the template ones.
pub fn load_all_accounts_except_root_and_template() -> String {
    let bytes = include_bytes!("../sql/load_all_accounts_except_root_and_template.sql");
    String::from_utf8_lossy(bytes).to_string()
}

/// load_transaction_for_account_guid_and_description loads the last other half of the transaction
/// for a particular account guid, and the description. Should have 3 account guids, then a description,
/// then another account guid to make this work.
pub fn load_transaction_for_account_guid_and_description() -> String {
    let bytes = include_bytes!("../sql/load_transaction_for_account_guid_and_description.sql");
    String::from_utf8_lossy(bytes).to_string()
}
