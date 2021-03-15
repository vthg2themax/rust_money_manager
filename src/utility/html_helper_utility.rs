/// html_helper_utility will be all the functions that have to do with HTML output to the form.
/// The only reason something should be here is if it outputs HTML to the form, so this could
/// be from a database call, or whatever, but it should be displayed to the end user.
/// Every one of these call should set the #footer, and #body to nothing first.
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::database_tables::*;
use crate::utility::js_helper_utility as js;
use crate::utility::database_helper_utility as dhu;
use crate::utility::sql_helper_utility as shu;

use chrono::prelude::*;
use chrono::Duration;

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

/// load_settings_into_body_from_memory loads the settings from the database into 
/// slot records, and then loads those into the settings page for the page.
pub fn load_settings_into_body_from_memory() {
    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database to see your settings.");
            return;
        }
        
        //Prepare a statement
        let stmt : dhu::Statement = crate::DATABASE[0].prepare(&shu::load_settings());
        stmt.getAsObject();

        let mut slots = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let slot : slots_manager::Slot = row.clone().into_serde().unwrap();
            slots.push(slot);
        }

        stmt.free();
    
        load_settings_into_body(slots);
    
    }
}

/// save_setting_for_display_transactions_older_than_one_year saves the setting
/// for displaying transactions older than one year, by deleting the named slots record by name and 
/// string_val and then saving a new one with the correct value from the checkebox on the page.
pub fn save_setting_for_display_transactions_older_than_one_year() {
    
    let setting_checkbox = document_query_selector("#settings_display_transactions_older_than_one_year_checkbox")
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .unwrap();    


    match slots_manager::save_slot_for_name_and_string_val_and_int64_val(
                        slots_manager::SLOT_NAME_SETTINGS.to_string(),
                        slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR.to_string(),
                        if setting_checkbox.checked() { 1 } else { 0 }
                    ) {

        Ok(_e) => {
            js::alert("Successfully saved setting!");
        },
        Err(_e) => {
            js::alert("Failed to save setting!");
            return;
        }
    }

}

/// load_reports_into_body loads the reports into the body of the form
pub fn load_reports_into_body() {
    //Clear out the body, and footer first    
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the reports form first
    let reports_div = document_create_element("div");
    reports_div.set_id("reports");
    body_div.append_child(&reports_div).unwrap();

    //Then the Header
    let reports_header = document_create_element("h3");
    reports_header.set_id("reports_header");
    reports_header.set_inner_html("Reports");
    reports_div.append_child(&reports_header).unwrap();

    //Then the list of reports

    //Setup the last 30 days report
    let reports_last_30_days_report_button = document_create_element("button");
    reports_last_30_days_report_button.set_inner_html("Last 30 Days");
    reports_last_30_days_report_button.set_id("reports_last_30_days_report_button");
    reports_div.append_child(&reports_last_30_days_report_button).unwrap();

    //Set the event listener
    let reports_last_30_days_report_button_on_click = Closure::wrap(Box::new(move || {
        display_last_30_days_report();
    }) as Box<dyn Fn()>);

    reports_last_30_days_report_button.set_onclick(
        Some(reports_last_30_days_report_button_on_click.as_ref().unchecked_ref())
    );
    reports_last_30_days_report_button_on_click.forget();

}

/// display_last_30_days_report displays the last 30 days worth of data in a nice
/// format.
pub fn display_last_30_days_report() {
    //clear out the body, and footer first
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Get the date we want to limit results to start at 30 days so far
    let from_date = chrono::NaiveDateTime::new(
        Local::now().naive_local().date() + Duration::days(-30),
        NaiveTime::from_hms_milli(0, 0, 0, 000)
    );

    let thru_date = chrono::NaiveDateTime::new(NaiveDate::from_ymd
        (Local::now().naive_local().date().year(),
        Local::now().naive_local().date().month(),
        Local::now().naive_local().date().day()), 
            NaiveTime::from_hms_milli(0, 0, 0, 000)
    );

    let mut report_splits = splits_manager::retrieve_splits_for_dates_report(from_date, 
                                                                        thru_date, 
                                                                        String::from("EXPENSE"));
                                                                        

    let mut final_html = String::from("");

    //Get the categories
    let mut categories = Vec::<String>::new();    
    
    for split in &report_splits {
        let account_name = &split.account_name;
        if !categories.contains(&account_name) {
            categories.push(split.account_name.clone());
        }
    }

    //sort the categories
    categories.sort();

    //Next create a hash map with the balance values
    let mut categories_and_balances = HashMap::<String, f64>::new();
    for category in &categories {
        let mut current_balance = 0.00;
        //Now lets get those balances
        for split in &report_splits {
            let split_category = &split.account_name;
            let split_amount : f64 = split.quantity_num as f64 / split.quantity_denom as f64;
            if split_category == category {
                current_balance += split_amount;
            }
        }
        
        categories_and_balances.insert(category.to_string(), current_balance);
    }
    

    final_html += "Categories";
    final_html += "<ul>";
    let mut sorted: Vec<_> = categories_and_balances.iter().collect();
    sorted.sort_by_key(|a| a.0);
    
    for category_and_balance in sorted {
        
        final_html += &format!("<li>{}:{}</li>",
                                category_and_balance.0,
                                dhu::format_money(*category_and_balance.1));

    }

    final_html += "</ul>";
    final_html += "<ul>";

    for split in &report_splits {
        final_html += &format!("<li>{}:{}</li>",split.account_name,split.quantity_num/split.quantity_denom);

    }

    final_html += "</ul>";

    body_div.set_inner_html(&final_html);
}

/// load_settings_into_body loads teh settings into the body from the given slots
pub fn load_settings_into_body(settings_slots : Vec<slots_manager::Slot>) {
    //Clear out the body, and footer first    
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the settings form first
    let settings_div = document_create_element("div");
    settings_div.set_id("settings");
    body_div.append_child(&settings_div).unwrap();

    //Then the Header
    let settings_header = document_create_element("h3");
    settings_header.set_id("settings_header");
    settings_header.set_inner_html("Settings");
    settings_div.append_child(&settings_header).unwrap();

    //Then the Display Transactions Older than 1 year Setting
    let settings_display_transactions_older_than_one_year_label = document_create_element("label");
    settings_display_transactions_older_than_one_year_label.set_inner_html(
            "Display Transactions Older Than One Year? "
    );
    settings_div.append_child(&settings_display_transactions_older_than_one_year_label).unwrap();

    let settings_display_transactions_older_than_one_year_checkbox = document_create_element("input")
                                                                .dyn_into::<web_sys::HtmlInputElement>()
                                                                .unwrap();
    settings_display_transactions_older_than_one_year_checkbox.set_type("checkbox");
    settings_display_transactions_older_than_one_year_checkbox.set_id("settings_display_transactions_older_than_one_year_checkbox");
    settings_display_transactions_older_than_one_year_label.append_child(&settings_display_transactions_older_than_one_year_checkbox).unwrap();

    //Set the event listener
    let settings_display_transactions_older_than_one_year_checkbox_on_click = Closure::wrap(Box::new(move || {
        save_setting_for_display_transactions_older_than_one_year();
    }) as Box<dyn Fn()>);

    settings_display_transactions_older_than_one_year_checkbox.set_onclick(
        Some(settings_display_transactions_older_than_one_year_checkbox_on_click.as_ref().unchecked_ref()));
    settings_display_transactions_older_than_one_year_checkbox_on_click.forget();    

    for settings_slot in settings_slots {
        //Handle the setting for display transactions older than 1 year
        if settings_slot.string_val == slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR {
            if settings_slot.int64_val == 1 {
                settings_display_transactions_older_than_one_year_checkbox.set_checked(true);
            } else if settings_slot.int64_val == 0 {
                settings_display_transactions_older_than_one_year_checkbox.set_checked(false);
            } else {
                js::alert(&format!("The settings_slot with name of {} is invalid! Please fix this!",
                                    slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR));
                return;
            }
        }
    }
}

/// load_accounts_with_balances_from_memory loads all the accounts with balances from memory.
/// This includes transactions in the future.
pub fn load_accounts_with_balances_from_memory() {
    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database to refresh your accounts view.");
            return;
        }
        
        //Prepare a statement
        let stmt : dhu::Statement = crate::DATABASE[0].prepare(&shu::load_accounts_with_balances());
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
        // match dhu::valid_database(array.clone()) {
        //     Ok(()) => {},
        //     Err(error_message) => {
        //         js::alert(&error_message);
        //         hide_loading_message();
        //         return;
        //     }
        // }

        unsafe {
            if crate::DATABASE.len() > 0 {
                crate::DATABASE.clear();
            }
            
            crate::DATABASE.push(dhu::Database::new(array.clone()));
        }        

        hide_loading_message();
        if load_accounts_into_body_after_load {
            //load_accounts_with_balances_from_memory();
        }
        
    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&file).expect("blob not readable");
    onloadend_cb.forget();

}

/// load_transactions_for_account_into_body_for_all_time loads the transactions for the given account 
/// element from the beginning of time into the body of the form for display
pub fn load_transactions_for_account_into_body_for_all_time(account_element : web_sys::HtmlElement) {
    
    //First try to get the guid from the dataset, otherwise the value
    let account_guid = account_element.dataset().get("guid").expect("Expected GUID!");

    js::log(&format!("The next step is to load the transactions for account with guid:{}",account_guid));

    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database in order to view the account by the given guid.");
            return;
        }

        //Get the date we want to limit results to, which is the the max SQLite Date value
        let date_to_use = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                            (9999,
                            12,
                            31), 
                        NaiveTime::from_hms_milli(23, 59, 59, 999)
        );

        //Get the balance, and account information for the previous year
        let mut accounts = Vec::new();
        {
            let stmt = crate::DATABASE[0].prepare(&shu::load_account_with_balance_for_guid());
    
            let binding_object = JsValue::from_serde(
                &vec!(
                    &account_guid)
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
        
        {
            let stmt = crate::DATABASE[0].prepare(&shu::load_transactions_for_account());
                
            let binding_object = JsValue::from_serde(
                &vec!(
                    &account_guid,
                    &account_guid,
                    &account_guid,
                    &account_guid,
                )
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
            js::alert("No transactions were found.");
            return;
        }
        
        load_transactions_into_body(transactions_with_splits.clone());
    
        let footer_div = document_query_selector("#footer");
        let transaction_editor = document_create_transaction_editor(accounts[0].guid,transactions_with_splits.clone());
        footer_div.append_child(&transaction_editor).expect("Failed to setup transaction editor!");

        //scroll to the bottom of the transaction_div
        let transaction_div = document_query_selector("#transaction_div");
        transaction_div.set_scroll_top(transaction_div.scroll_height());
    }
    
}

/// load_transactions_for_account_into_body_for_one_year_from_memory loads the transactions for the 
/// given account element for the last year into the body of the form for display.
pub fn load_transactions_for_account_into_body_for_one_year_from_memory(account_element : web_sys::HtmlElement) {
    
    //First try to get the guid from the dataset, otherwise the value
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
            let stmt = crate::DATABASE[0].prepare(&shu::load_account_with_balance_for_date_and_guid());
    
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
            memo : "".to_string(),
        };

        transactions_with_splits.push(transactions_before_year);

        {
            let stmt = crate::DATABASE[0].prepare(&shu::load_transactions_for_account_between_dates());
    
            let from_date = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                                                            (Local::now().naive_local().date().year()-1,
                                                            Local::now().naive_local().date().month(),
                                                            Local::now().naive_local().date().day()), 
                                                        NaiveTime::from_hms_milli(0, 0, 0, 000)
            );

            let from_date = from_date.format("%Y-%m-%d 00:00:00").to_string();
    
            let thru_date = chrono::NaiveDateTime::new(NaiveDate::from_ymd
                                                            (9999,
                                                            12,
                                                            31), 
                                                        NaiveTime::from_hms_milli(23, 59, 59, 999)
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
        
        load_transactions_into_body(transactions_with_splits.clone());
    
        let footer_div = document_query_selector("#footer");
        let transaction_editor = document_create_transaction_editor(accounts[0].guid,transactions_with_splits.clone());
        footer_div.append_child(&transaction_editor).expect("Failed to setup transaction editor!");

        //scroll to the bottom of the transaction_div
        let transaction_div = document_query_selector("#transaction_div");
        transaction_div.set_scroll_top(transaction_div.scroll_height());
    }
    
}

///wireup_controls wires up the controls for the form.
pub fn wireup_controls() {

    //Setup the load file handler
    let main_menu_load_file = document_query_selector("#main_menu_load_file")
                                .dyn_into::<web_sys::HtmlElement>()
                                .unwrap();
    
    let main_menu_load_file_on_click = Closure::wrap(Box::new(move || {
        //Get the input we need
        let money_manager_file_input = document_query_selector("#money_manager_file_input")
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
        let money_manager_file_input = document_query_selector("#money_manager_file_input")
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

    {
        //Setup the refresh accounts handler 
        let main_menu_refresh_accounts_on_click = Closure::wrap(Box::new(move || {        
            
            load_accounts_with_balances_from_memory();

        }) as Box<dyn Fn()>);
        
        let main_menu_accounts = document_query_selector("#main_menu_refresh_accounts");     
        main_menu_accounts.set_onclick(Some(main_menu_refresh_accounts_on_click.as_ref().unchecked_ref()));
        main_menu_refresh_accounts_on_click.forget();
    }

    {
        //Setup the save file handler
        let main_menu_save_file_on_click = Closure::wrap(Box::new(move || {        
            
            save_database();

        }) as Box<dyn Fn()>);
        
        let main_menu_save_file = document_query_selector("#main_menu_save_file");     
        main_menu_save_file.set_onclick(Some(main_menu_save_file_on_click.as_ref().unchecked_ref()));
        main_menu_save_file_on_click.forget();
    }

    {
        //Setup the settings button handler
        let main_menu_settings_on_click = Closure::wrap(Box::new(move || {        
            
            load_settings_into_body_from_memory();
            
        }) as Box<dyn Fn()>);
        
        let main_menu_settings = document_query_selector("#main_menu_settings");     
        main_menu_settings.set_onclick(Some(main_menu_settings_on_click.as_ref().unchecked_ref()));
        main_menu_settings_on_click.forget();
    }
    
    {
        //Setup the reports button handler
        let main_menu_reports_on_click = Closure::wrap(Box::new(move || {        
            
            load_reports_into_body();
            
        }) as Box<dyn Fn()>);
        
        let main_menu_reports = document_query_selector("#main_menu_reports");     
        main_menu_reports.set_onclick(Some(main_menu_reports_on_click.as_ref().unchecked_ref()));
        main_menu_reports_on_click.forget();
    }
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


#[wasm_bindgen()]
pub fn load_last_transaction_for_account() {
    let error_message : String = String::from("Failed to load last transaction for account");
    let currently_loaded_account_guid = document_query_selector("#currently_loaded_account_guid").dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
    let currently_loaded_account_guid = dhu::convert_string_to_guid(currently_loaded_account_guid.value())
                                            .expect(&format!("The given account_guid is not valid! '{}'",currently_loaded_account_guid.value()));
    
    let current_description = document_query_selector("#description_input").dyn_into::<web_sys::HtmlInputElement>().expect(&error_message).value();
    
    let transaction = transactions_manager::retrieve_transaction_with_split_information_for_account_guid_and_description(
                                                                                                        currently_loaded_account_guid, current_description);
    //If the transaction is just one, you can continue
    if transaction.len() == 1 {
        //Setup the change amount
        let change_input = document_query_selector("#change_input").dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        change_input.set_value(&format!("{:2}",
                                            (((transaction[0].value_num as f64 / transaction[0].value_denom as f64) as f64)*-1.00)
                                        ) 
                                );
        //Set the category part
        let category_select = document_query_selector("#category_select").dyn_into::<web_sys::HtmlSelectElement>().expect(&error_message);
        let options : web_sys::HtmlOptionsCollection = category_select.options();
        for i in 0..(options.length() -1) {
            let guid = options.item(i).expect(&error_message).dyn_into::<web_sys::HtmlOptionElement>().expect(&error_message).value();
            let guid = dhu::convert_string_to_guid(guid.clone()).expect(&format!("Failed to convert option value '{}' to guid.", guid.clone()));
            if guid == transaction[0].account_guid {
                category_select.set_selected_index(i as i32);
                break;
            }
            
        }
    }

}

/// dispaly_transactions_older_than_one_year is whether we should display transactions older
/// than one year.
pub fn display_transactions_older_than_one_year() -> bool {

    let slots = slots_manager::load_slots_for_name_and_string_val(
                    slots_manager::SLOT_NAME_SETTINGS.to_string(), 
                    slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR.to_string());
    match slots {
        Ok(slots) => {
            
            //By default it should be false
            if slots.len() < 1 {
                return false;
            }
            //If this is 1, they want this feature.
            if slots[0].int64_val == 1 {
                return true;
            }
        },
        Err(e) => {
            js::alert(&e);
        }
    }
    
    return false;

}

/// document_create_transaction_editor creates the transaction editor as a div.
pub fn document_create_transaction_editor(account_guid_currently_loaded : uuid::Uuid, transactions_to_prefill_description_with : Vec<transactions_manager::TransactionWithSplitInformation>) -> web_sys::HtmlElement {
    let error_message : String = String::from("was not able to create transaction editor!");

    let transaction_editor_div = document_create_element("div");
    
    //Add the class to the transaction editor
    transaction_editor_div.class_list().add_1("transaction_editor_div").expect("Failed to add class to element.");

    //create the top row of the editor
    let transaction_editor_top_row = document_create_element("div");
    transaction_editor_top_row.set_id("transaction_editor_top_row");
    transaction_editor_div.append_child(&transaction_editor_top_row).expect(&error_message);

    let mut transaction_editor_top_row_html = format!("
    <input type='hidden' id='currently_loaded_account_guid' data-guid='{account_guid}' value='{account_guid}' />
    <input type='date' id='date_input' value='{date_input}' />
    <input type='time' id='time_input' step='1' value='{time_input}' />
    <input type='text' id='description_input' onblur='money_manager.load_last_transaction_for_account();' placeholder='Description' list='description_datalist' />
    <datalist id='description_datalist'>
    ",
    account_guid=&dhu::convert_guid_to_sqlite_string(&account_guid_currently_loaded),
    date_input=&chrono::Local::now().naive_local().format("%Y-%m-%d").to_string(),
    time_input=&chrono::Local::now().naive_local().format("%H:%M:%S").to_string()
    );
    for txn in transactions_to_prefill_description_with {
        let option = format!(r#"<option value="{}">"#, dhu::sanitize_string(txn.description));
        if !transaction_editor_top_row_html.contains(&option) {
            transaction_editor_top_row_html += &option;
        }
    }
    transaction_editor_top_row_html += "</datalist>";
    transaction_editor_top_row_html += "
    <select id='category_select'>";

    //Setup the categories to choose from now
    let mut accounts = accounts_manager::load_all_accounts_except_root_and_template_from_memory();
    accounts.sort_by(|a, b|a.name.cmp(&b.name));    
    for account in accounts {
        //Don't load the current account we are in
        if account.guid != account_guid_currently_loaded {
            let option = format!(r#"<option value="{}">{}</option>"#, 
                                    account.guid.to_string(),
                                    dhu::sanitize_string(account.name),
                                );
            transaction_editor_top_row_html += &option;
        }
    }

    transaction_editor_top_row_html += "
    </select>";
    transaction_editor_top_row.set_inner_html(&transaction_editor_top_row_html);

    //Setup the change input next
    {
        let change_input = document_create_element("input")
                                    .dyn_into::<web_sys::HtmlInputElement>().expect(&error_message);
        change_input.set_id("change_input");
        change_input.set_type("tel");
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

        //Setup the enter_transaction handler
        let enter_transaction_on_click = Closure::wrap(Box::new(move || {
            enter_transaction_on_click();
        }) as Box<dyn Fn()>);

        enter_transaction_input.set_onclick(Some(enter_transaction_on_click.as_ref().unchecked_ref()));
        enter_transaction_on_click.forget();        

    }

    return transaction_editor_div;

}

/// enter_transaction_on_click() handles the enter key being pressed to enter a transaction.
pub fn enter_transaction_on_click() {
    
    let currently_loaded_account_guid = dhu::convert_string_to_guid(
                                            document_query_selector("#currently_loaded_account_guid")
                                            .dyn_into::<web_sys::HtmlInputElement>()
                                            .expect("Failed to get #currently_loaded_account_guid").value()
                                        ).expect("failed to get account guid.");
    let currently_loaded_account = accounts_manager::load_account_for_guid(currently_loaded_account_guid);
    let post_date_date = document_query_selector("#date_input")
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .expect("Failed to get post_date.").value();
    let post_date_time = document_query_selector("#time_input")
                            .dyn_into::<web_sys::HtmlInputElement>()
                            .expect("Failed to get post_time.").value();

    let post_date = dhu::convert_string_to_date(
                    &(post_date_date.replace("-","") + 
                    &post_date_time.replace("-","").replace(":","")
                ));
    let post_date = match post_date {
        Ok(result) => {
            result
        },
        Err(e) => {
            js::alert(&e);                    
            dhu::null_date()
        },
    };
        
    //Handle a null date
    if post_date.year() < 1 {
        js::log(&format!("{}{} doesn't make a valid time.",
                            post_date_date.replace("-",""),
                            post_date_time.replace("-","").replace(":",""),
                        )
                );
        return;
    }

    //handle a bad amount value
    let change_input = document_query_selector("#change_input")
                            .dyn_into::<web_sys::HtmlInputElement>().expect("Failed to dyn_into #change_input");
    
    match change_input.value().parse::<f64>() {
        Ok(_result) => {
            //we're good!
        },
        Err(_e) => {
            js::alert(&format!("The given amount '{}' is not a valid number.", change_input.value()));
            return;
        }
    }

    let amount = change_input.value().parse::<f64>().expect("Amount number is not valid!");

    //Get the commodity for this transaction to determine the units of the denom
    let commodity = commodities_manager::retrieve_commodity_for_guid(
                        currently_loaded_account.commodity_guid
                        .expect("Missing Commodity Guid!")
                    );

    let value_num = (amount * commodity.fraction as f64) as i64;

    //Get the account_name, and guid from the category select
    let category_select = document_query_selector("#category_select")
                            .dyn_into::<web_sys::HtmlSelectElement>()
                            .expect("Failed to find category select!");
    let options : web_sys::HtmlOptionsCollection = category_select.options();
    let mut account_name : String = String::from("");
    let mut account_guid = uuid::Uuid::nil();

    for i in 0..(options.length() -1) {        
        if category_select.selected_index() == i as i32 {
            let option = options.item(i).expect("Failed to find option!")
                            .dyn_into::<web_sys::HtmlOptionElement>()
                            .expect("Failed to find option!");
            account_guid = dhu::convert_string_to_guid(option.value()).expect("Failed to convert category guid!");
            account_name = option.text();
            break;
        }        
    }

    //Get the memo entered if any
    let memo = document_query_selector("#memo_textarea")
                    .dyn_into::<web_sys::HtmlTextAreaElement>()
                    .expect("Failed to convert memo_textarea!").value();
    

    let txn = transactions_manager::TransactionWithSplitInformation {
        excluded_account_guid : currently_loaded_account.guid, 
        excluded_account_name : currently_loaded_account.name, 
        excluded_account_mnemonic : String::from(""), 
        guid: uuid::Uuid::new_v4(), //guid is the GUID for this transaction
        currency_guid: commodity.guid,
        num : String::from(""),//Num is the invoice.id that this transaction belongs to. 
        post_date : dhu::convert_date_to_string_format(post_date), //post_date is the date this transaction is posted. (Ex: '20120801040000' is 'Aug 1 2012')
        enter_date : dhu::convert_date_to_string_format(chrono::Local::now().naive_local()),
        description : document_query_selector("#description_input")
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .expect("Failed to find description input!").value(),
        value_num : value_num,//value_num is the numerator for the transaction
        value_denom : commodity.fraction,
        account_name : account_name,
        account_guid : account_guid,
        memo : memo,
    };

    match transactions_manager::save_transaction(txn) {
        Ok(_e) => {
            //Reload the transactions to see our newly entered one
            let account_element = document_query_selector("#currently_loaded_account_guid");
            
            if display_transactions_older_than_one_year() {
                load_transactions_for_account_into_body_for_all_time(account_element);
            } else {
                load_transactions_for_account_into_body_for_one_year_from_memory(account_element);
            }                        
            
            //Clear the transaction editor now
            clear_transaction_editor();
            
            //Set focus on description to continue
            document_query_selector("#description_input").focus().expect("Failed to focus description_input!");
        },
        Err(e) => {
            js::alert(&e);
        },
    }
}

#[wasm_bindgen]
pub fn get_database_array() -> js_sys::Uint8Array {
    unsafe {
        if crate::DATABASE.len() == 0 {
            panic!("Please select a database to refresh your accounts view.");
        }
        let blob = crate::DATABASE[0].export();
        let b64 = base64::encode(blob.to_vec());
        return blob;
    }
}

/// save_database allows the user to save the database to a file. Doesn't currently work in firefox android.
pub fn save_database() {
    unsafe {
        if crate::DATABASE.len() == 0 {
            js::alert("Please select a database to refresh your accounts view.");
            return;
        }
        let blob = crate::DATABASE[0].export();
        let b64 = base64::encode(blob.to_vec());

        let filename = document_query_selector("#money_manager_file_input")
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap().files().unwrap().get(0).unwrap().name();

        let body = document_query_selector("#body");
        let div = document_create_element("div");
        div.set_inner_html(
                &format!("<a download='{filename}' id='MoneyManagerFile' 
                            href='data:application/octet-stream;base64,{base64_string}' target='_self'>Download</a>",
                            base64_string = b64,
                            filename = filename,
                        )
        );

        body.append_child(&div).unwrap();
        
        document_query_selector("#MoneyManagerFile").click();
        
        div.set_inner_html("");
    }

    //let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
    // let len = array.byte_length() as usize;
    // js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
 
}

///clear_transaction_editor 
pub fn clear_transaction_editor() {
    
    //clear the description
    let description_input = document_query_selector("#description_input")
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .expect("Failed to dyn_into #description_input!");
    description_input.set_value("");

    //clear the change_input
    let change_input = document_query_selector("#change_input")
                            .dyn_into::<web_sys::HtmlInputElement>().expect("Failed to dyn_into #change_input");
    change_input.set_value("");

    //clear the category_select
    let category_select = document_query_selector("#category_select")
                            .dyn_into::<web_sys::HtmlSelectElement>()
                            .expect("Failed to find category select!");
    category_select.set_selected_index(0);

    //clear the memo
    let memo = document_query_selector("#memo_textarea")
                    .dyn_into::<web_sys::HtmlTextAreaElement>()
                    .expect("Failed to convert memo_textarea!");

    memo.set_value("");
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
        let delete_link = document_create_element("a").dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        let result = match dhu::convert_string_to_date(&txn.post_date) {
            Ok(e) => {
                e
            },
            Err(_ex) => {
                NaiveDateTime::new(NaiveDate::from_ymd(0,1,1),
                                    NaiveTime::from_hms(0,0,0)
                                )
            }
        };

        delete_link.set_text_content(Some(&result.format("%m/%d/%Y").to_string()));
        delete_link.set_href("#");
        delete_link.set_id(&txn_guid_selector);
        delete_link.dataset().set("guid", 
                                &dhu::convert_guid_to_sqlite_string(&txn.guid))
                                .expect("Failed to set dataset's txn_guid!");
        delete_link.class_list().add_1("transaction_post_date").expect("Failed to add class to element.");
        transaction_div.append_child(&delete_link).expect("Failed to append delete_link to div!");

        //Setup the delete_link handler
        let delete_link_on_click = Closure::wrap(Box::new(move || {
            let delete_link = document_query_selector(&format!("#{}",txn_guid_selector));
            if delete_link.dataset().get("guid").is_none() == true {
                js::alert("The given guid is not valid!");
                return;
            }
            if dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).is_ok() == false {
                js::alert("The given guid is not valid!");
                return;
            }
            let txn_guid = dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).unwrap();

            if js::confirm("Are you sure you want to delete this transaction?") == true {                
                match transactions_manager::delete_transaction(txn_guid) {
                    Ok(_e) => {
                        js::alert("The transaction was successfully deleted.");
                        //Reload the transactions to see our newly entered one
                        let account_element = document_query_selector("#currently_loaded_account_guid");
                        
                        if display_transactions_older_than_one_year() {
                            load_transactions_for_account_into_body_for_all_time(account_element);
                        } else {
                            load_transactions_for_account_into_body_for_one_year_from_memory(account_element);
                        }                        
                        
                        //Clear the transaction editor now
                        clear_transaction_editor();
                        
                        //Set focus on description to continue
                        document_query_selector("#description_input").focus().expect("Failed to focus description_input!");

                    },
                    Err(e) => {
                        js::alert(&format!("There was an error deleting the transaction. {}",e));
                    }
                }
            }
            //load_transaction_editor_into_body(edit_link);
        }) as Box<dyn Fn()>);

        delete_link.set_onclick(Some(delete_link_on_click.as_ref().unchecked_ref()));
        delete_link_on_click.forget();        

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
            if display_transactions_older_than_one_year() {
                load_transactions_for_account_into_body_for_all_time(account_link);
            } else {
                load_transactions_for_account_into_body_for_one_year_from_memory(account_link);
            }
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
