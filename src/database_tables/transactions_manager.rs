use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::prelude::*;
//use crate::database_helper_utility as dhu;
//use chrono::prelude::*;
//use time::Duration;
use std::fmt;
use serde_repr::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountType {
    ASSET, 
    BANK, 
    CASH, 
    CREDIT, 
    EQUITY, 
    EXPENSE, 
    INCOME, 
    LIABILITY, 
    RECEIVABLE, 
    ROOT,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        write!(fmt, "{:?}", self)
    }
}


// fn convert_to_account_type(incoming_result : Result<String>) -> Result<AccountType> {
//     let incoming_account_type : String = incoming_result.unwrap();
//     match incoming_account_type.as_str() {        
//         "ASSET" => Ok(AccountType::ASSET), 
//         "BANK" =>  Ok(AccountType::BANK), 
//         "CASH" =>  Ok(AccountType::CASH), 
//         "CREDIT" => Ok(AccountType::CREDIT), 
//         "EQUITY" => Ok(AccountType::EQUITY), 
//         "EXPENSE" => Ok(AccountType::EXPENSE), 
//         "INCOME" => Ok(AccountType::INCOME), 
//         "LIABILITY" => Ok(AccountType::LIABILITY), 
//         "RECEIVABLE" => Ok(AccountType::RECEIVABLE), 
//         "ROOT" => Ok(AccountType::ROOT),
//         _ => panic!(format!("The given Account Type '{0}' is not valid!",
//                             incoming_account_type.as_str())),
//     }
// }

// impl Account {
//     pub fn new() {

//     }
// }



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub guid: Uuid, //guid is the GUID for this transaction
    pub currency_guid: Uuid,//currency_Guid is the commodity guid the account uses. Ex: USD or YEN.
    pub num : String,//Num is the invoice.id that this transaction belongs to. 
    pub post_date : String, //post_date is the date this transaction is posted. (Ex: '20120801040000' is 'Aug 1 2012')
    pub enter_date : String, //enter_date is the date this transaction was entered. (Ex: '20120801040000' is 'Aug 1 2012')
    pub description : String, //description is the description for this transaction.
}

/// TransactionWithOtherHalfOfSplitInformation holds one half of the split information along
/// with the Transaction data for this transaction guid.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionWithSplitInformation {
    pub excluded_account_guid : Uuid, //excluded_account_guid is the account_guid that this information was for. Usually the account currently displaying transactions for.
    pub excluded_account_name : String, //excluded_account_name is the account_name that this information was for. Usuallly the account name currently displaying this transaction.
    pub guid: Uuid, //guid is the GUID for this transaction
    pub currency_guid: Uuid,//currency_Guid is the commodity guid the account uses. Ex: USD or YEN.
    pub num : String,//Num is the invoice.id that this transaction belongs to. 
    pub post_date : String, //post_date is the date this transaction is posted. (Ex: '20120801040000' is 'Aug 1 2012')
    pub enter_date : String, //enter_date is the date this transaction was entered. (Ex: '20120801040000' is 'Aug 1 2012')
    pub description : String, //description is the description for this transaction.
    pub value_num : i64,//value_num is the numerator for the transaction
    pub value_denom : i64,//value_denom is the denominator for the transaction
    pub account_name : String,//account_name is the account.name for this transaction
    pub account_guid : Uuid,//account_guid is the account.guid for this transaction
}

//_Fields: guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,
//         parent_guid,code,description,hidden,placeholder "
pub fn _fields() -> String {
    String::from(
        ["guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,",
         "parent_guid,code,description,hidden,placeholder "].join("")
         )
} 

