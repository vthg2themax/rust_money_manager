extern crate guid_create;
extern crate chrono;

use guid_create::GUID;
use chrono::prelude::*;
use rusqlite::*;
use rusqlite::types::*;
use regex::Regex;

const FORMAT_STRING : &str = "%Y%m%d%H%M%S";

pub fn _null_guid() -> GUID { 
    return GUID::parse("00000000-0000-0000-0000-000000000000").unwrap();
}


///convert_string_to_guid converts the result to a string, if possible.
pub fn convert_string_to_guid(incoming_string : String) -> Result<GUID> {
    let mut incoming_string = incoming_string;
    //If it's 32 characters, it needs dashes
    if incoming_string.chars().count() == 32 {
        let part1 = &incoming_string[0..8];
        let part2 = &incoming_string[8..12];
        let part3 = &incoming_string[12..16];
        let part4 = &incoming_string[16..20];
        let part5 = &incoming_string[20..32];
        incoming_string = [part1,part2,part3,part4,part5].join("-");
    }

    return match GUID::parse(&incoming_string) {
        Ok(guid) => Ok(guid),
        Err(e) => panic!(format!("{0}",e)),
    };

}

///convert_string_result_to_guid converts the result to a guid, if possible.
pub fn convert_string_result_to_guid(incoming_result : Result<String>) -> Result<GUID> {
    //Carefully, unwrap the string, which could be a null
    if incoming_result.is_err() {
        return Ok(_null_guid());
    }
    let mut incoming_string = incoming_result.unwrap();
    //If it's 32 characters, it needs dashes
    if incoming_string.chars().count() == 32 {
        let part1 = &incoming_string[0..8];
        let part2 = &incoming_string[8..12];
        let part3 = &incoming_string[12..16];
        let part4 = &incoming_string[16..20];
        let part5 = &incoming_string[20..32];
        incoming_string = [part1,part2,part3,part4,part5].join("-");
    }

    return match GUID::parse(&incoming_string) {
        Ok(guid) => Ok(guid),
        Err(e) => panic!(format!("{0}",e)),
    };

}

///convert_guid_to_sqlite_string converts a guid to an sqlite string if possible, 
/// like so: f737a4904dac6736c7d8fe7b765ee354
pub fn convert_guid_to_sqlite_string(incoming_guid : GUID) -> Result<String> {    
    let mut incoming_guid = incoming_guid.to_string().to_lowercase();
    //If it's 36 characters, we chop off the dashes
    if incoming_guid.chars().count() == 36 {
        incoming_guid = incoming_guid.replace("-","");        
    }

    Ok(incoming_guid)

}

///convert_guid_to_sqlite_parameter converts a guid to an sqlite string if possible, 
/// like so: f737a4904dac6736c7d8fe7b765ee354 or NULL
pub fn convert_guid_to_sqlite_parameter(incoming_guid : GUID) -> Result<Option<String>> {    
    //If it's a null GUID we want to return a null value
    if incoming_guid == _null_guid() {
        return Ok(None);
    }
    //Otherwise attempt to convert the value to a sqlite guid string
    return Ok(Some(convert_guid_to_sqlite_string(incoming_guid)?));

}

///convert_date_to_string_format converts a date to a string format that works for
/// the sqlite database.
pub fn convert_date_to_string_format(incoming_date : chrono::NaiveDateTime ) -> String {    
    
    let return_value : String = String::from(incoming_date.format(&FORMAT_STRING).to_string());

    return return_value;
    
}

///convert_string_to_date_format attempts to convert a string to the sqlite
/// database datetime format. 
pub fn convert_string_to_date_format(incoming_date : &mut chrono::NaiveDateTime,
                                     incoming_string: &str) -> bool {
    
    match NaiveDateTime::parse_from_str(incoming_string, FORMAT_STRING) {
        Ok(good_value) => {
            *incoming_date = good_value;
            return true;
        },
        Err(_) => {
            *incoming_date = NaiveDate::from_ymd(0, 1, 1).and_hms(0,0,0);
            return false;
        }
    }    

}

