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

/// show_loading_message shows a loading message with the String you choose to display.
pub fn show_loading_message(message : String) {
    let loading_message = web_sys::window().expect("no global `window` exists")
                            .document().expect("Should have a document on window")
                            .create_element("div").expect("should be able to create div")
                            .dyn_into::<web_sys::HtmlElement>().expect("should be able to create div");

    loading_message.set_id("loading_message");
    loading_message
        .set_inner_html(format!(
            "<div {style}>
                {message}<br>
                {loading_spinner}
            </div>",
            style="style='display:flex;width:100%;height:100%;justify-content:center;align-items: center;'",
            message=message.as_str(),
            loading_spinner=r#"<div class="lds-spinner">
                                <div></div><div></div><div></div><div></div><div></div><div></div>
                                <div></div><div></div><div></div><div></div><div></div><div></div>
                                </div>"#
            ).as_str()
        );
    loading_message.style().set_property("display","").expect("failed to set property display.");
    loading_message.style().set_property("width","100vw").expect("failed to set property width.");
    loading_message.style().set_property("height","100vh").expect("failed to set property height.");
    loading_message.style().set_property("background-color","#0003").expect("failed to set property background-color.");
    loading_message.style().set_property("position","absolute").expect("failed to set property position.");
    loading_message.style().set_property("left","0").expect("failed to set property left.");
    loading_message.style().set_property("top","0").expect("failed to set property top.");


    let body = document_query_selector("#body".to_string());

    body.append_child(&loading_message).expect("Failed to apppend loading message.");

}

pub fn hide_loading_message() {
    let loading_message = document_query_selector("#loading_message".to_string());

    let body = document_query_selector("#body".to_string());

    body.remove_child(&loading_message).expect("Failed to remove loading message.");

}

pub fn get_default_page_html() -> String {
  let bytes = include_bytes!("index.html");
  String::from_utf8_lossy(bytes).to_string()

}

pub fn document_query_selector(query_selector : String) -> web_sys::HtmlElement {
    let error_message : String = format!("was not able to find {}",query_selector.as_str());

    return web_sys::window()
            .expect("no global 'window' exists")
            .document()
            .expect("Should have a document on window")
            .query_selector(&query_selector)
            .expect(&error_message)
            .expect(&error_message)
            .dyn_into::<web_sys::HtmlElement>()
            .expect(&error_message);

}

pub fn load_accounts_into_body(accounts : Vec<accounts_manager::Account>) {
  let body_div = web_sys::window().expect("should have a window")
                                .document().expect("should have a document")
                                .query_selector("#body")
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
