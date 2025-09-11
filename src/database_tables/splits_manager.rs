use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::js_helper_utility as js;
use crate::utility::sql_helper_utility as shu;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SplitWithTransactionInformation {
    pub guid: Uuid,                     //guid is the guid for this split.
    pub tx_guid: Uuid, //tx_GUID is the transaction guid that this split belongs to.
    pub account_guid: Uuid, //account_guid is the account guid that is in this transaction.
    pub memo: String,  //memo is the memo about this split.
    pub action: String, //action is the associated action with this split.
    pub reconcile_state: String, //reconcile_state is the whether the item has been reconciled or not as 'n' or 'y'
    pub reconcile_date: Option<String>, //reconcile_date is the datetime that this split had been reconciled. *USE 14 CHARACTERS OR LESS!*
    //This field is set as null in the database if it is not reconciled.
    pub value_num: i64,           //value_num is the value of this split.
    pub value_denom: i64, //value_denom is the denomination of this split. (Ex: 100 means divide by 100 to get the value.)
    pub quantity_num: i64, //quantity_num is the quantity of this split.
    pub quantity_denom: i64, //quantity_denom is the quantity of this split. (Ex: 100 means divide by 100 to get the value.)
    pub lot_guid: Option<String>, //lot_guid is the lot's guid of this split this is set as null in the database if not applicable.
    pub account_name: String,     //account_name is the account name for this account's guid
    #[serde(rename(deserialize = "Description"))]
    pub description: String, //description is the description for this transaction
    #[serde(rename(deserialize = "PostDate"))]
    pub post_date: String, //PostDate is the posting date
}

/// retrieve_splits_for_dates_report gives you the splits for use in making a report.
pub fn retrieve_splits_for_dates_report(
    from_date: chrono::NaiveDateTime,
    thru_date: chrono::NaiveDateTime,
    incoming_account_type: String,
) -> Vec<SplitWithTransactionInformation> {
    let mut splits = Vec::new();

    if crate::DATABASE.lock().unwrap().len() == 0 {
        panic!("Please select a database in order to continue!");
    }

    {
        let stmt =
            crate::DATABASE.lock().unwrap()[0].prepare(&shu::load_splits_for_last_30_day_report());

        let binding_object = serde_wasm_bindgen::to_value(&vec![
            &from_date.format("%Y-%m-%d 00:00:00").to_string(),
            &thru_date.format("%Y-%m-%d 23:59:59").to_string(),
            &incoming_account_type,
        ])
        .unwrap();

        stmt.bind(binding_object.clone());

        while stmt.step() {
            let row = stmt.getAsObject();
            js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let split: SplitWithTransactionInformation =
                serde_wasm_bindgen::from_value(row.clone()).unwrap();

            splits.push(split);
        }

        //Free the memory for the statement, and the bindings
        stmt.free();
        stmt.freemem();
    }

    return splits;
}
