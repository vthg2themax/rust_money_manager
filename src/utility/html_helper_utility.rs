/// html_helper_utility will be all the functions that have to do with HTML output to the form.
/// The only reason something should be here is if it outputs HTML to the form, so this could
/// be from a database call, or whatever, but it should be displayed to the end user.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::database_tables::*;
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

// #[wasm_bindgen]
// pub fn load_accounts_from_file_with_balances(file_input : web_sys::HtmlInputElement) {
//     //Check the file list from the input
//     let filelist = file_input.files().expect("Failed to get filelist from File Input!");
//     //Do not allow blank inputs
//     if filelist.length() < 1 {
//         js::alert("Please select at least one file.");
//         return;
//     }
//     if filelist.get(0) == None {
//         js::alert("Please select a valid file");
//         return;
//     }
    
//     let file = filelist.get(0).expect("Failed to get File from filelist!");

//     let file_reader : web_sys::FileReader = match web_sys::FileReader::new() {
//         Ok(f) => f,
//         Err(e) => {
//             js::alert("There was an error creating a file reader");
//             js::log(&JsValue::as_string(&e).expect("error converting jsvalue to string."));
//             web_sys::FileReader::new().expect("")
//         }
//     };

//     let fr_c = file_reader.clone();
//     // create onLoadEnd callback
//     let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
//         let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
//         let len = array.byte_length() as usize;
//         js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
//         // here you can for example use the received image/png data
//         let db : dhu::Database = dhu::Database::new(array.clone());
        
//         unsafe {
//             crate::DATABASE.push(dhu::Database::new(array.clone()));
//         }

//         //Prepare a statement
//         let stmt : dhu::Statement = db.prepare(&shu::sql_load_accounts_with_balances());
//         stmt.getAsObject();

//         // Bind new values
//         stmt.bind(JsValue::from(JsValue::null()));

//         let mut accounts = Vec::new();

//         while stmt.step() {
//             let row = stmt.getAsObject();
//             //log(&("Here is a row: ".to_owned() + &stringify(row.clone()).to_owned()));

//             let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
//             let tags : serde_json::Value = serde_json::from_str(                                    
//                                         js::stringify(row.clone()).as_str()                                    
//                                 ).unwrap();

//             let balance = format!("{}",tags["balance"])
//                             .parse::<f64>()
//                             .expect("Balance is not valid!");
            
//             account.tags.insert("balance".to_string(), balance.to_string());

//             //log(format!("The balance is: {}", balance).as_str());
//             accounts.push(account);
//         }

//         stmt.free();

//         load_accounts_into_body(accounts);

//     }) as Box<dyn Fn(web_sys::ProgressEvent)>);

//     file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
//     file_reader.read_as_array_buffer(&file).expect("blob not readable");
//     onloadend_cb.forget();

// }


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

/// load_transactions_for_account_into_body loads the transactions for the given account element
/// into the body of the form for display.
pub fn load_transactions_for_account_into_body(account_element : web_sys::HtmlElement) {

    let account_guid = account_element.dataset().get("guid").expect("Expected GUID!");

    js::log(&format!("The next step is to load the transactions for account with guid:{}",account_guid));

    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database in order to view the account by the given guid.");
            return;
        }

        //Prepare a statement
        let stmt = crate::DATABASE[0].prepare(&shu::sql_load_transactions_for_account());

        let binding_object = JsValue::from_serde(
            &vec!(&account_guid,&account_guid,&account_guid, &account_guid)
        ).unwrap();

        stmt.bind(binding_object.clone());

        let mut transactions_with_splits = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let txn : transactions_manager::TransactionWithSplitInformation = row.clone().into_serde().unwrap();
                
            transactions_with_splits.push(txn);
        }

        stmt.free();
        
        load_transactions_into_body(transactions_with_splits);
    
    }
    
}

///wireup_controls wires up the controls for the form.
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
    let main_menu_accounts = document_query_selector("#main_menu_refresh_accounts");     
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


    let body = document_query_selector("#body");

    body.append_child(&loading_message).expect("Failed to apppend loading message.");

}

/// hide_loading_message attempts to hide the loading message.
pub fn hide_loading_message() {
    let loading_message = document_query_selector("#loading_message");

    let body = document_query_selector("#body");

    body.remove_child(&loading_message).expect("Failed to remove loading message.");

}

// #[allow(dead_code)]
// pub fn get_default_page_html() -> String {
//   let bytes = include_bytes!("../index.html");
//   String::from_utf8_lossy(bytes).to_string()

// }

