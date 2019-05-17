use rusqlite::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;

// id,obj_guid,name,slot_type,int64_val,string_val,double_val,timespec_val,guid_val,
// numeric_val_num,numeric_val_denom,gdate_val 


#[derive(Debug)]
pub struct Slot {
    pub id: i64, //id is the Slot's id, it's an autoincrementing integer. Set to -1 to allow it to do that.
    pub obj_guid: GUID, //obj_guid is the object guid associated with this record.
    pub name: String, //name is the name that this slot is associated with. (Ex: 'notes' means a note on a transaction.)
    pub slot_type: i64, //slot_type is the integer type for this slot. (Ex: '4' means a note about a transaction.,'10' means a date-posted)
    pub int64_val: i64, //int64 is 0, unless it's actually used.
    pub string_val: String, //string_val is the information about this slot. (Ex: '32mpg' is the note about a transaction.)
    pub double_val: f64, //double_val is a float value for this slot. (Ex: '0.0')
    pub timespec_val: String, //timespec_val is a null value that could eventually be used
    pub guid_val: String, //guid_val is a null value string that could eventually be used
    pub numeric_val_num: i64, //numeric_val_num is the numeric value number. 0 by default
    pub numeric_val_denom: i64, //numeric_val_denom is the denom 1 by default.
    pub gdate_val: String, //gdate_val is a null value that could eventuall be used
}

pub fn _fields() -> String {
    String::from(
        ["id,obj_guid,name,slot_type,int64_val,string_val,double_val,timespec_val,guid_val,",
         "numeric_val_num,numeric_val_denom,gdate_val"].join("")
         )
} 

pub fn read_row_into_new_slot(incoming_row: &rusqlite::Row<'_>) -> Result<Slot> {
    Ok(
        Slot {
            id: incoming_row.get(0)?,
            obj_guid: dhu::convert_string_result_to_guid(incoming_row.get(1))?,
            name: incoming_row.get(2)?,
            slot_type: incoming_row.get(3)?,
            int64_val: incoming_row.get(4)?,
            string_val: incoming_row.get(5)?,
            double_val: incoming_row.get(6)?,
            timespec_val: incoming_row.get(7)?,
            guid_val: incoming_row.get(8)?,
            numeric_val_num: incoming_row.get(9)?,
            numeric_val_denom: incoming_row.get(10)?,
            gdate_val: incoming_row.get(11)?,
        }
    )
}


pub fn retrieve_all_for_obj_guid(file_path: &str, incoming_guid: GUID) -> Result<Vec<Slot>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the book records
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM slots WHERE obj_guid=@obj_guid ", 
         ""].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the commodities into a vector for returning the result
    let mut slots : Vec<Slot> = Vec::new();
    let mapped_rows = stmt.query_map_named(
        named_params!{"@obj_guid": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, 
        |row|
        read_row_into_new_slot(row)
    )?;

    //Now we can put each of the mapped row results into the results vector
    for row in mapped_rows {
        slots.push(row?);
    }    

    Ok(slots)
}

pub fn retrieve_all_for_name(file_path: &str, incoming_name: String) -> Result<Vec<Slot>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the book records
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM slots WHERE name=@name ", 
         ""].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the commodities into a vector for returning the result
    let mut slots : Vec<Slot> = Vec::new();
    let mapped_rows = stmt.query_map_named(
        named_params!{"@name": incoming_name }, 
        |row|
        read_row_into_new_slot(row)
    )?;

    //Now we can put each of the mapped row results into the results vector
    for row in mapped_rows {
        slots.push(row?);
    }    

    Ok(slots)
}

pub fn retrieve_all_for_guid_val(file_path: &str, incoming_guid: GUID) -> Result<Vec<Slot>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the book records
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM slots WHERE guid_val=@guid_val ", 
         ""].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the commodities into a vector for returning the result
    let mut slots : Vec<Slot> = Vec::new();
    let mapped_rows = stmt.query_map_named(
        named_params!{"@guid_val": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, 
        |row|
        read_row_into_new_slot(row)
    )?;

    //Now we can put each of the mapped row results into the results vector
    for row in mapped_rows {
        slots.push(row?);
    }    

    Ok(slots)
}



// ///retrieve_all_slots retrieves all the records.
// pub fn retrieve_all_slots(file_path : &str) -> Result<Vec<Lot>> {
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
//                     is_closed: if (row.get(2)? == 1){ true } else { false },
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
//     let mut lots : Vec<Loty> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@guid": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, |row| 
//         Ok( 
//             Lot{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     account_guid: dhu::convert_string_result_to_guid(row.get(1))?,
//                     is_closed: if (row.get(2)? == 1){ true } else { false },
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
//             "@guid" : dhu::convert_guid_to_sqlite_string(incoming_commodity.guid)?,
//             "@account_guid" : dhu::convert_guid_to_sqlite_string(incoming_lot.account_guid)?,
//             "@@is_closed" : if(incoming_lot.is_closed==true){1} else {0},
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

pub fn delete_existing(file_path : &str, incoming_obj_guid : GUID) -> Result<bool> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    
    let sql = 
        ["DELETE FROM slots ",
        " WHERE obj_guid=@obj_guid"
        ].join("");

    let result = conn.execute_named(&sql,
        named_params!{
            "@obj_guid" : dhu::convert_guid_to_sqlite_string(
                                                incoming_obj_guid)?,
        }
        ).unwrap();    

    
    if result != 1 {
        panic!(format!("There were {0} record changes instead of just 1!",
                        result.to_string())
        );
    }

    Ok(true)
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}