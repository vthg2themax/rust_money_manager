use crate::{
    accounts_manager, books_manager, commodities_manager, database_helper_utility, 
    sciter_helper_utility, versions_manager, lots_manager, slots_manager
    };

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use chrono::prelude::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;

pub fn get_active_accounts_with_balances(file_path : &str) -> Result<String> {    
    let mut return_value = String::from("");
    let accounts = accounts_manager::retrieve_active_accounts_with_balances(file_path)
                        .expect(&"Error Finding Accounts!");
        
    for account in accounts {
        return_value = [return_value,
                        account.name,
                        account.account_type.to_string(),
                        account.description,
                        (account.tags.get("balance").expect("No balance tag!")).to_string(),
                        String::from(" ")].join(", ");
        return_value = [return_value,
                        String::from("<br>")].join("");
        
    }

    Ok(return_value)
}