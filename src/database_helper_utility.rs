extern crate guid_create;
extern crate chrono;

use guid_create::GUID;
use chrono::prelude::*;
use rusqlite::*;
use rusqlite::types::*;

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