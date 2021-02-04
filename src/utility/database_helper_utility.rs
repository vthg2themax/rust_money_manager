/// database_helper_utility will be all the functions that have to do with database functionality, 
/// and helper methods to deal with the database. Nothing user facing should show here, so no alerts,
/// or other GUI things please.

use uuid::Uuid;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use chrono::prelude::*;
use regex::Regex;


#[wasm_bindgen()]
extern "C" {
    pub type Database;

    #[wasm_bindgen(constructor, js_namespace = sqlContext)]
    pub fn new(array: js_sys::Uint8Array) -> Database;

    #[wasm_bindgen(method)]
    pub fn prepare(this: &Database, s: &str) -> Statement;
    
    /// Free the memory allocated during parameter binding
    #[wasm_bindgen(method)]
    pub fn freemem(this: &Database);
}

#[wasm_bindgen]
extern "C" {
    
    pub type Statement;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Statement;

    #[wasm_bindgen(method)]
    pub fn bind(this: &Statement, binding_object : JsValue) -> bool;

    #[wasm_bindgen(method)]
    pub fn getAsObject(this: &Statement) -> JsValue;
    
    /// Free the memory used by the statement
    #[wasm_bindgen(method)]
    pub fn free(this: &Statement) -> bool;

    #[wasm_bindgen(method)]
    pub fn step(this: &Statement) -> bool;

}

const FORMAT_STRING : &str = "%Y%m%d%H%M%S";

/// valid_database checks the database for the first 16 chars to determine if it's
/// a valid database file. If it has this value, it probably is.
pub fn valid_database(incoming_database : js_sys::Uint8Array) -> Result<(),String> {    
    
    if (&incoming_database).to_vec().len() < 16 {
        let error_message : String = String::from(
                format!("The selected file is not a valid SQLite Database! Its length is {:?}.",
                (&incoming_database).to_vec().len())
        );
        return Err(error_message);
    }

    let first_16  = &js_sys::Uint8Array::new(&incoming_database).to_vec()[0..15];
    let mut descriptor : String = String::from("");
    for letter in first_16.iter() {
        descriptor += &(letter.to_ascii_uppercase() as char).to_string();
    }
    
    if !descriptor.starts_with("SQLITE FORMAT") {
        let error_message : String = String::from("The selected file does not have a valid SQLite Database header.");
        return Err(error_message);
    }
    
    Ok(())
}

///convert_string_to_guid converts the result to a string, if possible.
pub fn convert_string_to_guid(incoming_string : String) -> Result<Uuid,String> {
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

    return match Uuid::parse_str(&incoming_string) {
        Ok(guid) => Ok(guid),
        Err(e) => Err(format!("{0}",e)),
    };

}

// ///convert_string_result_to_guid converts the result to a guid, if possible.
// pub fn convert_string_result_to_guid(incoming_result : Result<String>) -> Result<GUID> {
//     //Carefully, unwrap the string, which could be a null
//     if incoming_result.is_err() {
//         return Ok(_null_guid());
//     }
//     let mut incoming_string = incoming_result.unwrap();
//     //If it's 32 characters, it needs dashes
//     if incoming_string.chars().count() == 32 {
//         let part1 = &incoming_string[0..8];
//         let part2 = &incoming_string[8..12];
//         let part3 = &incoming_string[12..16];
//         let part4 = &incoming_string[16..20];
//         let part5 = &incoming_string[20..32];
//         incoming_string = [part1,part2,part3,part4,part5].join("-");
//     }

//     return match GUID::parse(&incoming_string) {
//         Ok(guid) => Ok(guid),
//         Err(e) => panic!(format!("{0}",e)),
//     };

// }

///convert_guid_to_sqlite_string converts a guid to an sqlite string if possible, 
/// like so: f737a4904dac6736c7d8fe7b765ee354
pub fn convert_guid_to_sqlite_string(incoming_guid : &Uuid) -> String {    
    let mut incoming_guid = incoming_guid.to_string().to_lowercase();
    //If it's 36 characters, we chop off the dashes
    if incoming_guid.chars().count() == 36 {
        incoming_guid = incoming_guid.replace("-","");        
    }

    return incoming_guid;

}

