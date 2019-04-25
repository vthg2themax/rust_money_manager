use rusqlite::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;
use chrono::prelude::*;
use time::Duration;

#[derive(Debug)]
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

fn convert_to_account_type(incoming_result : Result<String>) -> Result<AccountType> {
    let incoming_account_type : String = incoming_result.unwrap();
    match incoming_account_type.as_str() {        
        "ASSET" => Ok(AccountType::ASSET), 
        "BANK" =>  Ok(AccountType::BANK), 
        "CASH" =>  Ok(AccountType::CASH), 
        "CREDIT" => Ok(AccountType::CREDIT), 
        "EQUITY" => Ok(AccountType::EQUITY), 
        "EXPENSE" => Ok(AccountType::EXPENSE), 
        "INCOME" => Ok(AccountType::INCOME), 
        "LIABILITY" => Ok(AccountType::LIABILITY), 
        "RECEIVABLE" => Ok(AccountType::RECEIVABLE), 
        "ROOT" => Ok(AccountType::ROOT),
        _ => panic!(format!("The given Account Type '{0}' is not valid!",
                            incoming_account_type.as_str())),
    }
}

#[derive(Debug)]
pub struct Account {
    pub guid: GUID, //guid is the GUID for this account.
    pub name: String, //Name is the name of the account.
    pub account_type: AccountType, //Account_Type is the account type. (Ex: 'ROOT' or 'CREDIT')
    pub commodity_guid: GUID,//Commodity_Guid is the commodity guid the account uses. Ex: USD or YEN.
    pub commodity_scu: i64,//Commodity_Scu is the commodity scu. -1 by default
    pub non_std_scu: i64, //Non_Std_Scu is the non std scu. -1 by default
    pub parent_guid: GUID, //Parent_Guid is the parent of this account's GUID. null guid by default
    pub code: String, //Code is the code for this account. Blank by default
    pub description: String, //Description is the description for this account. Blank by default.
    pub hidden: bool, //Hidden is a bit field whether this account is hidden or not.
    pub placeholder: bool,//Placeholder is whether this account is a placeholder account. (1 for yes, 0 for no)
}

//_Fields: guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,
//         parent_guid,code,description,hidden,placeholder "
pub fn _fields() -> String {
    String::from(
        ["guid,name,account_type,commodity_guid,commodity_scu,non_std_scu,",
         "parent_guid,code,description,hidden,placeholder "].join("")
         )
} 