// ///
// pub fn retrieve_active_accounts(file_path : &str) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for active account
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts WHERE (hidden=0) AND (placeholder=0) AND ",
//          "(NOT(account_type='ROOT')) AND (NOT(account_type='EXPENSE')) AND ",
//          "(NOT(account_type='EQUITY'))",
//          "AND (NOT(account_type='INCOME')) AND (NOT(name='Expenses'))"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// /// retrieve_active_accounts_with_balances retrieves a list of active accounts
// /// with the tags value containing the current balance of the account.
// pub fn retrieve_active_accounts_with_balances(file_path : &str) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for active account
//     let sql : String = String::from(
//         ["SELECT ",&_fields(),", ",
//          "(SELECT CASE (SELECT COUNT(DISTINCT(splits.value_denom)) ",
//                        "FROM splits ",
//                        "WHERE splits.account_guid=accounts.guid)",
//          "   WHEN 1 THEN (",
//          "       (SELECT SUM(splits.value_num *1.0) FROM splits WHERE splits.account_guid=accounts.guid) /",
//          "       (SELECT splits.value_denom FROM splits WHERE splits.account_guid=accounts.guid)",
//          "   )",
//          "   ELSE \"More Than A Single Denominator, Please Contact Support!\"",
//          "END",
//          ") as balance ",
//          "FROM accounts WHERE (hidden=0) AND (placeholder=0) AND ",
//          "(NOT(account_type='ROOT')) AND (NOT(account_type='EXPENSE')) AND ",
//          "(NOT(account_type='EQUITY'))",
//          "AND (NOT(account_type='INCOME')) AND (NOT(name='Expenses'))"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: {
//                         let mut tags = HashMap::new();
//                         let balance : f64 = row.get(11)?;
//                         tags.insert(String::from("balance"), format!("${:.2}",balance));
//                         tags
//                     },
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// ///retrieve_all_nonhidden_accounts retrieves all the non hidden accounts
// pub fn retrieve_all_nonhidden_accounts(file_path : &str) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts WHERE (NOT(name='Root Account')) ", 
//          " AND (NOT(name='Template Root')) AND hidden='0'"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// ///retrieve_all_accounts retrieves all the accounts, except the root, and template account.
// pub fn retrieve_all_accounts(file_path : &str) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts ", 
//          "WHERE (NOT(name='Root Account')) AND (NOT(name='Template Root'))"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

//RetrieveAllAccountsWithTransactionsInTheLastGivenDays
// pub fn retrieve_accounts_with_transactions_in_last_days(file_path : &str, given_days : i64) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     let start_date = Utc.datetime_from_str(
//                         &(Utc::now() + Duration::days(-1 * given_days)).format("%Y-%m-%d 00:00:00").to_string(), 
//                         "%Y-%m-%d %H:%M:%S").expect("Failed to create a start date for comparison!");
//     let end_date = Utc.datetime_from_str(
//                         &(Utc::now()).format("%Y-%m-%d 23:59:59").to_string(),
//                         "%Y-%m-%d %H:%M:%S").expect("Failed to create an end date for comparison!");
    
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM  ", 
//          "WHERE accounts.guid IN (",
//          "    SELECT splits.account_guid FROM splits WHERE splits.tx_guid IN (",
//          "        SELECT t.guid ",
//          "        FROM transactions as t",
//          "        WHERE datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'||",
//          "                       substr(t.post_date,7,2)||' '||substr(t.post_date,9,2)||':'||",
//          "                       substr(t.post_date,11,2)||':'||substr(t.post_date,13,2)) >= ",
//          "              datetime('", &start_date.format("%Y-%m-%d %H:%M:%S").to_string(), "')",
//          "        AND ",
//          "              datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'||",
//          "                       substr(t.post_date,7,2)||' '||substr(t.post_date,9,2)||':'||",
//          "                       substr(t.post_date,11,2)||':'||substr(t.post_date,13,2)) <= ",
//          "              datetime('", &end_date.format("%Y-%m-%d %H:%M:%S").to_string(), "')",
//          "        ) AND (NOT(accounts.name='Root Account')) AND (NOT(accounts.name='Template Root'))",
//          ""].join(""));
    
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// ///retrieve_by_guid retrieves an account by it's guid.
// pub fn retrieve_by_guid(file_path : &str, incoming_account_guid : GUID) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts ", 
//          "WHERE guid=@account_guid"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@account_guid": dhu::convert_guid_to_sqlite_string(incoming_account_guid)? }, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// //RetrieveAccountForAccountType retrieves the account for the given type. (Ex: ASSET, or LIABILITY)
// pub fn retrieve_account_for_account_type(file_path : &str, incoming_account_type : AccountType) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts ", 
//          "WHERE account_type=@account_type AND ",
//          "      parent_guid NOT IN (",
//          "                          SELECT guid ",
//          "                          FROM accounts ",
//          "                          WHERE account_type=@account_type",
//          "                         )"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@account_type": incoming_account_type.to_string() }, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     Ok(accounts)
// }