// ///convert_guid_to_sqlite_parameter converts a guid to an sqlite string if possible, 
// /// like so: f737a4904dac6736c7d8fe7b765ee354 or NULL
// pub fn convert_guid_to_sqlite_parameter(incoming_guid : GUID) -> Result<Option<String>> {    
//     //If it's a null GUID we want to return a null value
//     if incoming_guid == _null_guid() {
//         return Ok(None);
//     }
//     //Otherwise attempt to convert the value to a sqlite guid string
//     return Ok(Some(convert_guid_to_sqlite_string(incoming_guid)?));

// }

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

/// convert_string_to_date_format attempts to convert a string to a NaiveDateTime
/// Warning! Will panic if the string is invalid.
pub fn convert_string_to_date(incoming_string: &str) -> Result<chrono::NaiveDateTime,String> {
     match NaiveDateTime::parse_from_str(incoming_string, FORMAT_STRING) {
        Ok(e) => {
            return Ok(e);
        },
        Err(_) => {
            return Err(format!("Failed to convert the given string '{}' to a date.", incoming_string));
        }
    };
}

// /// MakeBackupCopiesOfFile makes backups of the file and saves copies
// /// of it. It makes up to x number of copies!
// /// 
// pub fn make_backup_copies_of_file(incoming_file_path : &std::path::Path,
//                                   number_of_copies: u8) -> std::result::Result<bool, String> {

//     let file_information = std::fs::metadata(incoming_file_path);

//     //Ensure this is in a directory we can reach
//     if file_information.is_err() == true {
//         let error_message : String = 
//                             format!("The given directory is not a valid result. {:#?}", 
//                                     file_information.err());
//         return Err(error_message);
//     }
//     //Ensure this is a not directory
//     if file_information.unwrap().is_dir() == true {
//         return Err(String::from(
//             "The given path is a directory! We do not make copies of directories.")
//         );
//     }

//     //Get the file name for this file without the (0).bak piece
//     let base_file_name: String = String::from(incoming_file_path.file_name()
//                                               .expect("Invalid File Name!")
//                                               .to_str()
//                                               .expect("Invalid File Name!"));
//     //Get the parent directory for this file as an easy to use string
//     let directory_file_path: String = String::from(
//                                         incoming_file_path.parent()
//                                         .expect("bad directory file path").to_str()
//                                         .expect(&["Directory File Path could not ",
//                                                   "be converted to string."].join("")));

//     //Get all the files that end with ([0-9]).bak files in the directory
//     let re = Regex::new(r"^.*[(](\d+)[)][.][Bb][Aa][Kk]$").unwrap();
//     let mut files_that_match : Vec<String> = Vec::new();
//     //Get the other files in the directory
//     let files = std::fs::read_dir(&directory_file_path)
//                                 .expect("Failed To Read Directory!");
    
//     for file in files {
//         let filename : String = file.unwrap().path().file_name().unwrap()
//                                     .to_str().unwrap().into();
//         if re.is_match(&filename) {
//             files_that_match.push(filename.clone());
//         }
//     }

//     //Lets do like a *(0).bak file filename schema 
//     //The larger the number, the older the file, we set the date modified on .bak 
//     //files to when the bak file was created at
//     //If there's not a *(0).bak file name, then we need to create it
//     if files_that_match.contains(&[&base_file_name,"(0).bak"].join("")) == false {
//         //Copy the original file to original file + "(0).bak"
//         std::fs::copy(incoming_file_path, 
//                       &[incoming_file_path.to_str().expect("Invalid Path!"), 
//                         "(0).bak"].join("")).expect("Failed To Copy The (0).bak file.");
//         //Set the last write time to now (Cannot Do with RUST YET! )
//         return Ok(true);
//     }

//     //Go through, and wipe out backup files that are more than the requested amount, or
//     //will be more than the backup amount. Backup files are created: (0).bak -> (X).bak
//     if files_that_match.len()  >= (number_of_copies as usize) {
//         //Delete all older .bak files greater than number_of_copies
//         for file in &mut files_that_match {
//             //Attempt to get the filename number to check against
//             let backup_number : u8 = re.captures_iter(&file).next()
//                                     .expect("Backup Number Not Found!")[1]
//                                     .parse::<u8>()
//                                     .expect("Backup Number Not Actually Number!");
                       