///
pub fn retrieve_active_accounts(file_path : &str) -> Result<Vec<Account>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the account fields for active account
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM accounts WHERE (hidden=0) AND (placeholder=0) AND ",
         "(NOT(account_type='ROOT')) AND (NOT(account_type='EXPENSE')) AND ",
         "(NOT(account_type='EQUITY'))",
         "AND (NOT(account_type='INCOME')) AND (NOT(name='Expenses'))"].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the accounts into a vector for returning the result
    let mut accounts : Vec<Account> = Vec::new();
    let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
        Ok( 
            Account{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    name: row.get(1)?,
                    account_type: convert_to_account_type(row.get(2))?,
                    commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
                    commodity_scu: row.get(4)?,
                    non_std_scu: row.get(5)?,
                    parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
                    code: row.get(7)?,
                    description: row.get(8)?,
                    hidden: row.get(9)?,
                    placeholder: row.get(10)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the accounts vector
    //std::result::Result<accounts_manager::Account, rusqlite::Error>    
    for row in mapped_rows {
        accounts.push(row?);
    }    

    Ok(accounts)
}

///retrieve_all_nonhidden_accounts retrieves all the non hidden accounts
pub fn retrieve_all_nonhidden_accounts(file_path : &str) -> Result<Vec<Account>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the account fields for the non hidden accounts
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM accounts WHERE (NOT(name='Root Account')) ", 
         " AND (NOT(name='Template Root')) AND hidden='0'"].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the accounts into a vector for returning the result
    let mut accounts : Vec<Account> = Vec::new();
    let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
        Ok( 
            Account{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    name: row.get(1)?,
                    account_type: convert_to_account_type(row.get(2))?,
                    commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
                    commodity_scu: row.get(4)?,
                    non_std_scu: row.get(5)?,
                    parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
                    code: row.get(7)?,
                    description: row.get(8)?,
                    hidden: row.get(9)?,
                    placeholder: row.get(10)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the accounts vector
    //std::result::Result<accounts_manager::Account, rusqlite::Error>    
    for row in mapped_rows {
        accounts.push(row?);
    }    

    Ok(accounts)
}

///retrieve_all_accounts retrieves all the accounts, except the root, and template account.
pub fn retrieve_all_accounts(file_path : &str) -> Result<Vec<Account>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the account fields for the non hidden accounts
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM accounts ", 
         "WHERE (NOT(name='Root Account')) AND (NOT(name='Template Root'))"].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the accounts into a vector for returning the result
    let mut accounts : Vec<Account> = Vec::new();
    let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
        Ok( 
            Account{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    name: row.get(1)?,
                    account_type: convert_to_account_type(row.get(2))?,
                    commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
                    commodity_scu: row.get(4)?,
                    non_std_scu: row.get(5)?,
                    parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
                    code: row.get(7)?,
                    description: row.get(8)?,
                    hidden: row.get(9)?,
                    placeholder: row.get(10)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the accounts vector
    //std::result::Result<accounts_manager::Account, rusqlite::Error>    
    for row in mapped_rows {
        accounts.push(row?);
    }    

    Ok(accounts)
}

//RetrieveAllAccountsWithTransactionsInTheLastGivenDays
pub fn retrieve_accounts_with_transactions_in_last_days(file_path : &str, given_days : i64) -> Result<Vec<Account>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    let start_date = Utc.datetime_from_str(
                        &(Utc::now() + Duration::days(-1 * given_days)).format("%Y-%m-%d 00:00:00").to_string(), 
                        "%Y-%m-%d %H:%M:%S").expect("Failed to create a start date for comparison!");
    let end_date = Utc.datetime_from_str(
                        &(Utc::now()).format("%Y-%m-%d 23:59:59").to_string(),
                        "%Y-%m-%d %H:%M:%S").expect("Failed to create an end date for comparison!");
    
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM  ", 
         "WHERE accounts.guid IN (",
         "    SELECT splits.account_guid FROM splits WHERE splits.tx_guid IN (",
         "        SELECT t.guid ",
         "        FROM transactions as t",
         "        WHERE datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'||",
         "                       substr(t.post_date,7,2)||' '||substr(t.post_date,9,2)||':'||",
         "                       substr(t.post_date,11,2)||':'||substr(t.post_date,13,2)) >= ",
         "              datetime('", &start_date.format("%Y-%m-%d %H:%M:%S").to_string(), "')",
         "        AND ",
         "              datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'||",
         "                       substr(t.post_date,7,2)||' '||substr(t.post_date,9,2)||':'||",
         "                       substr(t.post_date,11,2)||':'||substr(t.post_date,13,2)) <= ",
         "              datetime('", &end_date.format("%Y-%m-%d %H:%M:%S").to_string(), "')",
         "        ) AND (NOT(accounts.name='Root Account')) AND (NOT(accounts.name='Template Root'))",
         ""].join(""));
    
    let mut stmt = conn.prepare(&sql)?;
    //Get all the accounts into a vector for returning the result
    let mut accounts : Vec<Account> = Vec::new();
    let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
        Ok( 
            Account{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    name: row.get(1)?,
                    account_type: convert_to_account_type(row.get(2))?,
                    commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
                    commodity_scu: row.get(4)?,
                    non_std_scu: row.get(5)?,
                    parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
                    code: row.get(7)?,
                    description: row.get(8)?,
                    hidden: row.get(9)?,
                    placeholder: row.get(10)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the accounts vector
    //std::result::Result<accounts_manager::Account, rusqlite::Error>    
    for row in mapped_rows {
        accounts.push(row?);
    }    

    Ok(accounts)
}

///retrieve_by_guid retrieves an account by it's guid.
pub fn retrieve_by_guid(file_path : &str, incoming_account_guid : GUID) -> Result<Vec<Account>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the account fields for the non hidden accounts
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM accounts ", 
         "WHERE guid=@account_guid"].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the accounts into a vector for returning the result
    let mut accounts : Vec<Account> = Vec::new();
    let mapped_rows = stmt.query_map_named(
        named_params!{"@account_guid": dhu::convert_guid_to_sqlite_string(incoming_account_guid)? }, |row| 
        Ok( 
            Account{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    name: row.get(1)?,
                    account_type: convert_to_account_type(row.get(2))?,
                    commodity_guid: dhu::convert_string_result_to_guid(row.get(3))?,
                    commodity_scu: row.get(4)?,
                    non_std_scu: row.get(5)?,
                    parent_guid: dhu::convert_string_result_to_guid(row.get(6))?,
                    code: row.get(7)?,
                    description: row.get(8)?,
                    hidden: row.get(9)?,
                    placeholder: row.get(10)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the accounts vector
    //std::result::Result<accounts_manager::Account, rusqlite::Error>    
    for row in mapped_rows {
        accounts.push(row?);
    }    

    Ok(accounts)
}