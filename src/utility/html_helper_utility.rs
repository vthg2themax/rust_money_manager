/// html_helper_utility will be all the functions that have to do with HTML output to the form.
/// The only reason something should be here is if it outputs HTML to the form, so this could
/// be from a database call, or whatever, but it should be displayed to the end user.
/// Every one of these call should set the #footer, and #body to nothing first.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::database_tables::*;
use crate::utility::js_helper_utility as js;
use crate::utility::database_helper_utility as dhu;
use crate::utility::sql_helper_utility as shu;

use chrono::prelude::*;

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
            js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                                js::stringify(row.clone()).as_str()                                    
                                            ).unwrap();

            let balance = format!("{}",
                            tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            account.tags.insert("balance".to_string(), balance.to_string());

            let mnemonic : String = dhu::remove_first_and_last_double_quotes_from_string(
                                        tags["mnemonic"].to_string()
                                    );
            account.tags.insert("mnemonic".to_string(), mnemonic.clone());

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
pub fn load_transactions_for_account_into_body_from_memory(account_element : web_sys::HtmlElement) {
    
    let account_guid = account_element.dataset().get("guid").expect("Expected GUID!");

    js::log(&format!("The next step is to load the transactions for account with guid:{}",account_guid));

    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database in order to view the account by the given guid.");
            return;
        }

        //Get the date we want to limit results to start at 1 year so far
        let date_to_use = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                            (Local::now().naive_local().date().year()-1,
                            Local::now().naive_local().date().month(),
                            Local::now().naive_local().date().day()), 
                        NaiveTime::from_hms_milli(0, 0, 0, 000)
        );

        //Get the balance, and account information for the previous year
        let mut accounts = Vec::new();
        {
            let stmt = crate::DATABASE[0].prepare(&shu::sql_load_acount_with_balance_for_date_and_guid());
    
            let binding_object = JsValue::from_serde(
                &vec!(&date_to_use.format("%Y-%m-%d 00:00:00").to_string(), &account_guid)
            ).unwrap();
    
            stmt.bind(binding_object.clone());
    
            while stmt.step() {
                let row = stmt.getAsObject();
                js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));
    
                let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
                let tags : serde_json::Value = serde_json::from_str(                                    
                                                    js::stringify(row.clone()).as_str()                                    
                                                ).unwrap();
    
                let balance = format!("{}",
                                tags["balance"])
                                .parse::<f64>()
                                .expect("Balance is not valid!");
                account.tags.insert("balance".to_string(), balance.to_string());
    
                let mnemonic : String = dhu::remove_first_and_last_double_quotes_from_string(
                                            tags["mnemonic"].to_string()
                                        );
                account.tags.insert("mnemonic".to_string(), mnemonic.clone());
    
                accounts.push(account);
            }
    
            //Free the memory for the statement, and the bindings
            stmt.free();
            stmt.freemem();
    
            //Exit if there were no results returned
            if accounts.len() != 1 {
                js::alert(&format!("Cannot continue! There were {} accounts retrieved for guid '{}', as of '{}'.",accounts.len().to_string(),&date_to_use.to_string(),&account_guid));
                return;
            }
        }

        //Next now that we have a single account record, we can continue, and get the transactions loaded for the past year
        let mut transactions_with_splits = Vec::new();
        let transactions_before_year = transactions_manager::TransactionWithSplitInformation {
            excluded_account_guid : accounts[0].guid,
            excluded_account_name : accounts[0].name.clone(),
            excluded_account_mnemonic : accounts[0].tags["mnemonic"].clone(),
            guid: uuid::Uuid::nil(),
            currency_guid: uuid::Uuid::nil(),
            num : "".to_string(),
            post_date : dhu::convert_date_to_string_format(date_to_use),
            enter_date : dhu::convert_date_to_string_format(date_to_use),
            description : format!("Balance Prior To {}", date_to_use.format("%m/%d/%Y").to_string()),
            value_num : (accounts[0].tags["balance"].parse::<f64>().unwrap()*1_000_000.0)as i64,
            value_denom : -1_000_000,
            account_name : "".to_string(),
            account_guid : uuid::Uuid::nil(),
        };

        transactions_with_splits.push(transactions_before_year);

        {
            let stmt = crate::DATABASE[0].prepare(&shu::sql_load_transactions_for_account_between_dates());
    
            let from_date = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                                                            (Local::now().naive_local().date().year()-1,
                                                            Local::now().naive_local().date().month(),
                                                            Local::now().naive_local().date().day()), 
                                                        NaiveTime::from_hms_milli(0, 0, 0, 000)
            );

            let from_date = from_date.format("%Y-%m-%d 00:00:00").to_string();
    
            let thru_date = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                                                            (Local::now().naive_local().date().year(),
                                                            Local::now().naive_local().date().month(),
                                                            Local::now().naive_local().date().day()), 
                                                        NaiveTime::from_hms_milli(23, 0, 0, 000)
            );

            let thru_date = thru_date.format("%Y-%m-%d 23:59:59").to_string();

            let binding_object = JsValue::from_serde(
                &vec!(&account_guid,&account_guid,&account_guid,&account_guid,
                    &from_date, &thru_date)
            ).unwrap();
    
            stmt.bind(binding_object.clone());
    
            while stmt.step() {
                let row = stmt.getAsObject();
                //js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));
    
                let txn : transactions_manager::TransactionWithSplitInformation = row.clone().into_serde().unwrap();
                    
                transactions_with_splits.push(txn);
            }

            //Free the memory for the statement, and the bindings
            stmt.free();
            stmt.freemem();
        }

        if transactions_with_splits.len() < 1 {
            js::alert("No transactions were found that matched your request. Perhaps they are more than a year old?");
            return;
        }
        
        load_transactions_into_body(transactions_with_splits);
    
        let footer_div = document_query_selector("#footer");
        let transaction_editor = document_create_transaction_editor();
        footer_div.append_child(&transaction_editor).expect("Failed to setup transaction editor!");

        //scroll to the bottom of the transaction_div
        let transaction_div = document_query_selector("#transaction_div");
        transaction_div.set_scroll_top(transaction_div.scroll_height());
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

