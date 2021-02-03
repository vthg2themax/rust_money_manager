/// html_helper_utility will be all the functions that have to do with HTML output to the form.
/// The only reason something should be here is if it outputs HTML to the form, so this could
/// be from a database call, or whatever, but it should be displayed to the end user.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::database_tables::accounts_manager;
use crate::utility::js_helper_utility as js;
use crate::utility::database_helper_utility as dhu;
use crate::utility::sql_helper_utility as shu;

// use crate::{
//     accounts_manager, books_manager, commodities_manager, database_helper_utility, 
//     html_helper_utility, versions_manager, lots_manager, slots_manager
//     };

//use std::collections::HashMap;
//use chrono::prelude::*;
//use guid_create::GUID;
//use crate::database_helper_utility as dhu;

#[wasm_bindgen]
pub fn load_accounts_from_file_with_balances(file_input : web_sys::HtmlInputElement) {
    //Check the file list from the input
    let filelist = file_input.files().expect("Failed to get filelist from File Input!");
    //Do not allow blank inputs
    if filelist.length() < 1 {
        js::alert("Please select at least one file.");
        return;
    }
    if filelist.get(0) == None {
        js::alert("Please select a valid file");
        return;
    }
    
    let file = filelist.get(0).expect("Failed to get File from filelist!");

    let file_reader : web_sys::FileReader = match web_sys::FileReader::new() {
        Ok(f) => f,
        Err(e) => {
            js::alert("There was an error creating a file reader");
            js::log(&JsValue::as_string(&e).expect("error converting jsvalue to string."));
            web_sys::FileReader::new().expect("")
        }
    };

    let fr_c = file_reader.clone();
    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        let len = array.byte_length() as usize;
        js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
        // here you can for example use the received image/png data
        let db : dhu::Database = dhu::Database::new(array.clone());
        
        unsafe {
            crate::DATABASE.push(dhu::Database::new(array.clone()));
        }

        //Prepare a statement
        let stmt : dhu::Statement = db.prepare(&shu::sql_load_accounts_with_balances());
        stmt.getAsObject();

        // Bind new values
        stmt.bind(JsValue::from(JsValue::null()));

        let mut accounts = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            //log(&("Here is a row: ".to_owned() + &stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                        js::stringify(row.clone()).as_str()                                    
                                ).unwrap();

            let balance = format!("{}",tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            
            account.tags.insert("balance".to_string(), balance.to_string());

            //log(format!("The balance is: {}", balance).as_str());
            accounts.push(account);
        }

        stmt.free();

        load_accounts_into_body(accounts);

    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&file).expect("blob not readable");
    onloadend_cb.forget();

}


#[wasm_bindgen]
pub fn load_accounts_with_balances_from_memory() {
    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database to refresh your accounts view.");
            return;
        }
        
        //Prepare a statement
        let stmt : dhu::Statement = crate::DATABASE[0].prepare(&shu::sql_load_accounts_with_balances());
        stmt.getAsObject();

        let mut accounts = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            //log(&("Here is a row: ".to_owned() + &stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                                js::stringify(row.clone()).as_str()                                    
                                            ).unwrap();

            let balance = format!("{}",tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            
            account.tags.insert("balance".to_string(), balance.to_string());
    
                
            js::log(format!("The balance is: {}", balance).as_str());

            accounts.push(account);
        }

        stmt.free();
    
        load_accounts_into_body(accounts);
    
    }
}

/// load_accounts_with_balances_into_memory, creates a filereader to load the account into memory,
/// it also accepts a boolean to let you know whether to load the file contents into the body for 
/// accounts afterwards.
#[wasm_bindgen]
pub fn load_accounts_with_balances_into_memory(file_input : web_sys::HtmlInputElement, 
                                                load_accounts_into_body_after_load : bool) {
    
    //Check the file list from the input
    let filelist = file_input.files().expect("Failed to get filelist from File Input!");
    //Do not allow blank inputs
    if filelist.length() < 1 {
        js::alert("Please select at least one file.");
        return;
    }
    if filelist.get(0) == None {
        js::alert("Please select a valid file");
        return;
    }
    
    let file = filelist.get(0).expect("Failed to get File from filelist!");

    let file_reader : web_sys::FileReader = match web_sys::FileReader::new() {
        Ok(f) => f,
        Err(e) => {
            js::alert("There was an error creating a file reader");
            js::log(&JsValue::as_string(&e).expect("error converting jsvalue to string."));
            web_sys::FileReader::new().expect("")
        }
    };

    let fr_c = file_reader.clone();
    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        let len = array.byte_length() as usize;
        js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
        
        //Check for a valid database now that we have the bytes
        match dhu::valid_database(array.clone()) {
            Ok(()) => {},
            Err(error_message) => {
                js::alert(&error_message);
                hide_loading_message();
                return;
            }
        }        

        unsafe {
            if crate::DATABASE.len() > 0 {
                crate::DATABASE.clear();
            }
            
            crate::DATABASE.push(dhu::Database::new(array.clone()));
        }

        if load_accounts_into_body_after_load {
            load_accounts_with_balances_from_memory();
        }
        
    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&file).expect("blob not readable");
    onloadend_cb.forget();

}

#[wasm_bindgen]
pub fn load_account_into_body(account_element : web_sys::HtmlElement) {

    let account_guid = account_element.dataset().get("guid").expect("Expected GUID!").replace("-","");

    js::log(&format!("The next step is to load the account with guid:{}",account_guid));

    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database in order to view the account by the given guid.");
            return;
        }
        
        //     var stmt = db.prepare("SELECT * FROM accounts WHERE hidden = $hidden AND name LIKE $name");
        //     stmt.getAsObject({$hidden:1, $name:1});
        
        //Prepare a statement
        let stmt = crate::DATABASE[0].prepare(&shu::sql_load_transactions_for_account());

        let binding_object = JsValue::from_serde(&vec!(&account_guid, &account_guid)).unwrap();

        stmt.bind(binding_object.clone());

        let mut accounts = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                        js::stringify(row.clone()).as_str()                                    
                                ).unwrap();

            let balance = format!("{}",tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            
            account.tags.insert("balance".to_string(), balance.to_string());
    
                
            js::log(format!("The balance is: {}", balance).as_str());

            accounts.push(account);
        }

        stmt.free();
    
        js::log(&js::stringify(binding_object.clone()));
    
    }
    
}

///wireup_controls wires up the controls for the form.
#[wasm_bindgen]
pub fn wireup_controls() {

    //Get the elements we need
    let main_menu_load_file = web_sys::window().expect("should have a window")
                                .document().expect("should have a document")
                                .query_selector("#main_menu_load_file")
                                .expect("should have a main_menu_load_file")
                                .expect("should have a mani_menu_load_file")
                                .dyn_into::<web_sys::HtmlElement>()
                                .unwrap();
    
    let main_menu_load_file_on_click = Closure::wrap(Box::new(move || {
        //Get the input we need
        let money_manager_file_input = web_sys::window().expect("no global `window` exists")
                                        .document().expect("Should have a document on window")
                                        .query_selector("#money_manager_file_input")
                                        .expect("should have a file input")
                                        .expect("should have a file input")
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();
        money_manager_file_input.click();        
    }) as Box<dyn Fn()>);

    //Set the onClick handler for the main menu button
    main_menu_load_file.set_onclick(Some(main_menu_load_file_on_click.as_ref().unchecked_ref()));
    main_menu_load_file_on_click.forget();

    //Set the onchange handler for the money_manager_file_input        
    let money_manager_file_input_on_change = Closure::wrap(Box::new(move || {        
        //Get the input we need
        let money_manager_file_input = web_sys::window().expect("no global `window` exists")
                                        .document().expect("Should have a document on window")
                                        .query_selector("#money_manager_file_input")
                                        .expect("should have a file input")
                                        .expect("should have a file input")
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();            
        
        show_loading_message("Please wait while your file is loaded...".to_string());

        load_accounts_with_balances_into_memory(money_manager_file_input, true);

    }) as Box<dyn Fn()>);


    let money_manager_file_input = web_sys::window().expect("should have a window")
                                    .document().expect("should have a document")
                                    .query_selector("#money_manager_file_input")
                                    .expect("should have a file input")
                                    .expect("should have a file input")
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap();

    money_manager_file_input.set_onchange(Some(money_manager_file_input_on_change.as_ref().unchecked_ref()));
    money_manager_file_input_on_change.forget();

    //Setup the refresh accounts handler
    //Set the onchange handler for the money_manager_file_input        
    let main_menu_refresh_accounts_on_click = Closure::wrap(Box::new(move || {        
        
        load_accounts_with_balances_from_memory();

    }) as Box<dyn Fn()>);
    
    //Set the Accounts handler to show all the accounts
    let main_menu_accounts = web_sys::window().expect("should have a window")
                                    .document().expect("should have a document")
                                    .query_selector("#main_menu_refresh_accounts")
                                    .expect("should have a main_menu_refresh_accounts")
                                    .expect("should have a main_menu_refresh_accounts")
                                    .dyn_into::<web_sys::HtmlElement>()
                                    .unwrap();
    main_menu_accounts.set_onclick(Some(main_menu_refresh_accounts_on_click.as_ref().unchecked_ref()));
    main_menu_refresh_accounts_on_click.forget();
}

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

/// hide_loading_message attempts to hide the loading message.
pub fn hide_loading_message() {
    let loading_message = document_query_selector("#loading_message".to_string());

    let body = document_query_selector("#body".to_string());

    body.remove_child(&loading_message).expect("Failed to remove loading message.");

}

pub fn get_default_page_html() -> String {
  let bytes = include_bytes!("../index.html");
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
          <a href='#' onclick="money_manager.load_account_into_body(this);" data-guid='{guid}'>
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