//             println!("Backup Number is: '{:#?}' for file '{1}'.",
//                     backup_number, file.as_str());
//             //If the backup file number is large enough, we delete the file
//             if backup_number >= number_of_copies {
//                 let file_path = std::path::Path::new(incoming_file_path.parent()
//                                     .expect("Invalid File Path!")
//                                     .to_str().expect("Invalid File Path!")
//                                     ).join(file);

//                 match std::fs::remove_file(&file_path) {
//                     Ok(_) => {println!{"Deleted file: '{:#?}'", &file_path}},
//                     Err(e) => {println!("{0}",e);},
//                 }
//             }
//             //Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
//             //let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//             //assert!(re.is_match("2014-01-01"));
//             //file.chars().position(|c| c == 'g').unwrap()
//         }

//         //Now that we have deleted some files, we need to rescan the files that match
//         files_that_match.clear();
//         let files = std::fs::read_dir(&directory_file_path).expect("Failed To Read Directory!");
//         for file in files {
//             let filename : String = file.unwrap().path().file_name().unwrap().to_str().unwrap().into();
//             if re.is_match(&filename) {
//                 files_that_match.push(filename.clone());
//             }
//         }
//     }    

//     //Sort the files by backup number descending
//     files_that_match.sort_by(|a, b| 
//         {   let this_filename = a;
//             let next_filename = b;
//             let this_number : u8 = re.captures_iter(&this_filename).next()
//                                 .expect("Backup Number Not Found!")[1]
//                                 .parse::<u8>()
//                                 .expect("Backup Number Not Actually Number!");
//             let next_number : u8 = re.captures_iter(&next_filename).next()
//                                 .expect("Backup Number Not Found!")[1]
//                                 .parse::<u8>()
//                                 .expect("Backup Number Not Actually Number!");
//             next_number.cmp(&this_number)
//         }
//     );

//     //Now we can move the files along higher to lower
//     // Lets say we have the following files: (2).bak, (1).bak, (0).bak
//     // We should get each of the backup numbers, such as the oldest file (2).bak,
//     // and then rename it to (X+1).bak, all the way down until we get to (0).bak, 
//     // (which is renamed to (1).bak). At this point, we simply create the (0).bak file from the curent file.    
//     for file in &mut files_that_match {
//         //Attempt to get the filename number to check against
//         let backup_number : u8 = re.captures_iter(&file).next()
//                                 .expect("Backup Number Not Found!")[1]
//                                 .parse::<u8>()
//                                 .expect("Backup Number Not Actually Number!");
                    
//         println!("Backup Number is: '{:#?}' for file '{1}'.", backup_number, &file.as_str());

//         //rename the file X+1
//         let old_file_path = std::path::Path::new(&directory_file_path).join(&file);
//         let new_file_path = std::path::Path::new(&directory_file_path)
//                                 .join(
//                                     &[&base_file_name,"(",&(backup_number+1).to_string(),").bak"].join("")
//                                 );
//         match std::fs::rename(&old_file_path, &new_file_path) {
//             Ok(_) => {println!{"Renamed file: '{:#?}' to '{:#?}'.", &old_file_path, &new_file_path}},
//             Err(e) => {println!("{0}",e);},
//         }

//     }
    
//     //Finally we copy the current file to (0).bak to complete the backup process
//     std::fs::copy(incoming_file_path, &[incoming_file_path.to_str().expect("Invalid Path!"), "(0).bak"].join(""))
//             .expect("Failed To Copy The (0).bak file.");

//     Ok(true)
    
// }

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn test_creating_new_file() {
    //     let file_path = "/home/vince/Documents/new_test_file.sqlite";
    //     let path = std::path::Path::new(file_path);
    //     let result_of_file_operation = create_new_gnucash_file(path);
    //     if result_of_file_operation.is_err() {
    //         panic!(format!("There was an Error: '{:#?}'.", result_of_file_operation.err()));
    //     }
    // }
}

// pub fn create_new_gnucash_file(incoming_file_path_with_file_name : &std::path::Path) -> std::result::Result<bool, String> {
//     //Test if the given file path is a directory or an existing file
//     let file_information = std::fs::metadata(incoming_file_path_with_file_name);
//     match file_information {
//         Ok(_)=> {
//             //Since it's not an error, unwrap it to continue
//             let file_information = file_information.unwrap();

