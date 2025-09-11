use base64::{engine::general_purpose, Engine};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{database_tables::slots_manager, utility::{html_helper_utility::*, js_helper_utility}};
use crate::utility::database_helper_utility as dhu;


/// save_setting_for_display_transactions_older_than_one_year saves the setting
/// for displaying transactions older than one year, by deleting the named slots record by name and
/// string_val and then saving a new one with the correct value from the checkbox on the page.
pub fn save_setting_for_display_transactions_older_than_one_year() {
    let setting_checkbox =
        document_query_selector("#settings_display_transactions_older_than_one_year_checkbox")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();

    match slots_manager::save_slot_for_name_and_string_val_and_int64_val(
        slots_manager::SLOT_NAME_SETTINGS.to_string(),
        slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR.to_string(),
        if setting_checkbox.checked() { 1 } else { 0 },
    ) {
        Ok(_e) => {
            js_helper_utility::alert("Successfully saved setting!");
        }
        Err(_e) => {
            js_helper_utility::alert("Failed to save setting!");
            return;
        }
    }
}


/// load_settings_into_body loads the settings into the body from the given slots
pub fn load_settings_into_body(settings_slots: Vec<slots_manager::Slot>) {
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

    //Create a button that creates a new database
    let new_database_button = document_create_element("input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();
    new_database_button.set_type("button");
    new_database_button.set_value("Create Database");
    new_database_button.set_id("new_database_button");

    let new_database_button_on_click = Closure::wrap(Box::new(move || {
        if js_helper_utility::confirm("Are you sure you want to create a new database?") {
            let empty_database = dhu::Database::new_empty();
            let filled_database = dhu::create_default_database_tables(empty_database);
            let blob = filled_database.export();

            let b64 = general_purpose::STANDARD_NO_PAD.encode(blob.to_vec());

            let body = document_query_selector("#body");
            let div = document_create_element("div");
            div.set_inner_html(
                    &format!("<a download='MoneyManagerFile.gnucash' id='MoneyManagerFile' 
                                href='data:application/octet-stream;base64,{base64_string}' target='_self'>Download</a>",
                                base64_string = b64,
                            )
            );

            body.append_child(&div).unwrap();

            document_query_selector("#MoneyManagerFile").click();

            div.set_inner_html("");
        }
    }) as Box<dyn Fn()>);

    new_database_button.set_onclick(Some(new_database_button_on_click.as_ref().unchecked_ref()));
    new_database_button_on_click.forget();

    settings_div.append_child(&new_database_button).unwrap();

    //Then the Display Transactions Older than 1 year Setting
    let settings_display_transactions_older_than_one_year_label = document_create_element("label");
    settings_display_transactions_older_than_one_year_label
        .set_inner_html("Display Transactions Older Than One Year? ");
    settings_div
        .append_child(&settings_display_transactions_older_than_one_year_label)
        .unwrap();

    let settings_display_transactions_older_than_one_year_checkbox =
        document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
    settings_display_transactions_older_than_one_year_checkbox.set_type("checkbox");
    settings_display_transactions_older_than_one_year_checkbox
        .set_id("settings_display_transactions_older_than_one_year_checkbox");
    settings_display_transactions_older_than_one_year_label
        .append_child(&settings_display_transactions_older_than_one_year_checkbox)
        .unwrap();

    //Set the event listener
    let settings_display_transactions_older_than_one_year_checkbox_on_click =
        Closure::wrap(Box::new(move || {
            save_setting_for_display_transactions_older_than_one_year();
        }) as Box<dyn Fn()>);

    settings_display_transactions_older_than_one_year_checkbox.set_onclick(Some(
        settings_display_transactions_older_than_one_year_checkbox_on_click
            .as_ref()
            .unchecked_ref(),
    ));
    settings_display_transactions_older_than_one_year_checkbox_on_click.forget();

    for settings_slot in settings_slots {
        //Handle the setting for display transactions older than 1 year
        if settings_slot.string_val
            == slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR
        {
            if settings_slot.int64_val == 1 {
                settings_display_transactions_older_than_one_year_checkbox.set_checked(true);
            } else if settings_slot.int64_val == 0 {
                settings_display_transactions_older_than_one_year_checkbox.set_checked(false);
            } else {
                js_helper_utility::alert(&format!(
                    "The settings_slot with name of {} is invalid! Please fix this!",
                    slots_manager::SLOT_NAME_DISPLAY_TRANSACTIONS_OLDER_THAN_ONE_YEAR
                ));
                return;
            }
        }
    }
}