/// MakeBackupCopiesOfFile makes backups of the file and saves copies
/// of it. It makes up to x number of copies!
/// 
pub fn make_backup_copies_of_file(incoming_file_path : &std::path::Path,
                                  number_of_copies: u8) -> std::result::Result<bool, String> {

    let file_information = std::fs::metadata(incoming_file_path);

    //Ensure this is in a directory we can reach
    if file_information.is_err() == true {
        let error_message : String = 
                            format!("The given directory is not a valid result. {:#?}", 
                                    file_information.err());
        return Err(error_message);
    }
    //Ensure this is a not directory
    if file_information.unwrap().is_dir() == true {
        return Err(String::from(
            "The given path is a directory! We do not make copies of directories.")
        );
    }

    //Get the file name for this file without the (0).bak piece
    let base_file_name: String = String::from(incoming_file_path.file_name()
                                              .expect("Invalid File Name!")
                                              .to_str()
                                              .expect("Invalid File Name!"));
    //Get the parent directory for this file as an easy to use string
    let directory_file_path: String = String::from(
                                        incoming_file_path.parent()
                                        .expect("bad directory file path").to_str()
                                        .expect(&["Directory File Path could not ",
                                                  "be converted to string."].join("")));

    //Get all the files that end with ([0-9]).bak files in the directory
    let re = Regex::new(r"^.*[(](\d+)[)][.][Bb][Aa][Kk]$").unwrap();
    let mut files_that_match : Vec<String> = Vec::new();
    //Get the other files in the directory
    let files = std::fs::read_dir(&directory_file_path)
                                .expect("Failed To Read Directory!");
    
    for file in files {
        let filename : String = file.unwrap().path().file_name().unwrap()
                                    .to_str().unwrap().into();
        if re.is_match(&filename) {
            files_that_match.push(filename.clone());
        }
    }

    //Lets do like a *(0).bak file filename schema 
    //The larger the number, the older the file, we set the date modified on .bak 
    //files to when the bak file was created at
    //If there's not a *(0).bak file name, then we need to create it
    if files_that_match.contains(&[&base_file_name,"(0).bak"].join("")) == false {
        //Copy the original file to original file + "(0).bak"
        std::fs::copy(incoming_file_path, 
                      &[incoming_file_path.to_str().expect("Invalid Path!"), 
                        "(0).bak"].join("")).expect("Failed To Copy The (0).bak file.");
        //Set the last write time to now (Cannot Do with RUST YET! )
        return Ok(true);
    }

    //Go through, and wipe out backup files that are more than the requested amount, or
    //will be more than the backup amount. Backup files are created: (0).bak -> (X).bak
    if files_that_match.len()  >= (number_of_copies as usize) {
        //Delete all older .bak files greater than number_of_copies
        for file in &mut files_that_match {
            //Attempt to get the filename number to check against
            let backup_number : u8 = re.captures_iter(&file).next()
                                    .expect("Backup Number Not Found!")[1]
                                    .parse::<u8>()
                                    .expect("Backup Number Not Actually Number!");
                       
            println!("Backup Number is: '{:#?}' for file '{1}'.",
                    backup_number, file.as_str());
            //If the backup file number is large enough, we delete the file
            if backup_number >= number_of_copies {
                let file_path = std::path::Path::new(incoming_file_path.parent()
                                    .expect("Invalid File Path!")
                                    .to_str().expect("Invalid File Path!")
                                    ).join(file);

                match std::fs::remove_file(&file_path) {
                    Ok(_) => {println!{"Deleted file: '{:#?}'", &file_path}},
                    Err(e) => {println!("{0}",e);},
                }
            }
            //Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
            //let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
            //assert!(re.is_match("2014-01-01"));
            //file.chars().position(|c| c == 'g').unwrap()
        }

        //Now that we have deleted some files, we need to rescan the files that match
        files_that_match.clear();
        let files = std::fs::read_dir(&directory_file_path).expect("Failed To Read Directory!");
        for file in files {
            let filename : String = file.unwrap().path().file_name().unwrap().to_str().unwrap().into();
            if re.is_match(&filename) {
                files_that_match.push(filename.clone());
            }
        }
    }    

    //Sort the files by backup number descending
    files_that_match.sort_by(|a, b| 
        {   let this_filename = a;
            let next_filename = b;
            let this_number : u8 = re.captures_iter(&this_filename).next()
                                .expect("Backup Number Not Found!")[1]
                                .parse::<u8>()
                                .expect("Backup Number Not Actually Number!");
            let next_number : u8 = re.captures_iter(&next_filename).next()
                                .expect("Backup Number Not Found!")[1]
                                .parse::<u8>()
                                .expect("Backup Number Not Actually Number!");
            next_number.cmp(&this_number)
        }
    );

    //Now we can move the files along higher to lower
    // Lets say we have the following files: (2).bak, (1).bak, (0).bak
    // We should get each of the backup numbers, such as the oldest file (2).bak,
    // and then rename it to (X+1).bak, all the way down until we get to (0).bak, 
    // (which is renamed to (1).bak). At this point, we simply create the (0).bak file from the curent file.    
    for file in &mut files_that_match {
        //Attempt to get the filename number to check against
        let backup_number : u8 = re.captures_iter(&file).next()
                                .expect("Backup Number Not Found!")[1]
                                .parse::<u8>()
                                .expect("Backup Number Not Actually Number!");
                    
        println!("Backup Number is: '{:#?}' for file '{1}'.", backup_number, &file.as_str());

        //rename the file X+1
        let old_file_path = std::path::Path::new(&directory_file_path).join(&file);
        let new_file_path = std::path::Path::new(&directory_file_path)
                                .join(
                                    &[&base_file_name,"(",&(backup_number+1).to_string(),").bak"].join("")
                                );
        match std::fs::rename(&old_file_path, &new_file_path) {
            Ok(_) => {println!{"Renamed file: '{:#?}' to '{:#?}'.", &old_file_path, &new_file_path}},
            Err(e) => {println!("{0}",e);},
        }

    }
    
    //Finally we copy the current file to (0).bak to complete the backup process
    std::fs::copy(incoming_file_path, &[incoming_file_path.to_str().expect("Invalid Path!"), "(0).bak"].join(""))
            .expect("Failed To Copy The (0).bak file.");

    Ok(true)
    
}

