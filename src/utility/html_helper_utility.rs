use crate::html::accounts_screen::load_accounts_with_balances_from_memory;
use crate::html::*;

/**
html_helper_utility will be all the functions that have to do with HTML output to the form.
The only reason something should be here is if it outputs HTML to the form, so this could
be from a database call, or whatever, but it should be displayed to the end user.
Every one of these call should set the #footer, and #body to nothing first.
*/
use std::collections::HashMap;
use std::convert::TryInto;

use crate::database_tables::accounts_manager::Account;
use crate::database_tables::*;
use crate::utility::database_helper_utility as dhu;
use crate::utility::js_helper_utility as js;
use chrono::Duration;
use chrono::prelude::*;
use base64::{Engine as _, engine::general_purpose};
use wasm_bindgen::prelude::*;

use chrono::prelude::*;

use rand::prelude::*;

use uuid::*;

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
    reports_div
        .append_child(&reports_last_30_days_report_button)
        .unwrap();

    //Set the event listener
    let reports_last_30_days_report_button_on_click = Closure::wrap(Box::new(move || {
        display_last_30_days_report();
    }) as Box<dyn Fn()>);

    reports_last_30_days_report_button.set_onclick(Some(
        reports_last_30_days_report_button_on_click
            .as_ref()
            .unchecked_ref(),
    ));
    reports_last_30_days_report_button_on_click.forget();
}

