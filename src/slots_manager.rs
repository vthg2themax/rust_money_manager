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

/// A UTF-8 encoded, growable string.
///
/// The `String` type is the most common string type that has ownership over the
/// contents of the string. It has a close relationship with its borrowed
/// counterpart, the primitive [`str`].
///
/// [`str`]: ../../std/primitive.str.html
///
/// # Examples
///
/// You can create a `String` from a literal string with [`String::from`]:
///
/// ```
/// let hello = String::from("Hello, world!");
/// ```
///
/// You can append a [`char`] to a `String` with the [`push`] method, and
/// append a [`&str`] with the [`push_str`] method:
///
/// ```
/// let mut hello = String::from("Hello, ");
///
/// hello.push('w');
/// hello.push_str("orld!");
/// ```
///
/// [`String::from`]: #method.from
/// [`char`]: ../../std/primitive.char.html
/// [`push`]: #method.push
/// [`push_str`]: #method.push_str
///
/// If you have a vector of UTF-8 bytes, you can create a `String` from it with
/// the [`from_utf8`] method:
///
/// ```
/// // some bytes, in a vector
/// let sparkle_heart = vec![240, 159, 146, 150];
///
/// // We know these bytes are valid, so we'll use `unwrap()`.
/// let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
///
/// assert_eq!("💖", sparkle_heart);
/// ```
///
/// [`from_utf8`]: #method.from_utf8
///
/// # UTF-8
///
/// `String`s are always valid UTF-8. This has a few implications, the first of
/// which is that if you need a non-UTF-8 string, consider [`OsString`]. It is
/// similar, but without the UTF-8 constraint. The second implication is that
/// you cannot index into a `String`:
///
/// ```compile_fail,E0277
/// let s = "hello";
///
/// println!("The first letter of s is {}", s[0]); // ERROR!!!
/// ```
///
/// [`OsString`]: ../../std/ffi/struct.OsString.html
///
/// Indexing is intended to be a constant-time operation, but UTF-8 encoding
/// does not allow us to do this. Furthermore, it's not clear what sort of
/// thing the index should return: a byte, a codepoint, or a grapheme cluster.
/// The [`bytes`] and [`chars`] methods return iterators over the first
/// two, respectively.
///
/// [`bytes`]: #method.bytes
/// [`chars`]: #method.chars
///
/// # Deref
///
/// `String`s implement [`Deref`]`<Target=str>`, and so inherit all of [`str`]'s
/// methods. In addition, this means that you can pass a `String` to a
/// function which takes a [`&str`] by using an ampersand (`&`):
///
/// ```
/// fn takes_str(s: &str) { }
///
/// let s = String::from("Hello");
///
/// takes_str(&s);
/// ```
///
/// This will create a [`&str`] from the `String` and pass it in. This
/// conversion is very inexpensive, and so generally, functions will accept
/// [`&str`]s as arguments unless they need a `String` for some specific
/// reason.
///
/// In certain cases Rust doesn't have enough information to make this
/// conversion, known as [`Deref`] coercion. In the following example a string
/// slice [`&'a str`][`&str`] implements the trait `TraitExample`, and the function
/// `example_func` takes anything that implements the trait. In this case Rust
/// would need to make two implicit conversions, which Rust doesn't have the
/// means to do. For that reason, the following example will not compile.
///
/// ```compile_fail,E0277
/// trait TraitExample {}
///
/// impl<'a> TraitExample for &'a str {}
///
/// fn example_func<A: TraitExample>(example_arg: A) {}
///
/// fn main() {
///     let example_string = String::from("example_string");
///     example_func(&example_string);
/// }
/// ```
///
/// There are two options that would work instead. The first would be to
/// change the line `example_func(&example_string);` to
/// `example_func(example_string.as_str());`, using the method [`as_str()`]
/// to explicitly extract the string slice containing the string. The second
/// way changes `example_func(&example_string);` to
/// `example_func(&*example_string);`. In this case we are dereferencing a
/// `String` to a [`str`][`&str`], then referencing the [`str`][`&str`] back to
/// [`&str`]. The second way is more idiomatic, however both work to do the
/// conversion explicitly rather than relying on the implicit conversion.
///
/// # Representation
///
/// A `String` is made up of three components: a pointer to some bytes, a
/// length, and a capacity. The pointer points to an internal buffer `String`
/// uses to store its data. The length is the number of bytes currently stored
/// in the buffer, and the capacity is the size of the buffer in bytes. As such,
/// the length will always be less than or equal to the capacity.
///
/// This buffer is always stored on the heap.
///
/// You can look at these with the [`as_ptr`], [`len`], and [`capacity`]
/// methods:
///
/// ```
/// use std::mem;
///
/// let story = String::from("Once upon a time...");
///
/// let ptr = story.as_ptr();
/// let len = story.len();
/// let capacity = story.capacity();
///
/// // story has nineteen bytes
/// assert_eq!(19, len);
///
/// // Now that we have our parts, we throw the story away.
/// mem::forget(story);
///
/// // We can re-build a String out of ptr, len, and capacity. This is all
/// // unsafe because we are responsible for making sure the components are
/// // valid:
/// let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) } ;
///
/// assert_eq!(String::from("Once upon a time..."), s);
/// ```
///
/// [`as_ptr`]: #method.as_ptr
/// [`len`]: #method.len
/// [`capacity`]: #method.capacity
///
/// If a `String` has enough capacity, adding elements to it will not
/// re-allocate. For example, consider this program:
///
/// ```
/// let mut s = String::new();
///
/// println!("{}", s.capacity());
///
/// for _ in 0..5 {
///     s.push_str("hello");
///     println!("{}", s.capacity());
/// }
/// ```
///
/// This will output the following:
///
/// ```text
/// 0
/// 5
/// 10
/// 20
/// 20
/// 40
/// ```
///
/// At first, we have no memory allocated at all, but as we append to the
/// string, it increases its capacity appropriately. If we instead use the
/// [`with_capacity`] method to allocate the correct capacity initially:
///
/// ```
/// let mut s = String::with_capacity(25);
///
/// println!("{}", s.capacity());
///
/// for _ in 0..5 {
///     s.push_str("hello");
///     println!("{}", s.capacity());
/// }
/// ```
///
/// [`with_capacity`]: #method.with_capacity
///
/// We end up with a different output:
///
/// ```text
/// 25
/// 25
/// 25
/// 25
/// 25
/// 25
/// ```
///
/// Here, there's no need to allocate more memory inside the loop.
///
/// [`&str`]: ../../std/primitive.str.html
/// [`Deref`]: ../../std/ops/trait.Deref.html
/// [`as_str()`]: struct.String.html#method.as_str
// pub fn update_existing_multiple(file_path : &str, incoming_slots : &Vec<Slot>) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     let tx = conn.transaction();
//     let tx = match tx {
//         Ok(tx) => { tx },
//         Err(e) => {
//             return Err(Error::InvalidParameterName(
//                 format!("There was an error starting the transaction. {}",e)
//                 ));
//         },
//     };

//     for slot in incoming_slots {
//         {
            
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

//     }

//     let sql = 
        

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