#[cfg(test)]
mod tests {
    //use super::*;

    // #[test]
    // fn this_test_will_pass() {
    //     make_backup_copies_of_file(incoming_file_path: &std::path::Path, number_of_copies: u8)
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10, value);
    // }

    #[test]
    fn this_test_will_fail() {
        let value = 8;
        assert_eq!(5, value);
    }
}

//
//    ''' <summary>
//    ''' CreateNewGnuCashFileAndGetConnectionString creates a new file in the given location, and returns
//    ''' the connection string if successful.
//    ''' </summary>
//    ''' <param name="connectionString">The connection String to fill in.</param>
//    ''' <param name="fileLocation">The file location to create this new file at.</param>
//    ''' 
//    ''' <returns>TRUE if success, FALSE otherwise.</returns>
//    Public Function CreateNewGnuCashFileAndGetConnectionString(ByRef connectionString As String,
//                                                               fileLocation As String, fileName As String) As Boolean
//        Dim returnValue As Boolean = False
//
//        Try
//            SQLiteConnection.CreateFile(fileLocation & "\" & fileName & ".gnucash")
//            Dim SQLconnect As New SqliteConnection()
//            Dim SQLcommand As SQLiteCommand
//            SQLconnect.ConnectionString = "Data Source=" & fileLocation & "\" & fileName & ".gnucash"
//            SQLconnect.Open()
//            SQLcommand = SQLconnect.CreateCommand
//            SQLcommand.CommandText = " BEGIN TRANSACTION;" &
//                "CREATE TABLE accounts (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, account_type text(2048) Not NULL, commodity_guid text(32), commodity_scu Integer Not NULL, non_std_scu Integer Not NULL, parent_guid text(32), code text(2048), description text(2048), hidden Integer, placeholder Integer);" &
//                "CREATE TABLE billterms (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, description text(2048) Not NULL, refcount Integer Not NULL, invisible Integer Not NULL, parent text(32), type text(2048) Not NULL, duedays Integer, discountdays Integer, discount_num bigint, discount_denom bigint, cutoff Integer);" &
//                "CREATE TABLE books (guid text(32) PRIMARY KEY Not NULL, root_account_guid text(32) Not NULL, root_template_guid text(32) Not NULL);" &
//                "CREATE TABLE budget_amounts (id Integer PRIMARY KEY AUTOINCREMENT Not NULL, budget_guid text(32) Not NULL, account_guid text(32) Not NULL, period_num Integer Not NULL, amount_num bigint Not NULL, amount_denom bigint Not NULL);" &
//                "CREATE TABLE budgets (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, description text(2048), num_periods Integer Not NULL);" &
//                "CREATE TABLE commodities (guid text(32) PRIMARY KEY Not NULL, Namespace text(2048) Not NULL, mnemonic text(2048) Not NULL, fullname text(2048), cusip text(2048), fraction Integer Not NULL, quote_flag Integer Not NULL, quote_source text(2048), quote_tz text(2048));" &
//                "CREATE TABLE customers (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, id text(2048) Not NULL, notes text(2048) Not NULL, active Integer Not NULL, discount_num bigint Not NULL, discount_denom bigint Not NULL, credit_num bigint Not NULL, credit_denom bigint Not NULL, currency text(32) Not NULL, tax_override Integer Not NULL, addr_name text(1024), addr_addr1 text(1024), addr_addr2 text(1024), addr_addr3 text(1024), addr_addr4 text(1024), addr_phone text(128), addr_fax text(128), addr_email text(256), shipaddr_name text(1024), shipaddr_addr1 text(1024), shipaddr_addr2 text(1024), shipaddr_addr3 text(1024), shipaddr_addr4 text(1024), shipaddr_phone text(128), shipaddr_fax text(128), shipaddr_email text(256), terms text(32), tax_included Integer, taxtable text(32));" &
//                "CREATE TABLE employees (guid text(32) PRIMARY KEY Not NULL, username text(2048) Not NULL, id text(2048) Not NULL, language text(2048) Not NULL, acl text(2048) Not NULL, active Integer Not NULL, currency text(32) Not NULL, ccard_guid text(32), workday_num bigint Not NULL, workday_denom bigint Not NULL, rate_num bigint Not NULL, rate_denom bigint Not NULL, addr_name text(1024), addr_addr1 text(1024), addr_addr2 text(1024), addr_addr3 text(1024), addr_addr4 text(1024), addr_phone text(128), addr_fax text(128), addr_email text(256));" &
//                "CREATE TABLE entries (guid text(32) PRIMARY KEY Not NULL, Date text(14) Not NULL, date_entered text(14), description text(2048), action text(2048), notes text(2048), quantity_num bigint, quantity_denom bigint, i_acct text(32), i_price_num bigint, i_price_denom bigint, i_discount_num bigint, i_discount_denom bigint, invoice text(32), i_disc_type text(2048), i_disc_how text(2048), i_taxable Integer, i_taxincluded Integer, i_taxtable text(32), b_acct text(32), b_price_num bigint, b_price_denom bigint, bill text(32), b_taxable Integer, b_taxincluded Integer, b_taxtable text(32), b_paytype Integer, billable Integer, billto_type Integer, billto_guid text(32), order_guid text(32));" &
//                "CREATE TABLE gnclock ( Hostname varchar(255), PID int );" &
//                "CREATE TABLE invoices (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL, date_opened text(14), date_posted text(14), notes text(2048) Not NULL, active Integer Not NULL, currency text(32) Not NULL, owner_type Integer, owner_guid text(32), terms text(32), billing_id text(2048), post_txn text(32), post_lot text(32), post_acc text(32), billto_type Integer, billto_guid text(32), charge_amt_num bigint, charge_amt_denom bigint);" &
//                "CREATE TABLE jobs (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL, name text(2048) Not NULL, reference text(2048) Not NULL, active Integer Not NULL, owner_type Integer, owner_guid text(32));" &
//                "CREATE TABLE lots (guid text(32) PRIMARY KEY Not NULL, account_guid text(32), is_closed Integer Not NULL);" &
//                "CREATE TABLE orders (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL, notes text(2048) Not NULL, reference text(2048) Not NULL, active Integer Not NULL, date_opened text(14) Not NULL, date_closed text(14) Not NULL, owner_type Integer Not NULL, owner_guid text(32) Not NULL);" &
//                "CREATE TABLE prices (guid text(32) PRIMARY KEY Not NULL, commodity_guid text(32) Not NULL, currency_guid text(32) Not NULL, Date text(14) Not NULL, source text(2048), type text(2048), value_num bigint Not NULL, value_denom bigint Not NULL);" &
//                "CREATE TABLE recurrences (id Integer PRIMARY KEY AUTOINCREMENT Not NULL, obj_guid text(32) Not NULL, recurrence_mult Integer Not NULL, recurrence_period_type text(2048) Not NULL, recurrence_period_start text(8) Not NULL, recurrence_weekend_adjust text(2048) Not NULL);" &
//                "CREATE TABLE schedxactions (guid text(32) PRIMARY KEY Not NULL, name text(2048), enabled Integer Not NULL, start_date text(8), end_date text(8), last_occur text(8), num_occur Integer Not NULL, rem_occur Integer Not NULL, auto_create Integer Not NULL, auto_notify Integer Not NULL, adv_creation Integer Not NULL, adv_notify Integer Not NULL, instance_count Integer Not NULL, template_act_guid text(32) Not NULL);" &
//                "CREATE TABLE slots (id Integer PRIMARY KEY AUTOINCREMENT Not NULL, obj_guid text(32) Not NULL, name text(4096) Not NULL, slot_type Integer Not NULL, int64_val bigint, string_val text(4096), double_val float8, timespec_val text(14), guid_val text(32), numeric_val_num bigint, numeric_val_denom bigint, gdate_val text(8));" &
//                "CREATE TABLE splits (guid text(32) PRIMARY KEY Not NULL, tx_guid text(32) Not NULL, account_guid text(32) Not NULL, memo text(2048) Not NULL, action text(2048) Not NULL, reconcile_state text(1) Not NULL, reconcile_date text(14), value_num bigint Not NULL, value_denom bigint Not NULL, quantity_num bigint Not NULL, quantity_denom bigint Not NULL, lot_guid text(32));" &
//                "CREATE TABLE taxtable_entries (id Integer PRIMARY KEY AUTOINCREMENT Not NULL, taxtable text(32) Not NULL, account text(32) Not NULL, amount_num bigint Not NULL, amount_denom bigint Not NULL, type Integer Not NULL);" &
//                "CREATE TABLE taxtables (guid text(32) PRIMARY KEY Not NULL, name text(50) Not NULL, refcount bigint Not NULL, invisible Integer Not NULL, parent text(32));" &
//                "CREATE TABLE transactions (guid text(32) PRIMARY KEY Not NULL, currency_guid text(32) Not NULL, num text(2048) Not NULL, post_date text(14), enter_date text(14), description text(2048));" &
//                "CREATE TABLE vendors (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, id text(2048) Not NULL, notes text(2048) Not NULL, currency text(32) Not NULL, active Integer Not NULL, tax_override Integer Not NULL, addr_name text(1024), addr_addr1 text(1024), addr_addr2 text(1024), addr_addr3 text(1024), addr_addr4 text(1024), addr_phone text(128), addr_fax text(128), addr_email text(256), terms text(32), tax_inc text(2048), tax_table text(32));" &
//                "CREATE TABLE versions (table_name text(50) PRIMARY KEY Not NULL, table_version Integer Not NULL); "
//
//            SQLcommand.CommandText &= CreateUSDCommoditySQL()
//            SQLcommand.CommandText &= CreateStarterAccountsSQL()
//            SQLcommand.CommandText &= CreateStarterVersionsSQL()
//            SQLcommand.CommandText &= CreateStarterBookRecordSQL()
//
//            Dim CommitString As String = "COMMIT;"
//            SQLcommand.CommandText &= CommitString
//
//            Dim SQLreader As SQLiteDataReader = SQLcommand.ExecuteReader()
//            SQLcommand.Dispose()
//            SQLconnect.Close()
//            connectionString = SQLconnect.ConnectionString
//            returnValue = True
//        Catch ex As Exception
//            Console.WriteLine("Error creating New file." & vbCrLf & ex.ToString())
//        End Try
//        Return returnValue
//    End Function
//
//    ''' <summary>
//    ''' CreateUSDCommoditySQL is the create USD commodity SQL statement.
//    ''' </summary>
//    ''' <returns></returns>
//    Private Function CreateUSDCommoditySQL() As String
//        Dim returnValue As String = ""
//        Dim commoditiesManager As New CommoditiesManager("")
//        returnValue = "INSERT INTO commodities(" & commoditiesManager._Fields & ") " &
//                                        "VALUES('" & (commoditiesManager.ConvertGUID_To_String(Guid.NewGuid)) & "'," &
//                                      "'" & commoditiesManager.CommodityNamespace_Currency & "','USD','US Dollar'," &
//                                      "'840',100,1,'" & commoditiesManager.CommodityTypes_Currency & "','');"
//        Return returnValue
//    End Function
//
//    ''' <summary>
//    ''' CreateStarterAccounts creates a starter checking account, and a starter root account.
//    ''' </summary>
//    ''' <remarks></remarks>
//    Private Function CreateStarterAccountsSQL() As String
//        Dim returnValue As String = ""
//        Dim commoditiesManager As New CommoditiesManager("")
//        Dim USDcommodityGUID As String = commoditiesManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue = "INSERT INTO commodities(" & commoditiesManager._Fields & ") " &
//                                        "VALUES('" & (USDcommodityGUID) & "'," &
//                                      "'" & commoditiesManager.CommodityNamespace_Currency & "','USD','US Dollar'," &
//                                      "'840',100,1,'" & commoditiesManager.CommodityTypes_Currency & "','');  "
//
//        'Create Root Account with information for first commodity
//        Dim rootAccountManager As New AccountsManager("")
//        Dim rootAccountGUID As String = rootAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & rootAccountManager._Fields & ") " &
//                                        "VALUES('" & rootAccountGUID & "','Root Account'," &
//                                      "'" & rootAccountManager.Account_Type_Root & "',NULL, " &
//                                      "0,0,NULL,'','',0,0);  "
//        'Create Template Root Account
//        Dim rootTemplateAccountsManager As New AccountsManager("")
//        Dim templateRootAccountGUID As String = rootTemplateAccountsManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & rootAccountManager._Fields & ") " &
//                                        "VALUES('" & templateRootAccountGUID & "','Template Root'," &
//                                      "'" & rootAccountManager.Account_Type_Root & "',NULL, " &
//                                      "0,0,NULL,'','',0,0);  "
//
//        'Create Assets Account
//        Dim assetsAccountManager As New AccountsManager("")
//        Dim assetsAccountGUID As String = assetsAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & rootAccountManager._Fields & ") " &
//                                        "VALUES('" & assetsAccountGUID & "','Assets'," &
//                                      "'" & rootAccountManager.Account_Type_Asset & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & rootAccountGUID & "','','Assets',0,1);  "
//
//        'Create Checking Account
//        Dim checkingAccountManager As New AccountsManager("")
//        Dim checkingAccountGUID As String = assetsAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & rootAccountManager._Fields & ") " &
//                                        "VALUES('" & checkingAccountGUID & "','Checking Account'," &
//                                      "'" & rootAccountManager.Account_Type_Asset & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & assetsAccountGUID & "','','',0,0);  "
//
//        'Create Expenses Account
//        Dim expensesAccountManager As New AccountsManager("")
//        Dim expensesAccountGUID As String = expensesAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & rootAccountManager._Fields & ") " &
//                                        "VALUES('" & expensesAccountGUID & "','Expenses'," &
//                                      "'" & rootAccountManager.Account_Type_Expense & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & rootAccountGUID & "','','Expenses',0,1);  "
//
//        'Create Groceries Account
//        Dim groceriesAccountManager As New AccountsManager("")
//        Dim groceriesAccountGUID As String = groceriesAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & groceriesAccountManager._Fields & ") " &
//                                        "VALUES('" & groceriesAccountGUID & "','Groceries'," &
//                                      "'" & rootAccountManager.Account_Type_Expense & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & expensesAccountGUID & "','','Groceries',0,0);  "
//
//        'Create Dining Account
//        Dim diningAccountManager As New AccountsManager("")
//        Dim diningAccountGUID As String = diningAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & diningAccountManager._Fields & ") " &
//                                        "VALUES('" & diningAccountGUID & "','Dining'," &
//                                      "'" & rootAccountManager.Account_Type_Expense & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & expensesAccountGUID & "','','Dining',0,0);  "
//
//        'Create Liabilities Account
//        Dim liabilitiesAccountManager As New AccountsManager("")
//        Dim liabilitiesAccountGUID As String = liabilitiesAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & liabilitiesAccountManager._Fields & ") " &
//                                        "VALUES('" & liabilitiesAccountGUID & "','Liabilities'," &
//                                      "'" & rootAccountManager.Account_Type_Liability & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & rootAccountGUID & "','','Liabilities',0,1);  "
//
//        'Create Credit Card Account
//        Dim creditCardAccountManager As New AccountsManager("")
//        Dim creditCardAccountGUID As String = creditCardAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & creditCardAccountManager._Fields & ") " &
//                                        "VALUES('" & creditCardAccountGUID & "','Credit Card'," &
//                                      "'" & rootAccountManager.Account_Type_Credit & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & liabilitiesAccountGUID & "','','Credit Card',0,0);  "
//
//        'Create Auto Account
//        Dim autoAccountManager As New AccountsManager("")
//        Dim autoAccountGUID As String = autoAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & autoAccountManager._Fields & ") " &
//                                        "VALUES('" & autoAccountGUID & "','Auto'," &
//                                      "'" & rootAccountManager.Account_Type_Expense & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & expensesAccountGUID & "','','Auto',0,0);  "
//
//        'Create Gas Account
//        Dim autoGasAccountManager As New AccountsManager("")
//        Dim autoGasAccountGUID As String = autoGasAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & autoGasAccountManager._Fields & ") " &
//                                        "VALUES('" & autoGasAccountGUID & "','Gas'," &
//                                      "'" & rootAccountManager.Account_Type_Expense & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & autoAccountGUID & "','','Gas',0,0);  "
//
//        'Create Income Account
//        Dim incomeAccountManager As New AccountsManager("")
//        Dim incomeAccountGUID As String = incomeAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & incomeAccountManager._Fields & ") " &
//                                        "VALUES('" & incomeAccountGUID & "','Income'," &
//                                      "'" & rootAccountManager.Account_Type_Income & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & rootAccountGUID & "','','Income',0,0);  "
//
//        'Create Salary Income Account
//        Dim incomeSalaryAccountManager As New AccountsManager("")
//        Dim incomeSalaryAccountGUID As String = incomeSalaryAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & incomeSalaryAccountManager._Fields & ") " &
//                                        "VALUES('" & incomeSalaryAccountGUID & "','Salary'," &
//                                      "'" & rootAccountManager.Account_Type_Income & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & incomeAccountGUID & "','','Salary',0,0);  "
//
//        'Create Sales Income Account
//        Dim incomeSalesAccountManager As New AccountsManager("")
//        Dim incomeSalesAccountGUID As String = incomeSalesAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & incomeSalesAccountManager._Fields & ") " &
//                                        "VALUES('" & incomeSalesAccountGUID & "','Sales'," &
//                                      "'" & rootAccountManager.Account_Type_Income & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & incomeAccountGUID & "','','Sales',0,0);  "
//
//        'Create Bonus Income Account
//        Dim incomeBonusAccountManager As New AccountsManager("")
//        Dim incomeBonusAccountGUID As String = incomeBonusAccountManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "INSERT INTO accounts(" & incomeBonusAccountManager._Fields & ") " &
//                                        "VALUES('" & incomeBonusAccountGUID & "','Bonus'," &
//                                      "'" & rootAccountManager.Account_Type_Income & "','" & USDcommodityGUID & "', " &
//                                      "100,0,'" & incomeAccountGUID & "','','Bonus',0,0);  "
//
//        Return returnValue
//    End Function
//
//    ''' <summary>
//    ''' CreateStarterVersionsTable creates all the entries for the versions table.
//    ''' </summary>
//    ''' <remarks></remarks>
//    Private Function CreateStarterVersionsSQL() As String
//        Dim returnValue As String = ""
//        'Create Gnucash row
//        Dim GnuCashVersion As String = "2060600"
//        Dim GnucashVersionManager As New VersionsManager("")
//        returnValue &= "INSERT INTO versions(" & GnucashVersionManager._Fields & ") " &
//                                        "VALUES('Gnucash','" & GnuCashVersion & "'); "
//
//        'Create Gnucash-Resave row
//        Dim GnucashResaveVersion As String = "19920"
//        Dim GnucashResaveVersionManager As New VersionsManager("")
//        returnValue &= "INSERT INTO versions(" & GnucashResaveVersionManager._Fields & ") " &
//                                        "VALUES('Gnucash-Resave','" & GnucashResaveVersion & "'); "
//
//        'Create accounts row
//        Dim accountsVersionManager As New VersionsManager("")
//        Dim accountsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & accountsVersionManager._Fields & ") " &
//                                        "VALUES('accounts','" & accountsVersion & "'); "
//
//        'Create books row
//        Dim booksVersionManager As New VersionsManager("")
//        Dim booksVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & booksVersionManager._Fields & ") " &
//                                        "VALUES('books','" & booksVersion & "'); "
//
//        'Create budgets row
//        Dim budgetsVersionManager As New VersionsManager("")
//        Dim budgetsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & budgetsVersionManager._Fields & ") " &
//                                        "VALUES('budgets','" & budgetsVersion & "'); "
//
//        'Create budget_amounts row
//        Dim budget_amountsVersionManager As New VersionsManager("")
//        Dim budgetAmountsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & booksVersionManager._Fields & ") " &
//                                        "VALUES('budget_amounts','" & budgetAmountsVersion & "'); "
//
//        'Create commodities row
//        Dim commoditiesVersionManager As New VersionsManager("")
//        Dim commoditiesVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & commoditiesVersionManager._Fields & ") " &
//                                        "VALUES('commodities','" & commoditiesVersion & "'); "
//
//        'Create lots row
//        Dim lotsVersionManager As New VersionsManager("")
//        Dim lotsVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & lotsVersionManager._Fields & ") " &
//                                        "VALUES('lots','" & lotsVersion & "'); "
//
//        'Create prices row
//        Dim pricesVersionManager As New VersionsManager("")
//        Dim pricesVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & pricesVersionManager._Fields & ") " &
//                                        "VALUES('prices','" & pricesVersion & "'); "
//
//        'Create schedxactions row
//        Dim schedxactionsVersionManager As New VersionsManager("")
//        Dim schedxactionsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & schedxactionsVersionManager._Fields & ") " &
//                                        "VALUES('schedxactions','" & schedxactionsVersion & "'); "
//
//        'Create transactions row
//        Dim transactionsVersionManager As New VersionsManager("")
//        Dim transactionsVersion As String = "3"
//        returnValue &= "INSERT INTO versions(" & transactionsVersionManager._Fields & ") " &
//                                        "VALUES('transactions','" & transactionsVersion & "'); "
//
//        'Create splits row
//        Dim splitsVersionManager As New VersionsManager("")
//        Dim splitsVersion As String = "4"
//        returnValue &= "INSERT INTO versions(" & splitsVersionManager._Fields & ") " &
//                                        "VALUES('splits','" & splitsVersion & "'); "
//
//        'Create billterms row
//        Dim billtermsVersionManager As New VersionsManager("")
//        Dim billtermsVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & billtermsVersionManager._Fields & ") " &
//                                        "VALUES('billterms','" & billtermsVersion & "'); "
//
//        'Create customers row
//        Dim customersVersionManager As New VersionsManager("")
//        Dim customersVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & customersVersionManager._Fields & ") " &
//                                        "VALUES('customers','" & customersVersion & "'); "
//
//        'Create employees row
//        Dim employeesVersionManager As New VersionsManager("")
//        Dim employeesVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & employeesVersionManager._Fields & ") " &
//                                        "VALUES('employees','" & employeesVersion & "'); "
//
//        'Create entries row
//        Dim entriesVersionManager As New VersionsManager("")
//        Dim entriesVersion As String = "3"
//        returnValue &= "INSERT INTO versions(" & entriesVersionManager._Fields & ") " &
//                                        "VALUES('entries','" & entriesVersion & "'); "
//
//        'Create invoices row
//        Dim invoicesVersionManager As New VersionsManager("")
//        Dim invoicesVersion As String = "3"
//        returnValue &= "INSERT INTO versions(" & invoicesVersionManager._Fields & ") " &
//                                        "VALUES('invoices','" & invoicesVersion & "'); "
//
//        'Create jobs row
//        Dim jobsVersionManager As New VersionsManager("")
//        Dim jobsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & jobsVersionManager._Fields & ") " &
//                                        "VALUES('jobs','" & jobsVersion & "'); "
//
//        'Create orders row
//        Dim ordersVersionManager As New VersionsManager("")
//        Dim ordersVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & ordersVersionManager._Fields & ") " &
//                                        "VALUES('orders','" & ordersVersion & "'); "
//
//        'Create taxtables row
//        Dim taxtablesVersionManager As New VersionsManager("")
//        Dim taxtablesVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & taxtablesVersionManager._Fields & ") " &
//                                        "VALUES('taxtables','" & taxtablesVersion & "'); "
//
//        'Create taxtable_entries row
//        Dim taxtable_entriesVersionManager As New VersionsManager("")
//        Dim taxtable_entriesVersion As String = "3"
//        returnValue &= "INSERT INTO versions(" & taxtable_entriesVersionManager._Fields & ") " &
//                                        "VALUES('taxtable_entries','" & taxtable_entriesVersion & "'); "
//
//        'Create vendors row
//        Dim vendorsVersionManager As New VersionsManager("")
//        Dim vendorsVersion As String = "1"
//        returnValue &= "INSERT INTO versions(" & vendorsVersionManager._Fields & ") " &
//                                        "VALUES('vendors','" & vendorsVersion & "'); "
//
//        'Create recurrences row
//        Dim recurrencesVersionManager As New VersionsManager("")
//        Dim recurrencesVersion As String = "2"
//        returnValue &= "INSERT INTO versions(" & recurrencesVersionManager._Fields & ") " &
//                                        "VALUES('recurrences','" & recurrencesVersion & "'); "
//
//        'Create slots row
//        Dim slotsVersionManager As New VersionsManager("")
//        Dim slotsVersion As String = "3"
//        returnValue &= "INSERT INTO versions(" & slotsVersionManager._Fields & ") " &
//                                        "VALUES('slots','" & slotsVersion & "'); "
//        Return returnValue
//    End Function
//
//
//    ''' <summary>
//    ''' CreateStarterBookRecordSQL creates the starter book record.
//    ''' </summary>
//    Private Function CreateStarterBookRecordSQL() As String
//        Dim returnValue As String = " "
//
//        Dim bookManager As New BooksManager("")
//        Dim bookGUID As String = bookManager.ConvertGUID_To_String(Guid.NewGuid)
//        returnValue &= " " &
//                       "CREATE TEMP TABLE IF NOT EXISTS Variables (Name TEXT PRIMARY KEY, Value TEXT); " &
//                       "INSERT OR REPLACE INTO Variables VALUES ('@rootGUID', (SELECT guid FROM accounts WHERE accounts.name='Root Account') ); " &
//                       "INSERT OR REPLACE INTO Variables VALUES ('@templateRootGUID', (SELECT guid FROM accounts WHERE accounts.name='Template Root') ); " &
//                       "INSERT INTO books(" & bookManager._Fields & ") " &
//                                "VALUES('" & bookGUID & "'," &
//                                "(SELECT Value FROM Variables WHERE Name='@rootGUID')," &
//                                "(SELECT Value FROM Variables WHERE Name='@templateRootGUID'));  "
//
//
//        Return returnValue
//    End Function