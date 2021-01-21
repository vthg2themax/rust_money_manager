// use crate::{
//     accounts_manager, books_manager, commodities_manager, database_helper_utility, 
//     html_helper_utility, versions_manager, lots_manager, slots_manager
//     };

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;
//use std::collections::HashMap;
//use chrono::prelude::*;
//use guid_create::GUID;
//use crate::database_helper_utility as dhu;



pub fn get_default_page_html() -> String {
  let bytes = include_bytes!("index.html");
  String::from_utf8_lossy(bytes).to_string()

}


// pub fn get_active_accounts_with_balances(file_path : &str) -> Result<String> {    
//     let mut return_value = String::from("");
//     let accounts = accounts_manager::retrieve_active_accounts_with_balances(file_path)
//                         .expect(&"Error Finding Accounts!");

//     return_value += "<div class='accounts_table'>";

//     for account in accounts {
//         return_value += format!(r#"
//         <div class='account_div'>
//           <div>
//             <a onclick="external.invoke('open')" data-guid='{}'>
//               {}
//             </a>
//             <label>
//           </div>          
//         </div>"#,
//           dhu::convert_guid_to_sqlite_string(account.guid).expect("Invalid GUID"),
//           account.name
//         ).as_str();

//             //     +"'>"+
//             // "</a></td>"),
//             //             String::from("<td>"),
//             //             account.account_type.to_string(),
//             //             String::from("</td>"),
//             //             String::from("<td>"),
//             //             account.description,
//             //             String::from("</td>"),
//             //             String::from("<td>"),
//             //             (account.tags.get("balance").expect("No balance tag!")).to_string(),
//             //             String::from("</td>"),
//             //             String::from("</tr>"),
//             //             String::from(" ")].join("");
        
//     }

//     return_value += "</table>";

//     Ok(return_value)
// }