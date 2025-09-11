use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Book {
    pub guid: Uuid, //guid is the GUID for this book.
    pub root_account_guid: Uuid, //rootAccountGuid is the root account GUID for this book.
    pub root_template_guid: Uuid,//rootTemplateGuid is the root template's GUID.

}

pub fn _fields() -> String {
    String::from(
        ["guid,root_account_guid,root_template_guid",
         ""].join("")
         )
} 

// ///retrieve_all_books retrieves all the Book records.
// pub fn retrieve_all_books(file_path : &str) -> Result<Vec<Book>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the book records
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM books ", 
//          ""].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the books into a vector for returning the result
//     let mut books : Vec<Book> = Vec::new();
//     let mapped_rows = stmt.query_map(NO_PARAMS, |row| 
//         Ok( 
//             Book{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     root_account_guid: dhu::convert_string_result_to_guid(row.get(1))?,
//                     root_template_guid: dhu::convert_string_result_to_guid(row.get(2))?,                    
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the books vector
//     //std::result::Result<books_manager::Book, rusqlite::Error>    
//     for row in mapped_rows {
//         books.push(row?);
//     }    

//     Ok(books)
// }

// ///retrieve_by_guid retrieves a book by it's guid.
// pub fn retrieve_by_guid(file_path : &str, incoming_guid : GUID) -> Result<Vec<Book>> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
//     //Get all the book record fields
//     let sql : String = String::from(
//         ["SELECT ",&_fields()," FROM books ", 
//          "WHERE guid=@guid"].join(""));
//     let mut stmt = conn.prepare(&sql)?;
//     //Get all the records into a vector for returning the result
//     let mut books : Vec<Book> = Vec::new();
//     let mapped_rows = stmt.query_map_named(
//         named_params!{"@guid": dhu::convert_guid_to_sqlite_string(incoming_guid)? }, |row| 
//         Ok( 
//             Book{
//                     guid: dhu::convert_string_result_to_guid(row.get(0))?,
//                     root_account_guid: dhu::convert_string_result_to_guid(row.get(1))?,
//                     root_template_guid: dhu::convert_string_result_to_guid(row.get(2))?, 
//             }
//         )
//     )?;

//     //Now we can put each of the mapped row results into the results vector
//     for row in mapped_rows {
//         books.push(row?);
//     }    

//     Ok(books)
// }

// pub fn save_new(file_path : &str, incoming_book : &Book) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["INSERT INTO books (", &_fields(),") values (",
//          "@guid,@root_account_guid,@root_template_guid )"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_book.guid)?,
//             "@root_account_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_book.root_account_guid)?,
//             "@root_template_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_book.root_template_guid)?,
//         }
//         ).unwrap();    

    
//     if result != 1 {
//         panic!(format!("There were {0} record changes instead of just 1!",
//                         result.to_string())
//         );
//     }

//     Ok(true)
    
// }

// pub fn update_existing(file_path : &str, incoming_book : &Book) -> Result<bool> {
//     //Attempt to open the file from the given path to perform this operation
//     let conn = Connection::open(file_path)?;
    
//     let sql = 
//         ["UPDATE books SET ",
//         "                     root_account_guid=@root_account_guid, ",
//         "                     root_template_guid=@root_template_guid ",
//         " WHERE guid=@guid"
//         ].join("");

//     let result = conn.execute_named(&sql,
//         named_params!{
//             "@guid" : dhu::convert_guid_to_sqlite_string(
//                                                 incoming_book.guid)?,
//             "@root_account_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_book.root_account_guid)?,
//             "@root_template_guid" : dhu::convert_guid_to_sqlite_parameter(
//                                                 incoming_book.root_template_guid)?,            
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
//         ["DELETE FROM books ",
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