//             //Ensure this is a not directory
//             if file_information.is_dir() == true {
//                 return Err(String::from(
//                     ["The given path is a directory! ",
//                     "Please ensure that you enter a valid file name",
//                     " after the Directory Name."].join(""))
//                 );
//             }
//             //Ensure this file does not already exist
//             if file_information.is_file() == true {
//                 return Err(String::from(
//                     ["The given file already exists. ",
//                     "Please ensure that you enter a new file name",
//                     " after the Directory Name."].join(""))
//                 );
//             }
//         },
//         Err(_)=>{} //Do nothing, since we this will error if the path does not exist
//     }    
    
//     //Attempt to get the directory that this file lives in
//     let directory_file_path = incoming_file_path_with_file_name.parent();
//     match directory_file_path {
//         Some(_) => { },
//         None => {
//             return Err(format!(
//                 "The parent for the given directory is not valid. '{:#?}'.",
//                         &incoming_file_path_with_file_name));
//         }
//     }
    
//     //Attempt to create a new sqlite file
//     let new_file = Connection::open(&incoming_file_path_with_file_name);
//     let mut new_file = match new_file {
//         Ok(con) => { con },
//         Err(e) => {
//             return Err(format!("There was an error creating the file. {}",e));
//         },
//     };

//     let tx = new_file.transaction();
//     let tx = match tx {
//         Ok(tx) => { tx },
//         Err(e) => {
//             return Err(format!("There was an error starting the transaction. {}",e));
//         },
//     };
    
//     //Create the accounts table
//     {
//         let sql = ["CREATE TABLE accounts (guid text(32) PRIMARY KEY Not NULL,",
//                    " name text(2048) Not NULL, account_type text(2048) Not NULL,",
//                    " commodity_guid text(32), commodity_scu Integer Not NULL,",
//                    " non_std_scu Integer Not NULL, parent_guid text(32),",
//                    " code text(2048), description text(2048), hidden Integer,",
//                    " placeholder Integer);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create the billterms table
//     {
//         let sql = ["CREATE TABLE billterms (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL,",
//            " description text(2048) Not NULL, refcount Integer Not NULL, invisible Integer Not NULL,",
//            " parent text(32), type text(2048) Not NULL, duedays Integer, discountdays Integer, ",
//            " discount_num bigint, discount_denom bigint, cutoff Integer);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create the books table
//     {
//         let sql = ["CREATE TABLE books (guid text(32) PRIMARY KEY Not NULL, root_account_guid text(32) Not NULL,",
//            " root_template_guid text(32) Not NULL);",].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create the budget amounts table
//     {
//         let sql = ["CREATE TABLE budget_amounts (id Integer PRIMARY KEY AUTOINCREMENT Not NULL,",
//            " budget_guid text(32) Not NULL, account_guid text(32) Not NULL, period_num Integer Not NULL,",
//            " amount_num bigint Not NULL, amount_denom bigint Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    

//     //Create the budgets table
//     {
//         let sql = ["CREATE TABLE budgets (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL,",
//            " description text(2048), num_periods Integer Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    

//     //Create the commodities table
//     {
//         let sql = ["CREATE TABLE commodities (guid text(32) PRIMARY KEY Not NULL, Namespace text(2048) Not NULL,",
//            " mnemonic text(2048) Not NULL, fullname text(2048), cusip text(2048), fraction Integer Not NULL,",
//            " quote_flag Integer Not NULL, quote_source text(2048), quote_tz text(2048));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    

//     //Create the customers table
//     {
//         let sql = ["CREATE TABLE customers (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL, ",
//             " id text(2048) Not NULL, notes text(2048) Not NULL, active Integer Not NULL,",
//             " discount_num bigint Not NULL, discount_denom bigint Not NULL, credit_num bigint Not NULL,",
//             " credit_denom bigint Not NULL, currency text(32) Not NULL, tax_override Integer Not NULL,",
//             " addr_name text(1024), addr_addr1 text(1024), addr_addr2 text(1024), addr_addr3 text(1024),",
//             " addr_addr4 text(1024), addr_phone text(128), addr_fax text(128), addr_email text(256),",
//             " shipaddr_name text(1024), shipaddr_addr1 text(1024), shipaddr_addr2 text(1024),",
//             " shipaddr_addr3 text(1024), shipaddr_addr4 text(1024), shipaddr_phone text(128),",
//             " shipaddr_fax text(128), shipaddr_email text(256), terms text(32), tax_included Integer,",
//             " taxtable text(32));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the employees table
//     {
//         let sql = ["CREATE TABLE employees (guid text(32) PRIMARY KEY Not NULL, username text(2048) Not NULL,",
//             " id text(2048) Not NULL, language text(2048) Not NULL, acl text(2048) Not NULL,",
//             " active Integer Not NULL, currency text(32) Not NULL, ccard_guid text(32),",
//             " workday_num bigint Not NULL, workday_denom bigint Not NULL, rate_num bigint Not NULL,",
//             " rate_denom bigint Not NULL, addr_name text(1024), addr_addr1 text(1024),",
//             " addr_addr2 text(1024), addr_addr3 text(1024), addr_addr4 text(1024), addr_phone text(128),",
//             " addr_fax text(128), addr_email text(256));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the entries table
//     {
//         let sql = ["CREATE TABLE entries (guid text(32) PRIMARY KEY Not NULL, Date text(14) Not NULL,",
//                    " date_entered text(14), description text(2048), action text(2048), notes text(2048),",
//                    " quantity_num bigint, quantity_denom bigint, i_acct text(32), i_price_num bigint,",
//                    " i_price_denom bigint, i_discount_num bigint, i_discount_denom bigint, invoice text(32),",
//                    " i_disc_type text(2048), i_disc_how text(2048), i_taxable Integer, i_taxincluded Integer,",
//                    " i_taxtable text(32), b_acct text(32), b_price_num bigint, b_price_denom bigint,",
//                    " bill text(32), b_taxable Integer, b_taxincluded Integer, b_taxtable text(32),",
//                    " b_paytype Integer, billable Integer, billto_type Integer, billto_guid text(32),",
//                    " order_guid text(32));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }

//     }

//     //Create the gnclock table
//     {
//         let sql = "CREATE TABLE gnclock ( Hostname varchar(255), PID int );";
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the invoices table
//     {
//         let sql = ["CREATE TABLE invoices (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL,",
//                    " date_opened text(14), date_posted text(14), notes text(2048) Not NULL,",
//                    " active Integer Not NULL, currency text(32) Not NULL, owner_type Integer,",
//                    " owner_guid text(32), terms text(32), billing_id text(2048), post_txn text(32),",
//                    " post_lot text(32), post_acc text(32), billto_type Integer, billto_guid text(32),",
//                    " charge_amt_num bigint, charge_amt_denom bigint);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the jobs table
//     {
//         let sql = ["CREATE TABLE jobs (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL,",
//                    " name text(2048) Not NULL, reference text(2048) Not NULL, active Integer Not NULL,",
//                    " owner_type Integer, owner_guid text(32));",].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }           
//     }

//     //Create the lots table
//     {
//         let sql = ["CREATE TABLE lots (guid text(32) PRIMARY KEY Not NULL, account_guid text(32),",
//                    " is_closed Integer Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create the orders table
//     {
//         let sql = ["CREATE TABLE orders (guid text(32) PRIMARY KEY Not NULL, id text(2048) Not NULL,",
//                    " notes text(2048) Not NULL, reference text(2048) Not NULL, active Integer Not NULL,",
//                    " date_opened text(14) Not NULL, date_closed text(14) Not NULL,",
//                    " owner_type Integer Not NULL, owner_guid text(32) Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the prices table
//     {
//         let sql = ["CREATE TABLE prices (guid text(32) PRIMARY KEY Not NULL, commodity_guid text(32) Not NULL,",
//                    " currency_guid text(32) Not NULL, Date text(14) Not NULL, source text(2048), type text(2048),",
//                    " value_num bigint Not NULL, value_denom bigint Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the recurrences table
//     {
//         let sql = ["CREATE TABLE recurrences (id Integer PRIMARY KEY AUTOINCREMENT Not NULL,",
//                    " obj_guid text(32) Not NULL, recurrence_mult Integer Not NULL,",
//                    " recurrence_period_type text(2048) Not NULL, recurrence_period_start text(8) Not NULL,",
//                    " recurrence_weekend_adjust text(2048) Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the schedxactions table
//     {
//         let sql = ["CREATE TABLE schedxactions (guid text(32) PRIMARY KEY Not NULL, name text(2048),",
//                    " enabled Integer Not NULL, start_date text(8), end_date text(8), last_occur text(8),",
//                    " num_occur Integer Not NULL, rem_occur Integer Not NULL, auto_create Integer Not NULL,",
//                    " auto_notify Integer Not NULL, adv_creation Integer Not NULL,",
//                    " adv_notify Integer Not NULL, instance_count Integer Not NULL,",
//                    " template_act_guid text(32) Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the slots table
//     {
//         let sql = ["CREATE TABLE slots (id Integer PRIMARY KEY AUTOINCREMENT Not NULL,",
//                    " obj_guid text(32) Not NULL, name text(4096) Not NULL, slot_type Integer Not NULL,",
//                    " int64_val bigint, string_val text(4096), double_val float8, timespec_val text(14),",
//                    " guid_val text(32), numeric_val_num bigint, numeric_val_denom bigint,",
//                    " gdate_val text(8));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the splits table
//     {
//         let sql = ["CREATE TABLE splits (guid text(32) PRIMARY KEY Not NULL, tx_guid text(32) Not NULL,",
//                    " account_guid text(32) Not NULL, memo text(2048) Not NULL, action text(2048) Not NULL,",
//                    " reconcile_state text(1) Not NULL, reconcile_date text(14), value_num bigint Not NULL,",
//                    " value_denom bigint Not NULL, quantity_num bigint Not NULL,",
//                    " quantity_denom bigint Not NULL, lot_guid text(32));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the taxtable_entries table
//     {
//         let sql = ["CREATE TABLE taxtable_entries (id Integer PRIMARY KEY AUTOINCREMENT Not NULL,",
//                    " taxtable text(32) Not NULL, account text(32) Not NULL, amount_num bigint Not NULL,",
//                    " amount_denom bigint Not NULL, type Integer Not NULL);"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the taxtables table
//     {
//         let sql = ["CREATE TABLE taxtables (guid text(32) PRIMARY KEY Not NULL, name text(50) Not NULL,",
//                    " refcount bigint Not NULL, invisible Integer Not NULL, parent text(32));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the transactions table
//     {
//         let sql = ["CREATE TABLE transactions (guid text(32) PRIMARY KEY Not NULL,",
//                    " currency_guid text(32) Not NULL, num text(2048) Not NULL, post_date text(14),",
//                    " enter_date text(14), description text(2048));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the vendors table
//     {
//         let sql = ["CREATE TABLE vendors (guid text(32) PRIMARY KEY Not NULL, name text(2048) Not NULL,",
//                    " id text(2048) Not NULL, notes text(2048) Not NULL, currency text(32) Not NULL,",
//                    " active Integer Not NULL, tax_override Integer Not NULL, addr_name text(1024),",
//                    " addr_addr1 text(1024), addr_addr2 text(1024), addr_addr3 text(1024),",
//                    " addr_addr4 text(1024), addr_phone text(128), addr_fax text(128),",
//                    " addr_email text(256), terms text(32), tax_inc text(2048), tax_table text(32));"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create the versions table
//     {
//         let sql = ["CREATE TABLE versions (table_name text(50) PRIMARY KEY Not NULL,",
//                    " table_version Integer Not NULL); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the Starter USD Commodity
//     {
//         let sql = ["INSERT INTO commodities(", &String::from(commodities_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap() , "',", 
//                    "'CURRENCY','USD','US Dollar','840',100,1,'currency','');"].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the Root Account with information for first commodity
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Root Account','ROOT',NULL,0,0,NULL,'','',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }


//     //Create Template Root Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Template Root','ROOT',NULL,0,0,NULL,'','',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Assets Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Assets','ASSET',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Root Account'),",
//                    "'','Assets',0,1); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Checking Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Checking Account','ASSET',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Assets'),",
//                    "'','',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Expenses Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Expenses','EXPENSE',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Root Account'),",
//                    "'','Expenses',0,1); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create Groceries Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Groceries','EXPENSE',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Expenses'),",
//                    "'','Groceries',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Dining Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Dining','EXPENSE',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Expenses'),",
//                    "'','Dining',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create Liabilities Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Liabilities','LIABILITY',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='LIABILITY'),",
//                    "'','Liabilities',0,1); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Credit Card Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Credit Card','CREDIT',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Liabilities'),",
//                    "'','Credit Card',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Auto Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Auto','EXPENSE',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Expenses'),",
//                    "'','Auto',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Gas Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Gas','EXPENSE',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Auto'),",
//                    "'','Gas',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Income Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Income','INCOME',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Root Account'),",
//                    "'','Income',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Salary Income Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Salary','INCOME',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Income'),",
//                    "'','Salary',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Sales Income Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Sales','INCOME',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Income'),",
//                    "'','Sales',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Bonus Income Account
//     {
//         let sql = ["INSERT INTO accounts(", &String::from(accounts_manager::_fields()), ") ",
//                    "VALUES('", &convert_guid_to_sqlite_string(GUID::rand()).unwrap(), "',",
//                    "'Bonus','INCOME',(SELECT guid FROM commodities WHERE cusip='840'),",
//                    "100,0,(SELECT guid FROM accounts WHERE name='Income'),",
//                    "'','Bonus',0,0); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Gnucash row
//     {
//         let gnu_cash_version = "2060600";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('Gnucash','", &String::from(gnu_cash_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create Gnucash-Resave row
//     {
//         let gnu_cash_resize_version = "19920";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('Gnucash-Resave','", &String::from(gnu_cash_resize_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create accounts row
//     {
//         let accounts_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('accounts','", &String::from(accounts_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create books row
//     {
//         let books_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('books','", &String::from(books_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create budgets row
//     {
//         let budgets_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('budgets','", &String::from(budgets_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create budget_amounts row
//     {
//         let budget_amounts_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('budget_amounts','", &String::from(budget_amounts_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create commodities row
//     {
//         let commodities_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('commodities','", &String::from(commodities_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create lots row
//     {
//         let lots_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('lots','", &String::from(lots_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create prices row
//     {
//         let prices_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('prices','", &String::from(prices_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create schedxactions row
//     {
//         let schedxactions_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('schedxactions','", &String::from(schedxactions_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create transactions row
//     {
//         let transactions_version = "3";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('transactions','", &String::from(transactions_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create splits row
//     {
//         let splits_version = "4";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('splits','", &String::from(splits_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create billterms row
//     {
//         let billterms_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('billterms','", &String::from(billterms_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create customers row
//     {
//         let customers_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('customers','", &String::from(customers_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create employees row
//     {
//         let employees_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('employees','", &String::from(employees_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create entries row
//     {
//         let entries_version = "3";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('entries','", &String::from(entries_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create invoices row
//     {
//         let invoices_version = "3";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('invoices','", &String::from(invoices_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create jobs row
//     {
//         let jobs_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('jobs','", &String::from(jobs_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create orders row
//     {
//         let orders_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('orders','", &String::from(orders_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create taxtables row
//     {
//         let taxtables_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('taxtables','", &String::from(taxtables_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create taxtable_entries row
//     {
//         let taxtable_entries_version = "3";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('taxtable_entries','", &String::from(taxtable_entries_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create vendors row
//     {
//         let vendors_version = "1";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('vendors','", &String::from(vendors_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create recurrences row
//     {
//         let recurrences_version = "2";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('recurrences','", &String::from(recurrences_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }
    
//     //Create slots row
//     {
//         let slots_version = "3";
        
//         let sql = ["INSERT INTO versions(", &String::from(versions_manager::_fields()), ") ",
//                    "VALUES('slots','", &String::from(slots_version),
//                    "'); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     //Create the books record
//     {                   
//         let sql = ["INSERT INTO books(", &String::from(books_manager::_fields()), ") ",
//                    "VALUES('",&convert_guid_to_sqlite_string(GUID::rand()).unwrap(),"',",
//                    "  (SELECT guid FROM accounts WHERE accounts.name='Root Account'),",
//                    "  (SELECT guid FROM accounts WHERE accounts.name='Template Root')",
//                    "); "].join("");
//         match tx.execute(&sql,NO_PARAMS) {
//             Ok(_) => {  },
//             Err(e) => {
//                 return Err(format!("There was an error executing the transaction. {}",e));
//             },
//         }
//     }

//     // tx.execute("delete from cat_colors", NO_PARAMS)?;
//     // tx.execute("insert into cat_colors (name) values (?1)", &[&"lavender"])?;
//     // tx.execute("insert into cat_colors (name) values (?1)", &[&"blue"])?;

//     match tx.commit() {
//         Ok(_) => {  },
//         Err(e) => {
//             return Err(format!("There was an error committing the transaction. {}",e));
//         },
//     }


    

//     Ok(true)
// }
