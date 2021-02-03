use uuid::Uuid;

//guid, account_guid, is_closed


#[derive(Debug)]
pub struct Lot {
    pub guid: Uuid, //guid is the lot's guid. Not NULL
    pub account_guid: Uuid, //account_guid is the account guid for this lot. Can Be Null
    pub is_closed: bool, //is_closed is whether this lot is closed. NOT NULL
}

pub fn _fields() -> String {
    String::from(
        ["guid, account_guid, is_closed",
         ""].join("")
         )
} 

// ///retrieve_all_lots retrieves all the records.
// pub fn retrieve_all_lots(file_path : &str) -> Result<Vec<Lot>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the book records
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM lots ", 
//          ""].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the commodities into a vector for returning the result
//     let mut lots : Vec<Lot> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Lot{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     account_guid: dhu::convert_string_result_to_guid(row.get(1))?,
//                     is_closed: row.get(2)?,
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the results vector
//     for row in mapped_rows {
//         lots.push(row?);
//     }    

//     Ok(lots)
// }

// ///retrieve_by_guid retrieves a lot by it's guid.
// pub fn retrieve_by_guid(file_path : &str, incoming_guid : GUID) -> Result<Vec<Lot>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the lot record fields
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM lots ", 
//          "WHERE guid=@guid"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the records into a vector for returning the result
//     let mut lots : Vec<Lot> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@guid": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, |row| 
//         Ok( 
//             Lot{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     account_guid: dhu::convert_string_result_to_guid(row.get(1))?,
//                     is_closed: row.get(2)?,
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the results vector
//     for row in mapped_rows {
//         lots.push(row?);
//     }    

//     Ok(lots)
// }

// pub fn save_new(file_path : &str, incoming_lot : &Lot) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["INSERT INTO lots (", &_fields(),") values (",
//          "@guid,@account_guid,@is_closed",
//          ")"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_lot.guid)?,
//             "@account_guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_lot.account_guid)?,
//             "@is_closed" : incoming_lot.is_closed,
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

// pub fn update_existing(file_path : &str, incoming_lot : &Lot) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["UPDATE lots SET ",
//                                 "account_guid=@account_guid,",
//                                 "is_closed=@is_closed ",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(incoming_lot.guid)?,
//             "@account_guid" : dhu::convert_guid_to_sqlite_string(incoming_lot.account_guid)?,
//             "@@is_closed" : if incoming_lot.is_closed {1} else {0},
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

// pub fn delete_existing(file_path : &str, incoming_guid : GUID) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["DELETE FROM lots ",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_guid)?,
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}