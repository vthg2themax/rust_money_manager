use rusqlite::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;

//guid,namespace,mnemonic,fullname,cusip,fraction,quote_flag,quote_source,quote_tz


#[derive(Debug)]
pub struct Commodity {
    pub guid: GUID, //guid is the GUID for this record.
    pub namespace: String, //namespace is the namespace of the commodity. (Ex: 'CURRENCY')
    pub mnemonic: String, //mnemonic is the commodity mnemoic. (Ex: 'USD' or 'BKC')
    pub fullname: String, //fullname is the full name. (Ex: 'United States Dollar')
    pub cusip: String, //(Ex: 840 for USD): This is any numeric or alphanumeric code that is used to identify the commodity. The CUSIP code is a unique identifying numeric string that is associated with every stock, bond or mutual fund, and most kinds of traded options, futures and commodities. 
    pub fraction: i64, //Fraction is the amount it's divisible. (Ex: 'USD' = 100)
    pub quote_flag: i32, //Quote_Flag is the quote's flag. (Ex: 'USD' = 1)
    pub quote_source: String, //Quote_Source is unknown. (Ex: 'USD' = 'currency')
    pub quote_tz: String, //(Ex: 'USD' = Empty String): The timezone to assign on the online quotes
}

pub fn _fields() -> String {
    String::from(
        ["guid,namespace,mnemonic,fullname,cusip,",
         "fraction,quote_flag,quote_source,quote_tz"].join("")
         )
} 

///retrieve_all_commodities retrieves all the commodity records.
pub fn retrieve_all_commodities(file_path : &str) -> Result<Vec<Commodity>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the book records
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM commodities ", 
         ""].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the commodities into a vector for returning the result
    let mut commodities : Vec<Commodity> = Vec::new();
    let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
        Ok( 
            Commodity{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    namespace: row.get(1)?,
                    mnemonic: row.get(2)?,
                    fullname: row.get(3)?,
                    cusip: row.get(4)?,
                    fraction: row.get(5)?,
                    quote_flag: row.get(6)?,
                    quote_source: row.get(7)?,
                    quote_tz: row.get(8)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the results vector
    for row in mapped_rows {
        commodities.push(row?);
    }    

    Ok(commodities)
}

///retrieve_by_guid retrieves a commodity by it's guid.
pub fn retrieve_by_guid(file_path : &str, incoming_guid : GUID) -> Result<Vec<Commodity>> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    //Get all the commodity record fields
    let sql : String = String::from(
        ["SELECT ",&_fields()," FROM commodities ", 
         "WHERE guid=@guid"].join(""));
    let mut stmt = conn.prepare(&sql)?;
    //Get all the records into a vector for returning the result
    let mut commodities : Vec<Commodity> = Vec::new();
    let mapped_rows = stmt.query_map_named(
        named_params!{"@guid": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, |row| 
        Ok( 
            Commodity{
                    guid: dhu::convert_string_result_to_guid(row.get(0))?,
                    namespace: row.get(1)?,
                    mnemonic: row.get(2)?,
                    fullname: row.get(3)?,
                    cusip: row.get(4)?,
                    fraction: row.get(5)?,
                    quote_flag: row.get(6)?,
                    quote_source: row.get(7)?,
                    quote_tz: row.get(8)?,
            }
        )
    )?;

    //Now we can put each of the mapped row results into the results vector
    for row in mapped_rows {
        commodities.push(row?);
    }    

    Ok(commodities)
}

pub fn save_new(file_path : &str, incoming_commodity : &Commodity) -> Result<bool> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    
    let sql = 
        ["INSERT INTO commodities (", &_fields(),") values (",
         "@guid,@namespace,@mnemonic,@fullname,@cusip,",
         "@fraction,@quote_flag,@quote_source,@quote_tz",
         ")"
        ].join("");

    let result = conn.execute_named(&sql,
        named_params!{
            "@guid" : dhu::convert_guid_to_sqlite_string(
                                                incoming_commodity.guid)?,
            "@namespace" : incoming_commodity.namespace,
            "@mnemonic" : incoming_commodity.mnemonic,
            "@fullname": incoming_commodity.fullname,
            "@cusip" : incoming_commodity.cusip,
            "@fraction" : incoming_commodity.fraction,
            "@quote_flag" : incoming_commodity.quote_flag,
            "@quote_source" : incoming_commodity.quote_source,
            "@quote_tz" : incoming_commodity.quote_tz,
        }
        ).unwrap();    

    
    if result != 1 {
        panic!(format!("There were {0} record changes instead of just 1!",
                        result.to_string())
        );
    }

    Ok(true)
    
}

pub fn update_existing(file_path : &str, incoming_commodity : &Commodity) -> Result<bool> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    
    let sql = 
        ["UPDATE commodities SET ",
                                "namespace=@namespace,mnemonic=@mnemonic,",
                                "fullname=@fullname,cusip=@cusip,",
                                "fraction=@fraction,quote_flag=@quote_flag,",
                                "quote_source=@quote_source,quote_tz=@quote_tz",
        " WHERE guid=@guid"
        ].join("");

    let result = conn.execute_named(&sql,
        named_params!{
            "@guid" : dhu::convert_guid_to_sqlite_string(
                                                incoming_commodity.guid)?,
            "@namespace" : incoming_commodity.namespace,
            "@mnemonic" : incoming_commodity.mnemonic,
            "@fullname": incoming_commodity.fullname,
            "@cusip" : incoming_commodity.cusip,
            "@fraction" : incoming_commodity.fraction,
            "@quote_flag" : incoming_commodity.quote_flag,
            "@quote_source" : incoming_commodity.quote_source,
            "@quote_tz" : incoming_commodity.quote_tz,
        }
        ).unwrap();    

    
    if result != 1 {
        panic!(format!("There were {0} record changes instead of just 1!",
                        result.to_string())
        );
    }

    Ok(true)
    
}

pub fn delete_existing(file_path : &str, incoming_guid : GUID) -> Result<bool> {
    //Attempt to open the file from the given path to perform this operation
    let conn = Connection::open(file_path)?;
    
    let sql = 
        ["DELETE FROM commodities ",
        " WHERE guid=@guid"
        ].join("");

    let result = conn.execute_named(&sql,
        named_params!{
            "@guid" : dhu::convert_guid_to_sqlite_string(
                                                incoming_guid)?,
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