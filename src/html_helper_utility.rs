use crate::accounts_manager;
use crate::wasm_bindgen::JsCast;
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

pub fn load_accounts(accounts : Vec<accounts_manager::Account>) {
  let body_div = web_sys::window().expect("should have a window")
                                .document().expect("should have a document")
                                .query_selector("#body_div")
                                .expect("query_selector should exist")
                                .expect("should have a valid element")
                                .dyn_into::<web_sys::HtmlElement>()
                                .unwrap();
  
  let mut return_value = String::from("<div id='accounts_div'>");

  for account in accounts {
    return_value += format!(r#"
        <div class='account_div'>
          <a onclick="external.invoke('open')" data-guid='{guid}'>
            {account_name}
          </a>
          <div>
            {account_type}
          </div>
          <div>
            {balance}
          </div>
        </div>"#,
          guid = account.guid,
          account_name = account.name,
          account_type = account.account_type.to_string(),
          balance = (account.tags.get("balance").expect("No balance tag!")).to_string(),
        ).as_str();
  }

  return_value += "</div>";

  body_div.set_inner_html(return_value.as_str());

}