// //retrieve_top_account_by_name retrieves the top account for a given name.
// // There must only be 1 account found for this, or it fails.
// pub fn retrieve_top_account_by_name(file_path : &str, incoming_account_name : String) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts ",
//          "WHERE name=@incoming_account_name "].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@incoming_account_name": incoming_account_name }, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     if accounts.len() != 1 {
//         let error_message : String = ["There were ", &accounts.len().to_string(),
//                                       " accounts found for this",
//                                       " name: '", &incoming_account_name,
//                                       "'. Please check your entries",
//                                       " and try again!"].join("");        
                                              
//         return Err(rusqlite::Error::InvalidParameterName(error_message));
//     } 

//     Ok(accounts)
// }

// //retrieve_top_account_by_name_starting_with retrieves the top account for a given name starting with.
// // There must only be 1 account found for this, or it fails.
// pub fn retrieve_top_account_by_name_starting_with(file_path : &str, incoming_account_name : String) -> Result<Vec<Account>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the account fields for the non hidden accounts
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM accounts ",
//          "WHERE name LIKE @incoming_account_name "].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the accounts into a vector for returning the result
//     let mut accounts : Vec<Account> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@incoming_account_name": 
//                       [incoming_account_name.clone(), "%".to_string()].join("") }, |row| 
//         Ok( 
//             Account{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     name: row.get(1)?,
//                     account_type: convert_to_account_type(row.get(2))?,
//                     commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
//                     commodity_scu: row.get(4)?,
//                     non_std_scu: row.get(5)?,
//                     parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
//                     code: row.get(7)?,
//                     description: row.get(8)?,
//                     hidden: row.get(9)?,
//                     placeholder: row.get(10)?,
//                     tags: HashMap::new(),
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the accounts vector
//     //std::result::Result<accounts_manager::Account, rusqlite::Error>    
//     for row in mapped_rows {
//         accounts.push(row?);
//     }    

//     if accounts.len() != 1 {
//         let error_message : String = ["There were ", &accounts.len().to_string(),
//                                       " accounts found for this",
//                                       " name: '", &incoming_account_name,
//                                       "'. Please check your entries",
//                                       " and try again!"].join("");        
                                              
//         return Err(rusqlite::Error::InvalidParameterName(error_message));
//     } 

//     Ok(accounts)
// }

// pub fn delete_existing(file_path : &str, incoming_account_guid : GUID) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["DELETE FROM accounts ",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_account_guid)?,            
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

// pub fn update_existing(file_path : &str, incoming_account : &Account) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["UPDATE accounts SET ",
//         "                     name=@name, account_type=@account_type, ",
//         "                     commodity_guid=@commodity_guid, ",
//         "                     commodity_scu=@commodity_scu,",
//         "                     non_std_scu=@non_std_scu, parent_guid=@parent_guid, ",
//         "                     code=@code, description=@description,",
//         "                     hidden=@hidden, placeholder=@placeholder ",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_account.guid)?,
//             "@name" : incoming_account.name.to_string(),
//             "@account_type" : incoming_account.account_type.to_string(),
//             "@commodity_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_account.commodity_guid)?,
//             "@commodity_scu" : incoming_account.commodity_scu,
//             "@non_std_scu" : incoming_account.non_std_scu,
//             "@parent_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_account.parent_guid)?,
//             "@code" : incoming_account.code,
//             "@description" : incoming_account.description,
//             "@hidden" : if incoming_account.hidden == false {0} else {1},
//             "@placeholder" : if incoming_account.placeholder == false {0} else {1},
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

// pub fn save_new(file_path : &str, incoming_account : &Account) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["INSERT INTO accounts (", &_fields(),") values (",
//         "@guid,@name,@account_type,@commodity_guid,@commodity_scu,@non_std_scu,",
//          "@parent_guid,@code,@description,@hidden,@placeholder )"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_account.guid)?,
//             "@name" : incoming_account.name.to_string(),
//             "@account_type" : incoming_account.account_type.to_string(),
//             "@commodity_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_account.commodity_guid)?,
//             "@commodity_scu" : incoming_account.commodity_scu,
//             "@non_std_scu" : incoming_account.non_std_scu,
//             "@parent_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_account.parent_guid)?,
//             "@code" : incoming_account.code,
//             "@description" : incoming_account.description,
//             "@hidden" : if incoming_account.hidden == false {0} else {1},
//             "@placeholder" : if incoming_account.placeholder == false {0} else {1},
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }