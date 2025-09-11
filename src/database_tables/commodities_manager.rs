use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::utility::database_helper_utility as dhu;
use crate::utility::js_helper_utility as js;
use crate::utility::sql_helper_utility as shu;

//guid,namespace,mnemonic,fullname,cusip,fraction,quote_flag,quote_source,quote_tz

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commodity {
    pub guid: Uuid,           //guid is the GUID for this record.
    pub namespace: String,    //namespace is the namespace of the commodity. (Ex: 'CURRENCY')
    pub mnemonic: String,     //mnemonic is the commodity mnemoic. (Ex: 'USD' or 'BKC')
    pub fullname: String,     //fullname is the full name. (Ex: 'United States Dollar')
    pub cusip: String, //(Ex: 840 for USD): This is any numeric or alphanumeric code that is used to identify the commodity. The CUSIP code is a unique identifying numeric string that is associated with every stock, bond or mutual fund, and most kinds of traded options, futures and commodities.
    pub fraction: i64, //Fraction is the amount it's divisible. (Ex: 'USD' = 100)
    pub quote_flag: i32, //Quote_Flag is the quote's flag. (Ex: 'USD' = 1)
    pub quote_source: String, //Quote_Source is unknown. (Ex: 'USD' = 'currency')
    pub quote_tz: String, //(Ex: 'USD' = Empty String): The timezone to assign on the online quotes
}

pub const FIELDS: &str =
    "guid,namespace,mnemonic,fullname,cusip,fraction,quote_flag,quote_source,quote_tz";

/// retrieve_all_commodities retrieves all the commodities in the system.
pub fn retrieve_all_commodities() -> Vec<Commodity> {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        panic!("Please select a database to select from you commodities.");
    }

    //Prepare a statement
    let stmt = crate::DATABASE.lock().unwrap()[0].prepare("SELECT * FROM commodities");

    let mut commodities = Vec::new();

    while stmt.step() {
        let row = stmt.getAsObject();
        js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

        let commodity: Commodity = serde_wasm_bindgen::from_value(row.clone()).unwrap();

        commodities.push(commodity);
    }

    stmt.free();

    return commodities;
}

/// retrieve_commodity_for_guid retrieves a commodity for a guid. Will cause an exception,
/// if it fails, so be careful!
pub fn retrieve_commodity_for_guid(commodity_guid: Uuid) -> Commodity {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        panic!("Please select a database to select from you commodities.");
    }

    //Prepare a statement
    let stmt = crate::DATABASE.lock().unwrap()[0].prepare(&shu::load_commodity_for_guid());

    let binding_object =
        serde_wasm_bindgen::to_value(&vec![&dhu::convert_guid_to_sqlite_string(&commodity_guid)])
            .unwrap();

    stmt.bind(binding_object.clone());

    let mut commodities = Vec::new();

    while stmt.step() {
        let row = stmt.getAsObject();
        js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

        let commodity: Commodity = serde_wasm_bindgen::from_value(row.clone()).unwrap();

        commodities.push(commodity);
    }

    stmt.free();

    return commodities[0].clone();
}

// pub fn save_new(file_path : &str, incoming_commodity : &Commodity) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;

//     let sql =
//         ["INSERT INTO commodities (", &_fields(),") values (",
//          "@guid,@namespace,@mnemonic,@fullname,@cusip,",
//          "@fraction,@quote_flag,@quote_source,@quote_tz",
//          ")"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_commodity.guid)?,
//             "@namespace" : incoming_commodity.namespace,
//             "@mnemonic" : incoming_commodity.mnemonic,
//             "@fullname": incoming_commodity.fullname,
//             "@cusip" : incoming_commodity.cusip,
//             "@fraction" : incoming_commodity.fraction,
//             "@quote_flag" : incoming_commodity.quote_flag,
//             "@quote_source" : incoming_commodity.quote_source,
//             "@quote_tz" : incoming_commodity.quote_tz,
//         }
//         ).unwrap();

//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)

// }

// pub fn update_existing(file_path : &str, incoming_commodity : &Commodity) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;

//     let sql =
//         ["UPDATE commodities SET ",
//                                 "namespace=@namespace,mnemonic=@mnemonic,",
//                                 "fullname=@fullname,cusip=@cusip,",
//                                 "fraction=@fraction,quote_flag=@quote_flag,",
//                                 "quote_source=@quote_source,quote_tz=@quote_tz",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_commodity.guid)?,
//             "@namespace" : incoming_commodity.namespace,
//             "@mnemonic" : incoming_commodity.mnemonic,
//             "@fullname": incoming_commodity.fullname,
//             "@cusip" : incoming_commodity.cusip,
//             "@fraction" : incoming_commodity.fraction,
//             "@quote_flag" : incoming_commodity.quote_flag,
//             "@quote_source" : incoming_commodity.quote_source,
//             "@quote_tz" : incoming_commodity.quote_tz,
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
//         ["DELETE FROM commodities ",
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