pub fn document_create_transaction_editor() -> web_sys::HtmlElement {
    let error_message : String = String::from("was not able to create transaction editor!");

    //Create a header to hold the headings
    let transaction_editor_div = web_sys::window().expect("no global `window` exists")
                                    .document().expect("Should have a document on window")
                                    .create_element("div").expect(&error_message)
                                    .dyn_into::<web_sys::HtmlElement>()
                                    .expect(&error_message);
    
    //Add the class to the transaction editor
    transaction_editor_div.class_list().add_1("transaction_editor_div").expect("Failed to add class to element.");

    //create the top row of the editor
    let transaction_editor_top_row = document_create_element("div");
    transaction_editor_top_row.set_id("transaction_editor_top_row");
    transaction_editor_div.append_child(&transaction_editor_top_row).expect(&error_message);

    //create the date input
    {
        let date_input = document_create_element("input")
                            .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        date_input.set_type("date");
        date_input.set_id("date_input");
        date_input.set_value(&chrono::Local::now().naive_local().format("%Y-%m-%d").to_string());
        transaction_editor_top_row.append_child(&date_input).expect(&error_message);
    }

    //create the time input
    {
        let time_input = document_create_element("input")
                            .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        time_input.set_type("time");
        time_input.set_id("time_input");
        time_input.set_step("1");
        time_input.set_value(&chrono::Local::now().naive_local().format("%H:%M:00").to_string());
        transaction_editor_top_row.append_child(&time_input).expect(&error_message);
    }

    //create the description input
    {
        let description_input = document_create_element("input")
                                .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        description_input.set_type("text");
        description_input.set_id("description_input");
        description_input.set_value("");
        description_input.set_placeholder("Description");
        transaction_editor_top_row.append_child(&description_input).expect(&error_message);
    }
    //Create the Category input next
    {
        let category_input = document_create_element("input")
                                .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        category_input.set_type("text");
        category_input.set_id("category_input");
        category_input.set_value("");
        category_input.set_placeholder("Category");
        transaction_editor_top_row.append_child(&category_input).expect(&error_message);
    }

    //Create the Change input next
    {
        let change_input = document_create_element("input")
                                .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        change_input.set_type("tel");
        change_input.set_id("change_input");
        change_input.set_value("");
        change_input.set_placeholder("Amount");
        transaction_editor_top_row.append_child(&change_input).expect(&error_message);
    }

    //Setup the bottom row
    let transaction_editor_bottom_row = document_create_element("div");
    transaction_editor_bottom_row.set_id("transaction_editor_bottom_row");
    transaction_editor_div.append_child(&transaction_editor_bottom_row).expect(&error_message);

    //Create the memo input next
    {
        let memo_textarea = document_create_element("textarea")
                                .dyn_into::<web_sys::HtmlTextAreaElement>().expect(&error_message);
        memo_textarea.set_id("memo_textarea");
        memo_textarea.set_value("");
        memo_textarea.set_placeholder("Memo");
        transaction_editor_bottom_row.append_child(&memo_textarea).expect(&error_message);
    }

    //Create the Enter Transaction input next
    {
        let enter_transaction_input = document_create_element("input")
                                        .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        enter_transaction_input.set_type("button");
        enter_transaction_input.set_id("enter_transaction_input");
        enter_transaction_input.set_value("Enter");
        transaction_editor_bottom_row.append_child(&enter_transaction_input).expect(&error_message);
    }

    return transaction_editor_div;

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

pub fn document_create_body_table_header(tag : &str, headers : Vec<String>, css_prefix : &str) -> web_sys::HtmlElement {
    let error_message : String = format!("was not able to create '{}'!", tag);

    //Create a header to hold the headings
    let body_table_header = web_sys::window().expect("no global `window` exists")
                            .document().expect("Should have a document on window")
                            .create_element(tag).expect(&error_message)
                            .dyn_into::<web_sys::HtmlElement>()
                            .expect(&error_message);
    
    //Add the class list
    body_table_header.class_list().add_1("body_table_header").expect("Failed to add class to element.");

    //Next put the header items into the header
    for header in headers {
        let header_element = document_create_element("div");
        header_element.set_text_content(Some(&header));
        //Set the header css class value to be like a rust variable
        let header_css_class = format!("{}_{}",css_prefix.replace(" ","_").to_ascii_lowercase(),
                                                header.replace(" ","_").to_ascii_lowercase());
        header_element.class_list().add_1(&header_css_class).expect("Failed to add class to element.");
        body_table_header.append_child(&header_element).expect("Failed to add header element to the header table.");
    }

    return body_table_header;

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
    
    //Clear out the body, and footer first    
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the transactions header first
    {
        let headers = vec!("Post Date".to_string(),
                            "Description".to_string(),
                            "Category".to_string(),
                            "Decrease".to_string(),
                            "Increase".to_string(),
                            "Change".to_string(),
                            "Balance".to_string(),
                        );
        let header_element = document_create_body_table_header("div", headers, "transaction");
        body_div.append_child(&header_element).expect("Failed to append header_element to body_div.");
    }

    let transactions_div = document_create_element("div");
    transactions_div.set_id("transaction_div");
    transactions_div.class_list().add_1("body_table").expect("Failed to add class to element.");
    body_div.append_child(&transactions_div).expect("Failed to append transactions_div to body!");

    let mut balance_amount : f64 = 0.0;

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
        edit_link.class_list().add_1("transaction_post_date").expect("Failed to add class to element.");
        transaction_div.append_child(&edit_link).expect("Failed to append edit_link to div!");

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
        txn_description.class_list().add_1("transaction_description").expect("Failed to add class to element.");
        transaction_div.append_child(&txn_description).expect("Failed to append txn_description to div!");

        //Setup the transaction category
        let txn_category = document_create_element("div");
        txn_category.set_text_content(
            Some(&format!("{}",&txn.account_name))
        );
        txn_category.class_list().add_1("transaction_category").expect("Failed to add class to element.");
        transaction_div.append_child(&txn_category).expect("Failed to append txn_category to div!");
        
        //Setup the Decrease column
        let txn_decrease = document_create_element("div");
        txn_decrease.set_text_content(
            Some(&format!("{}","0.00"))
        );
        txn_decrease.class_list().add_1("transaction_decrease").expect("failed to decrease");
        transaction_div.append_child(&txn_decrease).expect("Failed to append txn_decrease to div!");
    
        //Setup the Increase column
        let txn_increase = document_create_element("div");
        txn_increase.set_text_content(
            Some(&format!("{}","0.00"))
        );
        txn_increase.class_list().add_1("transaction_increase").expect("failed to increase");
        transaction_div.append_child(&txn_increase).expect("Failed to append txn_increase to div!");

        //Setup the amount, it's negative because we are looking at the other end of the split
        let amount : f64 = (txn.value_num as f64 / txn.value_denom as f64) * -1.0;

        //Setup the change amount, it's negative because we are looking at the other end of the split
        let txn_change = document_create_element("div");
        if txn.excluded_account_mnemonic == "USD" {
            txn_change.set_text_content(
                Some(&format!("{}",dhu::format_money(amount)))
            );
        } else {
            txn_change.set_text_content(
                Some(&format!("{}",amount))
            );
        }
        txn_change.class_list().add_1("transaction_change").expect("failed to add class to change");
        transaction_div.append_child(&txn_change).expect("Failed to append txn_increase to div!");
        
        //Update the balance
        balance_amount = balance_amount + amount;

        //Setup the Balance Column
        let txn_balance = document_create_element("div");
        if txn.excluded_account_mnemonic == "USD" {
            txn_balance.set_text_content(
                Some(&format!("{}",dhu::format_money(balance_amount)))
            );
        } else {
            txn_balance.set_text_content(
                Some(&format!("{}",balance_amount))
            );
        }
        txn_balance.class_list().add_1("transaction_balance").expect("Failed to add class to element.");
        transaction_div.append_child(&txn_balance).expect("Failed to append txn_balance to div!");

        //If amount is positive then setup the positive amounts
        if amount >= 0.0 {
            if txn.excluded_account_mnemonic == "USD" {
                txn_increase.set_text_content(
                    Some(&format!("{}",dhu::format_money(amount)))
                );
            } else {
                txn_increase.set_text_content(
                    Some(&format!("{}",amount))
                );
            }
        } else {
            //Otherwise we setup the negative amounts
            if txn.excluded_account_mnemonic == "USD" {
                txn_decrease.set_text_content(
                    Some(&format!("{}",dhu::format_money(amount)))
                );
            } else {
                txn_decrease.set_text_content(
                    Some(&format!("{}",amount))
                );
            }
        }

        
    }
}