pub fn document_query_selector(query_selector : &str) -> web_sys::HtmlElement {
    let error_message : String = format!("was not able to find {}",query_selector);

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

pub fn document_create_element(tag : &str) -> web_sys::HtmlElement {
    let error_message : String = format!("was not able to create '{}'!", tag);

    return web_sys::window().expect("no global `window` exists")
                    .document().expect("Should have a document on window")
                    .create_element(tag).expect(&error_message)
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect(&error_message);
}

/// load_transaction_into_body loads the transactions for the given transactions into the body.
pub fn load_transactions_into_body(transactions_with_splits : Vec<transactions_manager::TransactionWithSplitInformation>) {
    let body_div = document_query_selector("#body");
  
    //Clear out the body first    
    body_div.set_inner_html("");

    let transactions_div = document_create_element("div");
    transactions_div.set_id("transaction_div");
    transactions_div.class_list().add_1("body_table").expect("Failed to add class to element.");
    body_div.append_child(&transactions_div).expect("Failed to append transactions_div to body!");

    for txn in transactions_with_splits {
        //Setup the query_selector acceptable guid
        let txn_guid_selector = format!("transaction_{}", &dhu::convert_guid_to_sqlite_string(&txn.guid));

        //Create transaction div
        let transaction_div = document_create_element("div");
        transaction_div.class_list().add_1("body_row").expect("Failed to add class to element.");
        //Put it inside the transactions div
        transactions_div.append_child(&transaction_div).expect("Failed to append transaction_div to accounts_div!");

        //Setup the transaction link, and place it inside the transactions div
        let edit_link = document_create_element("a").dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        let result = match dhu::convert_string_to_date(&txn.post_date) {
            Ok(e) => {
                e
            },
            Err(ex) => {
                panic!(ex);
            }
        };

        edit_link.set_text_content(Some(&result.format("%m/%d/%Y").to_string()));
        edit_link.set_href("#");
        edit_link.set_id(&txn_guid_selector);
        edit_link.dataset().set("guid", 
                                &dhu::convert_guid_to_sqlite_string(&txn.guid))
                                .expect("Failed to set dataset's txn_guid!");
                                
        transactions_div.append_child(&edit_link).expect("Failed to append edit_link to div!");

        //Setup the edit_link handler
        let edit_link_on_click = Closure::wrap(Box::new(move || {
            let edit_link = document_query_selector(&format!("#{}",txn_guid_selector));
            //load_transaction_editor_into_body(edit_link);
        }) as Box<dyn Fn()>);

        edit_link.set_onclick(Some(edit_link_on_click.as_ref().unchecked_ref()));
        edit_link_on_click.forget();        

        //Setup the transaction description, and place it inside the account div
        let txn_description = document_create_element("div");
        txn_description.set_text_content(
            Some(format!("{}",&txn.description).as_str())
        );
        transactions_div.append_child(&txn_description).expect("Failed to append txn_description to div!");

        
    }
}

/// load_accounts_into_body loads the accounts into the body.
pub fn load_accounts_into_body(accounts : Vec<accounts_manager::Account>) {
    let body_div = document_query_selector("#body");
  
    //Clear out the body first    
    body_div.set_inner_html("");

    //Create accounts_div, and place it in the body
    let accounts_div = document_create_element("div");
    accounts_div.class_list().add_1("body_table").expect("Failed to add class to element.");
    body_div.append_child(&accounts_div).expect("Failed to append accounts_div to body!");

    for account in accounts {
        //Setup the query_selector acceptable guid
        let account_guid_selector = format!("account_{}",
                                            dhu::convert_guid_to_sqlite_string(&account.guid));

        //Create account div
        let account_div = document_create_element("div");
        account_div.class_list().add_1("body_row").expect("Failed to add class to element.");
        //Put it inside the accounts div
        accounts_div.append_child(&account_div).expect("Failed to append account_div to accounts_div!");

        //Setup the account link, and place it inside the accounts div
        let account_link = document_create_element("a").dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        account_link.set_text_content(Some(&format!("{}",account.name)));
        account_link.set_href("#");
        account_link.set_id(&account_guid_selector);
        account_link.dataset().set("guid", 
                                    &dhu::convert_guid_to_sqlite_string(&account.guid))
                                    .expect("Failed to set dataset's account.guid!");

        account_div.append_child(&account_link).expect("Failed to append account_link to account_div!");

        //Setup the account_link handler
        let account_link_on_click = Closure::wrap(Box::new(move || {
            let account_link = document_query_selector(&format!("#{}",&account_guid_selector));
            load_transactions_for_account_into_body(account_link);
        }) as Box<dyn Fn()>);

        account_link.set_onclick(Some(account_link_on_click.as_ref().unchecked_ref()));
        account_link_on_click.forget();        

        //Setup the account type, and place it inside the account div
        let account_type = document_create_element("div");
        account_type.set_text_content(
            Some(format!("{}",&account.account_type).as_str())
        );
        account_div.append_child(&account_type).expect("Failed to append account_type to account_div!");

        //Setup the account description, and place it inside the account div
        let account_description = document_create_element("div");
        account_description.set_text_content(
            Some(format!("{}",&account.description).as_str())
        );
        account_div.append_child(&account_type).expect("Failed to append account_type to account_div!");

        //Setup the account balance, and place it inside the account div
        let account_balance = document_create_element("div");
        account_balance.set_inner_html(
            &format!("{}",&account.tags.get("balance").unwrap_or(&"No balance tag!".to_string()))
        );
        account_div.append_child(&account_balance).expect("Failed to append account_balance to account_div!");
    }
}