/// generate_html_for_report_for_account_type generates an HTML report for the given
/// dates, and account_type, such as EXPENSE, or INCOME.
pub fn generate_html_for_report_for_account_type(
    from_date: chrono::NaiveDateTime,
    thru_date: chrono::NaiveDateTime,
    account_type: String,
) -> String {
    let report_splits = splits_manager::retrieve_splits_for_dates_report(
        from_date,
        thru_date,
        account_type.clone(),
    );
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
            let split_amount: f64 = split.quantity_num as f64 / split.quantity_denom as f64;
            if split_category == category {
                current_balance += split_amount;
            }
        }

        categories_and_balances.insert(category.to_string(), current_balance);
    }

    final_html += "<div style='display:flex;'>";
    final_html += "<div>";
    final_html += &format!("{} {}", &account_type.clone(), "Categories");
    final_html += "<ul>";
    let mut sorted: Vec<_> = categories_and_balances.iter().collect();
    sorted.sort_by_key(|a| a.0);

    for category_and_balance in sorted.clone() {
        final_html += &format!(
            "<li>{}:{}</li>",
            category_and_balance.0,
            dhu::format_money(*category_and_balance.1)
        );
    }

    final_html += "</ul>";
    let sum: f64 = categories_and_balances
        .iter()
        .map(|balance| balance.1)
        .sum();
    final_html += &format!("Total: {}", dhu::format_money(sum));

    let labels: Vec<String> = sorted
        .clone()
        .iter()
        .map(|category| format!("'{}'", &category.0))
        .collect::<Vec<String>>();

    let data: Vec<String> = sorted
        .clone()
        .iter()
        .map(|balance| format!("{:.2}", &balance.1))
        .collect::<Vec<String>>();

    let mut rng = rand::rng();
    let colors: Vec<String> = sorted
        .clone()
        .iter()
        .map(|_x| {
            format!(
                "'rgb({},{},{})'",
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255)
            )
        })
        .collect::<Vec<String>>();

    final_html += "</div>";
    final_html += "<div>";
    final_html += &format!(
        "<canvas id='{}_categories_chart' width='400' height='400'></canvas>",
        account_type.clone()
    );
    final_html += r#"<img style="display:none;" src="/" onerror="
    var ctx = document.querySelector('"#;
    final_html += &format!("#{}_categories_chart", account_type.clone());
    final_html += r#"'); 
    var chart = new Chart(ctx, {
    // The type of chart we want to create
    type: 'pie',

    // The data for our dataset
    data:{
        'labels':["#;
    final_html += &labels.join(",");
    final_html += r#"],
        'datasets':[{
                'label':'My First Dataset',
                'data':["#;
    final_html += &data.join(",");
    final_html += r#"],
                'backgroundColor':["#;
    final_html += &colors.join(",");
    final_html += r#"]
        }]
    },

    // Configuration options go here
    options: {}
    });
    "#;

    final_html += r#"" />"#;
    final_html += "</div>";
    final_html += "</div>";

    final_html += "Transactions List";
    final_html += "<ul>";

    for split in &report_splits {
        let split_amount: f64 = split.quantity_num as f64 / split.quantity_denom as f64;
        let split_amount = dhu::format_money(split_amount);
        final_html += &format!(
            "<li>{}: {} - {}:{}</li>",
            chrono::NaiveDateTime::parse_from_str(&split.post_date, "%Y%m%d%H%M%S")
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            split.description,
            split.account_name,
            split_amount
        );
    }

    final_html += "</ul>";

    return final_html;
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
        NaiveTime::from_hms_milli_opt(0, 0, 0, 000).unwrap(),
    );

    let thru_date = chrono::NaiveDateTime::new(
        NaiveDate::from_ymd_opt(
            Local::now().naive_local().date().year(),
            Local::now().naive_local().date().month(),
            Local::now().naive_local().date().day(),
        )
        .unwrap(),
        NaiveTime::from_hms_milli_opt(0, 0, 0, 000).unwrap(),
    );

    let mut final_html = String::from("");

    //First setup the Expenses Report Part
    final_html +=
        &generate_html_for_report_for_account_type(from_date, thru_date, String::from("EXPENSE"));

    //Next we setup the Income Report Part
    final_html +=
        &generate_html_for_report_for_account_type(from_date, thru_date, String::from("INCOME"));

    body_div.set_inner_html(&final_html);
}


/// load_accounts_with_balances_into_memory, creates a filereader to load the account into memory,
/// it also accepts a boolean to let you know whether to load the file contents into the body for
/// accounts afterwards.
pub fn load_accounts_with_balances_into_memory(
    file_input: web_sys::HtmlInputElement,
    load_accounts_into_body_after_load: bool,
) {
    //Check the file list from the input
    let filelist = file_input
        .files()
        .expect("Failed to get filelist from File Input!");
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

    let file_reader: web_sys::FileReader = match web_sys::FileReader::new() {
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
        //let len = array.byte_length() as usize;
        //js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));

        //Check for a valid database now that we have the bytes
        // match dhu::valid_database(array.clone()) {
        //     Ok(()) => {},
        //     Err(error_message) => {
        //         js::alert(&error_message);
        //         hide_loading_message();
        //         return;
        //     }
        // }

        if crate::DATABASE.lock().unwrap().len() > 0 {
            crate::DATABASE.lock().unwrap().clear();
        }

        crate::DATABASE
            .lock()
            .unwrap()
            .push(dhu::Database::new(array.clone()));

        //Create a new input with the filename
        let money_manager_filename_input = document_query_selector("#money_manager_filename_input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        money_manager_filename_input.set_value(&file_input.files().unwrap().get(0).unwrap().name());

        //Remove the file after we are done loading it.
        file_input.set_files(None);
        file_input.set_value("");

        hide_loading_message();
        if load_accounts_into_body_after_load {
            //load_accounts_with_balances_from_memory();
        }
    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader
        .read_as_array_buffer(&file)
        .expect("blob not readable");
    onloadend_cb.forget();
}

///wireup_controls wires up the controls for the form.
pub fn wireup_controls() {
    // Setup the version number in the title
    {
        let error_message: String = format!("was not able to find document");
        let html_document = web_sys::window()
            .expect("no global 'window' exists")
            .document()
            .expect("Should have a document on window")
            .dyn_into::<web_sys::Document>()
            .expect(&error_message);
        let application_name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        html_document.set_title(&format!("{application_name}: ({version})"));
    }

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

    let money_manager_file_input = web_sys::window()
        .expect("should have a window")
        .document()
        .expect("should have a document")
        .query_selector("#money_manager_file_input")
        .expect("should have a file input")
        .expect("should have a file input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    money_manager_file_input.set_onchange(Some(
        money_manager_file_input_on_change.as_ref().unchecked_ref(),
    ));
    money_manager_file_input_on_change.forget();

    {
        //Setup the refresh accounts handler
        let main_menu_refresh_accounts_on_click = Closure::wrap(Box::new(move || {
            load_accounts_with_balances_from_memory();
        }) as Box<dyn Fn()>);

        let main_menu_accounts = document_query_selector("#main_menu_refresh_accounts");
        main_menu_accounts.set_onclick(Some(
            main_menu_refresh_accounts_on_click.as_ref().unchecked_ref(),
        ));
        main_menu_refresh_accounts_on_click.forget();
    }

    {
        //Setup the save file handler
        let main_menu_save_file_on_click = Closure::wrap(Box::new(move || {
            save_database();
        }) as Box<dyn Fn()>);

        let main_menu_save_file = document_query_selector("#main_menu_save_file");
        main_menu_save_file
            .set_onclick(Some(main_menu_save_file_on_click.as_ref().unchecked_ref()));
        main_menu_save_file_on_click.forget();
    }

    {
        //Setup the settings button handler
        let main_menu_settings_on_click = Closure::wrap(Box::new(move || {
            //Attempt to load the settings
            if crate::DATABASE.lock().unwrap().len() < 1 {
                let slots: Vec<slots_manager::Slot> = Vec::new();
                settings_screen::load_settings_into_body(slots);
            } else {
                let slots = slots_manager::load_slots_for_name("settings".to_string())
                    .expect("Failed to load slots for the name 'settings'!");
                settings_screen::load_settings_into_body(slots);
            }
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
pub fn show_loading_message(message: String) {
    let loading_message = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("Should have a document on window")
        .create_element("div")
        .expect("should be able to create div")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("should be able to create div");

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
    loading_message
        .style()
        .set_property("display", "")
        .expect("failed to set property display.");
    loading_message
        .style()
        .set_property("width", "100vw")
        .expect("failed to set property width.");
    loading_message
        .style()
        .set_property("height", "100vh")
        .expect("failed to set property height.");
    loading_message
        .style()
        .set_property("background-color", "#0003")
        .expect("failed to set property background-color.");
    loading_message
        .style()
        .set_property("position", "absolute")
        .expect("failed to set property position.");
    loading_message
        .style()
        .set_property("left", "0")
        .expect("failed to set property left.");
    loading_message
        .style()
        .set_property("top", "0")
        .expect("failed to set property top.");

    let body = document_query_selector("#body");

    body.append_child(&loading_message)
        .expect("Failed to apppend loading message.");
}

/// dispaly_transactions_older_than_one_year is whether we should display transactions older
/// than one year.
pub fn display_transactions_older_than_one_year() -> bool {
    let slots = slots_manager::load_slots_for_name_and_string_val(
        slots_manager::SLOT_NAME_SETTINGS.to_string(),
        slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR.to_string(),
    );
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
        }
        Err(e) => {
            js::alert(&e);
        }
    }

    return false;
}

/// get_database_array gets you a Uint8Array of the database. Crashes all major browsers.
/// Currently used to pass data between the web assembly, and the javascript caller.
#[allow(dead_code)]
#[wasm_bindgen]
pub fn get_database_array() -> js_sys::Uint8Array {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        panic!("Please select a database to refresh your accounts view.");
    }

    return crate::DATABASE.lock().unwrap()[0].export();
}

/// get_database_array gets you a Uint8Array of the database. Crashes all major browsers.
/// Currently used to pass data between the web assembly, and the javascript caller.
#[allow(dead_code)]
#[wasm_bindgen]
pub fn get_database_blob() -> web_sys::Blob {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        panic!("Please select a database to refresh your accounts view.");
    }

    let blob =
        web_sys::Blob::new_with_u8_array_sequence(&crate::DATABASE.lock().unwrap()[0].export())
            .unwrap();

    return blob;
}

/// save_database allows the user to save the database to a file. Doesn't currently work in firefox android.
pub fn save_database() {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        js::alert("Please select a database to refresh your accounts view.");
        return;
    }
    let blob = crate::DATABASE.lock().unwrap()[0].export();
    let b64 = general_purpose::STANDARD_NO_PAD.encode(blob.to_vec());

    let filename = document_query_selector("#money_manager_filename_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

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

    //let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
    // let len = array.byte_length() as usize;
    // js::log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
}

/// hide_loading_message attempts to hide the loading message.
pub fn hide_loading_message() {
    let loading_message = document_query_selector("#loading_message");

    let body = document_query_selector("#body");

    body.remove_child(&loading_message)
        .expect("Failed to remove loading message.");
}

// #[allow(dead_code)]
// pub fn get_default_page_html() -> String {
//   let bytes = include_bytes!("../index.html");
//   String::from_utf8_lossy(bytes).to_string()

// }

pub fn document_query_selector(query_selector: &str) -> web_sys::HtmlElement {
    let error_message: String = format!("was not able to find {}", query_selector);

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

pub fn document_create_body_table_header(
    tag: &str,
    headers: Vec<String>,
    css_prefix: &str,
) -> web_sys::HtmlElement {
    let error_message: String = format!("was not able to create '{}'!", tag);

    //Create a header to hold the headings
    let body_table_header = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("Should have a document on window")
        .create_element(tag)
        .expect(&error_message)
        .dyn_into::<web_sys::HtmlElement>()
        .expect(&error_message);

    //Add the class list
    body_table_header
        .class_list()
        .add_1("body_table_header")
        .expect("Failed to add class to element.");

    //Next put the header items into the header
    for header in headers {
        let header_element = document_create_element("div");
        header_element.set_text_content(Some(&header));
        //Set the header css class value to be like a rust variable
        let header_css_class = format!(
            "{}_{}",
            css_prefix.replace(" ", "_").to_ascii_lowercase(),
            header.replace(" ", "_").to_ascii_lowercase()
        );
        header_element
            .class_list()
            .add_1(&header_css_class)
            .expect("Failed to add class to element.");
        body_table_header
            .append_child(&header_element)
            .expect("Failed to add header element to the header table.");
    }

    return body_table_header;
}

pub fn document_create_element(tag: &str) -> web_sys::HtmlElement {
    let error_message: String = format!("was not able to create '{}'!", tag);

    return web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("Should have a document on window")
        .create_element(tag)
        .expect(&error_message)
        .dyn_into::<web_sys::HtmlElement>()
        .expect(&error_message);
}