/// load_accounts_into_body loads the accounts into the body.
pub fn load_accounts_into_body(accounts : Vec<accounts_manager::Account>) {
    
    //Clear out the body, and footer first    
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the header for the body
    {
        let headings = vec!("Name".to_string(),
                            "Type".to_string(),
                            "Description".to_string(),
                            "Balance".to_string()
                        );
        let accounts_header = document_create_body_table_header("div", headings, "account");
        
        body_div.append_child(&accounts_header).expect("Failed to append accounts_header to body!");
    }

    //Create accounts_div, and place it in the body
    let accounts_div = document_create_element("div");
    accounts_div.set_id("accounts_div");
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
        account_link.class_list().add_1("account_name").expect("Failed to add class to element.");

        account_div.append_child(&account_link).expect("Failed to append account_link to account_div!");
        
        //Setup the account_link handler
        let account_link_on_click = Closure::wrap(Box::new(move || {
            show_loading_message("Please wait while your transactions are loaded...".to_string());
            let account_link = document_query_selector(&format!("#{}",&account_guid_selector));
            load_transactions_for_account_into_body_from_memory(account_link);
        }) as Box<dyn Fn()>);
        
        account_link.set_onclick(Some(account_link_on_click.as_ref().unchecked_ref()));
        account_link_on_click.forget();

        //Setup the account type, and place it inside the account div
        let account_type = document_create_element("div");
        account_type.set_text_content(
            Some(format!("{}",&account.account_type).as_str())
        );
        account_type.class_list().add_1("account_type").expect("Failed to add class to element.");
        account_div.append_child(&account_type).expect("Failed to append account_type to account_div!");

        //Setup the account description, and place it inside the account div
        let account_description = document_create_element("div");
        account_description.set_text_content(
            Some(format!("{}",&account.description).as_str())
        );
        account_description.class_list().add_1("account_description").expect("Failed to add class to element.");
        account_div.append_child(&account_description).expect("Failed to append account_type to account_div!");

        //Setup the account balance, and place it inside the account div
        let account_balance = document_create_element("div");
        let balance = &format!("{}",
                        &account.tags.get("balance").unwrap_or(&"No balance tag!".to_string()));
        let mnemonic = &format!("{}",
                        &account.tags.get("mnemonic").unwrap_or(&"".to_string()));
        js::log(&format!("mnenomic is '{}'",mnemonic));

        //unpdate the balance in a way that looks nice
        if mnemonic == "USD" {
            let balance_number = balance.parse::<f64>().unwrap_or(0.0);
            account_balance.set_inner_html(&dhu::format_money(balance_number));
        } else {
            account_balance.set_inner_html(&balance);
        }
        
        account_balance.class_list().add_1("account_balance").expect("Failed to add class to element.");
        account_balance.style().set_property("text-align","end").expect("Failed to change style!");
        account_div.append_child(&account_balance).expect("Failed to append account_balance to account_div!");
    }

    //scroll to the top of the accounts_div
    accounts_div.set_scroll_top(0);

}
