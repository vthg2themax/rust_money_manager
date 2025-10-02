use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::HtmlInputElement;

use crate::database_tables::transactions_manager::TransactionWithSplitInformation;
use crate::utility::js_helper_utility as js;
use crate::utility::sql_helper_utility as shu;
use crate::utility::{csv_helper_utility, database_helper_utility as dhu};
use crate::{
    database_tables::{
        accounts_manager::{self, Account},
        commodities_manager, transactions_manager,
    },
    html::{accounts_screen::load_account_editor_into_body, transactions_screen},
    utility::html_helper_utility::*,
};
use chrono::Duration;
use chrono::prelude::*;

pub fn currently_loaded_account_guid() -> Result<Uuid, String> {
    dhu::convert_string_to_guid(
        document_query_selector("#currently_loaded_account_guid")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to get #currently_loaded_account_guid on page!")
            .value()
    )
}

pub fn currently_loaded_account_guid_string() -> Result<String, String> {
    return match document_query_selector("#currently_loaded_account_guid")
            .dyn_into::<web_sys::HtmlInputElement>() {
        Ok(result) => Ok(result.value()),
        Err(_err) => Err("Failed to find currently loaded account guid string!".to_string())
    };    
}

/// load_transaction_into_body loads the transactions for the given transactions into the body.
pub fn load_transactions_into_body(transactions_with_splits: Vec<transactions_manager::TransactionWithSplitInformation>) {
    //Clear out the body, and footer first
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    // Setup the transaction header
    let transaction_header_div = document_create_element("div");
    transaction_header_div.set_id("transaction_header_div");
    transaction_header_div
        .set_attribute(
            "style",
            "display:grid;grid-template-columns:50% 50%;grid-template-rows:100%;margin-bottom:1vh;",
        )
        .expect("Failed to modify transaction_header_div style!");


    let account_name = transactions_with_splits[0].clone().excluded_account_name.to_string();
    let account_header_div = document_create_element("div");
    account_header_div.set_id("account_header_div");
    account_header_div
        .set_attribute("style", "grid-column:1;grid-row:1;justify-self:start;")
        .expect("Failed to modify account_header_div style!");
    account_header_div.set_inner_text(&format!(
        "Transactions for Account: {}",
        account_name
    ));
    transaction_header_div
        .append_child(&account_header_div)
        .expect("Failed to append account_header_div to transactions_div!");

    //Create the Export CSV Button
    let export_csv_button = document_create_element("input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to create export_csv_button!");
    export_csv_button.set_type("button");
    export_csv_button.set_id("export_csv_button");
    export_csv_button.set_value("Export CSV");

    //Setup the export csv button handler
    let account_guid_string = transactions_with_splits[0].clone().excluded_account_guid.to_string();

    let binding = Closure::once_into_js(move || {
        let account_guid = match currently_loaded_account_guid() {
            Ok(result) => result,
            Err(_e) => { panic!() }
        };
        let transactions_with_split_information = 
            transactions_manager::retrieve_transactions_with_split_information_for_account_guid_for_past_year(account_guid)
            .expect("Failed to create set binding for export!");

        csv_helper_utility::export_transactions_to_csv(account_guid_string, transactions_with_split_information);
    });
    let export_csv_button_on_click = binding.dyn_ref().expect("Failed to convert closure.");
    export_csv_button
        .add_event_listener_with_callback("click", export_csv_button_on_click)
        .expect("Failed to modify export_csv_button style!");

    export_csv_button
        .set_attribute("style", "grid-column:2;grid-row:1;justify-self:end;")
        .expect("Failed to modify export_csv_button style!");
    
    transaction_header_div
        .append_child(&export_csv_button)
        .expect("Failed to append export_csv_button to transactions_div!");

    body_div
        .append_child(&transaction_header_div)
        .expect("Failed to append transaction_header_div to transactions_div!");

    //Create the transaction list div
    {
        let headers = vec![
            "Post Date".to_string(),
            "Description".to_string(),
            "Category".to_string(),
            "Decrease".to_string(),
            "Increase".to_string(),
            "Change".to_string(),
            "Balance".to_string(),
        ];
        let header_element = document_create_body_table_header("div", headers, "transaction");
        body_div
            .append_child(&header_element)
            .expect("Failed to append header_element to body_div.");
    }

    let transactions_div = document_create_element("div");
    transactions_div.set_id("transaction_div");
    transactions_div
        .class_list()
        .add_1("body_table")
        .expect("Failed to add class to element.");
    body_div
        .append_child(&transactions_div)
        .expect("Failed to append transactions_div to body!");

    let mut balance_amount: f64 = 0.0;

    for txn in transactions_with_splits {
        //Setup the query_selector acceptable guid
        let txn_guid_selector = format!(
            "transaction_{}",
            &dhu::convert_guid_to_sqlite_string(&txn.guid)
        );

        //Create transaction div
        let transaction_div = document_create_element("div");
        transaction_div
            .class_list()
            .add_1("body_row")
            .expect("Failed to add class to element.");
        //Put it inside the transactions div
        transactions_div
            .append_child(&transaction_div)
            .expect("Failed to append transaction_div to accounts_div!");

        //Setup the transaction delete link, and place it inside the transactions div
        let delete_link = document_create_element("a")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .unwrap();
        delete_link.set_inner_html(
            "<img src='/css/fontawesome-free-5.15.3-desktop/svgs/regular/trash-alt.svg' />",
        );
        delete_link.set_href("javascript:void(0);");
        delete_link.set_id(&txn_guid_selector);
        delete_link
            .dataset()
            .set("guid", &dhu::convert_guid_to_sqlite_string(&txn.guid))
            .expect("Failed to set dataset's txn_guid!");
        delete_link
            .class_list()
            .add_1("trashcan")
            .expect("Failed to add class to element.");
        transaction_div
            .append_child(&delete_link)
            .expect("Failed to append delete_link to div!");

        //Setup the delete_link handler
        let delete_link_on_click = Closure::wrap(Box::new(move || {
            let delete_link = document_query_selector(&format!("#{}", txn_guid_selector));
            if delete_link.dataset().get("guid").is_none() == true {
                js::alert("The given guid is not valid!");
                return;
            }
            if dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).is_ok()
                == false
            {
                js::alert("The given guid is not valid!");
                return;
            }
            let txn_guid =
                dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).unwrap();

            if js::confirm("Are you sure you want to delete this transaction?")
                == true
            {
                match transactions_manager::delete_transaction(txn_guid) {
                    Ok(_e) => {
                        js::alert("The transaction was successfully deleted.");
                        //Reload the transactions to see our newly entered one
                        let account_guid_string = currently_loaded_account_guid_string().expect("Failed to find in delete_transaction!");
                        let account_guid = currently_loaded_account_guid().expect("Failed to find in delete_transaction!");

                        if display_transactions_older_than_one_year() {
                            load_transactions_for_account_into_body_for_all_time(
                                account_guid_string,
                            );
                        } else {
                            load_transactions_for_account_into_body_for_one_year_from_memory(account_guid);
                        }

                        //Clear the transaction editor now
                        clear_transaction_editor();

                        //Set focus on description to continue
                        document_query_selector("#description_input")
                            .focus()
                            .expect("Failed to focus description_input!");
                    }
                    Err(e) => {
                        js::alert(&format!(
                            "There was an error deleting the transaction. {}",
                            e
                        ));
                    }
                }
            }
            //load_transaction_editor_into_body(edit_link);
        }) as Box<dyn Fn()>);

        delete_link.set_onclick(Some(delete_link_on_click.as_ref().unchecked_ref()));
        delete_link_on_click.forget();

        //Setup the transaction date
        let txn_date = document_create_element("div");
        let result = match dhu::convert_string_to_date(&txn.post_date) {
            Ok(e) => e,
            Err(_ex) => NaiveDateTime::new(
                NaiveDate::from_ymd_opt(0, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
        };
        txn_date.set_text_content(Some(&result.format("%m/%d/%Y").to_string()));
        txn_date
            .class_list()
            .add_1("transaction_post_date")
            .expect("Failed to add class to element.");
        transaction_div
            .append_child(&txn_date)
            .expect("Failed to append txn_date!");

        //Setup the transaction description, and place it inside the transaction div
        let txn_description = document_create_element("a")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .unwrap();
        txn_description.set_text_content(Some(format!("{}", &txn.description).as_str()));
        txn_description
            .class_list()
            .add_1("transaction_description")
            .expect("Failed to add class to element.");
        transaction_div
            .append_child(&txn_description)
            .expect("Failed to append txn_description to div!");

        let txn_memo: String = String::from(&txn.memo.clone());
        if txn_memo != "" {
            //Setup the transaction memo, and place it as a hyperlink for the description
            txn_description.set_href("javascript:void(0);");
            let txn_description_on_click = Closure::wrap(Box::new(move || {
                let memo = dhu::sanitize_string(txn_memo.clone());
                js::alert(&memo);
            }) as Box<dyn Fn()>);

            txn_description.set_onclick(Some(txn_description_on_click.as_ref().unchecked_ref()));
            txn_description_on_click.forget();
        }

        //Setup the transaction category
        let txn_category = document_create_element("div");
        txn_category.set_text_content(Some(&format!("{}", &txn.account_name)));
        txn_category
            .class_list()
            .add_1("transaction_category")
            .expect("Failed to add class to element.");
        transaction_div
            .append_child(&txn_category)
            .expect("Failed to append txn_category to div!");

        //Setup the Decrease column
        let txn_decrease = document_create_element("div");
        txn_decrease.set_text_content(Some(&format!("{}", "0.00")));
        txn_decrease
            .class_list()
            .add_1("transaction_decrease")
            .expect("failed to decrease");
        transaction_div
            .append_child(&txn_decrease)
            .expect("Failed to append txn_decrease to div!");

        //Setup the Increase column
        let txn_increase = document_create_element("div");
        txn_increase.set_text_content(Some(&format!("{}", "0.00")));
        txn_increase
            .class_list()
            .add_1("transaction_increase")
            .expect("failed to increase");
        transaction_div
            .append_child(&txn_increase)
            .expect("Failed to append txn_increase to div!");

        //Setup the amount, it's negative because we are looking at the other end of the split
        let amount: f64 = (txn.value_num as f64 / txn.value_denom as f64) * -1.0;

        //Setup the change amount, it's negative because we are looking at the other end of the split
        let txn_change = document_create_element("div");
        if txn.excluded_account_mnemonic == "USD" {
            txn_change.set_text_content(Some(&format!("{}", dhu::format_money(amount))));
        } else {
            txn_change.set_text_content(Some(&format!("{}", amount)));
        }
        txn_change
            .class_list()
            .add_1("transaction_change")
            .expect("failed to add class to change");
        transaction_div
            .append_child(&txn_change)
            .expect("Failed to append txn_increase to div!");

        //Update the balance
        balance_amount = balance_amount + amount;

        //Setup the Balance Column
        let txn_balance = document_create_element("div");
        if txn.excluded_account_mnemonic == "USD" {
            txn_balance.set_text_content(Some(&format!("{}", dhu::format_money(balance_amount))));
        } else {
            txn_balance.set_text_content(Some(&format!("{}", balance_amount)));
        }
        txn_balance
            .class_list()
            .add_1("transaction_balance")
            .expect("Failed to add class to element.");
        transaction_div
            .append_child(&txn_balance)
            .expect("Failed to append txn_balance to div!");

        //If amount is positive then setup the positive amounts
        if amount >= 0.0 {
            if txn.excluded_account_mnemonic == "USD" {
                txn_increase.set_text_content(Some(&format!("{}", dhu::format_money(amount))));
            } else {
                txn_increase.set_text_content(Some(&format!("{}", amount)));
            }
        } else {
            //Otherwise we setup the negative amounts
            if txn.excluded_account_mnemonic == "USD" {
                txn_decrease.set_text_content(Some(&format!("{}", dhu::format_money(amount))));
            } else {
                txn_decrease.set_text_content(Some(&format!("{}", amount)));
            }
        }
    }
}

/// load_accounts_into_body loads the accounts into the body.
pub fn load_accounts_into_body(accounts: Vec<Account>) {
    //Clear out the body, and footer first
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the New Account Button
    let new_account_button = document_create_element("input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to create New Account Button!");
    new_account_button.set_type("button");
    new_account_button.set_id("new_account_button");
    new_account_button.set_value("New Account");

    //Setup the new_account_button handler
    let new_account_button_on_click = Closure::wrap(Box::new(move || {
        let guid = Uuid::new_v4();
        load_account_editor_into_body(guid);
    }) as Box<dyn Fn()>);

    new_account_button.set_onclick(Some(new_account_button_on_click.as_ref().unchecked_ref()));
    new_account_button_on_click.forget();

    body_div
        .append_child(&new_account_button)
        .expect("Failed to append New Account Button!");

    //Create the header for the body
    {
        let headings = vec![
            "Name".to_string(),
            "Type".to_string(),
            "Description".to_string(),
            "Balance".to_string(),
        ];
        let accounts_header = document_create_body_table_header("div", headings, "account");

        body_div
            .append_child(&accounts_header)
            .expect("Failed to append accounts_header to body!");
    }

    //Create accounts_div, and place it in the body
    let accounts_div = document_create_element("div");
    accounts_div.set_id("accounts_div");
    accounts_div
        .class_list()
        .add_1("body_table")
        .expect("Failed to add class to element.");
    body_div
        .append_child(&accounts_div)
        .expect("Failed to append accounts_div to body!");

    for account in accounts {
        //Setup the query_selector acceptable guid
        let account_guid_selector = format!(
            "account_{}",
            dhu::convert_guid_to_sqlite_string(&account.guid)
        );

        //Setup the account_guid
        let account_guid = account.clone().guid;
        let account_guid_string = dhu::convert_guid_to_sqlite_string(&account_guid);

        //Create account div
        let account_div = document_create_element("div");
        account_div
            .class_list()
            .add_1("body_row")
            .expect("Failed to add class to element.");
        //Put it inside the accounts div
        accounts_div
            .append_child(&account_div)
            .expect("Failed to append account_div to accounts_div!");

        //Setup the accounts edit link, and place it inside the accounts div
        let edit_link = document_create_element("a")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .unwrap();
        edit_link.set_href("javascript:void(0);");
        edit_link.set_inner_html(
            "<img src='/css/fontawesome-free-5.15.3-desktop/svgs/regular/edit.svg' />",
        );
        edit_link
            .class_list()
            .add_1("edit")
            .expect("Failed to add class to element!");
        account_div
            .append_child(&edit_link)
            .expect("Failed to append edit_link!");

        //Setup the edit_link onclick
        let edit_link_on_click = Closure::wrap(Box::new(move || {
            load_account_editor_into_body(account_guid);
        }) as Box<dyn Fn()>);
        edit_link.set_onclick(Some(edit_link_on_click.as_ref().unchecked_ref()));
        edit_link_on_click.forget();

        //Setup the account link, and place it inside the accounts div
        let account_link = document_create_element("a")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .unwrap();
        account_link.set_text_content(Some(&format!("{}", &account.clone().name)));
        account_link.set_href("javascript:void(0);");
        account_link.set_id(&account_guid_selector);
        account_link
            .dataset()
            .set("guid", &dhu::convert_guid_to_sqlite_string(&account_guid))
            .expect("Failed to set dataset's account.guid!");
        account_link
            .class_list()
            .add_1("account_name")
            .expect("Failed to add class to element.");

        account_div
            .append_child(&account_link)
            .expect("Failed to append account_link to account_div!");

        //Setup the account_link handlers
        let account_link_on_click = Closure::wrap(Box::new(move || {
            show_loading_message("Please wait while your transactions are loaded...".to_string());
            if display_transactions_older_than_one_year() {
                load_transactions_for_account_into_body_for_all_time(account_guid_string.clone());
            } else {
                load_transactions_for_account_into_body_for_one_year_from_memory(account_guid);
            }
        }) as Box<dyn Fn()>);

        //account_div.set_onclick(Some(account_link_on_click.as_ref().unchecked_ref()));
        account_link.set_onclick(Some(account_link_on_click.as_ref().unchecked_ref()));

        account_link_on_click.forget();

        //Setup the account type, and place it inside the account div
        let account_type = document_create_element("div");
        account_type.set_text_content(Some(format!("{}", &account.clone().account_type).as_str()));
        account_type
            .class_list()
            .add_1("account_type")
            .expect("Failed to add class to element.");
        account_div
            .append_child(&account_type)
            .expect("Failed to append account_type to account_div!");

        //Setup the account description, and place it inside the account div
        let account_description = document_create_element("div");
        account_description
            .set_text_content(Some(format!("{}", &account.clone().description).as_str()));
        account_description
            .class_list()
            .add_1("account_description")
            .expect("Failed to add class to element.");
        account_div
            .append_child(&account_description)
            .expect("Failed to append account_type to account_div!");

        //Setup the account balance, and place it inside the account div
        let account_balance = document_create_element("div");
        let balance = &format!(
            "{}",
            &account
                .clone()
                .tags
                .get("balance")
                .unwrap_or(&"No balance tag!".to_string())
        );
        let mnemonic = &format!(
            "{}",
            &account
                .clone()
                .tags
                .get("mnemonic")
                .unwrap_or(&"".to_string())
        );
        //js::log(&format!("mnenomic is '{}'", mnemonic));

        //unpdate the balance in a way that looks nice
        if mnemonic == "USD" {
            let balance_number = balance.parse::<f64>().unwrap_or(0.0);
            account_balance.set_inner_html(&dhu::format_money(balance_number));
        } else {
            account_balance.set_inner_html(&balance);
        }

        account_balance
            .class_list()
            .add_1("account_balance")
            .expect("Failed to add class to element.");
        account_balance
            .style()
            .set_property("text-align", "end")
            .expect("Failed to change style!");
        account_div
            .append_child(&account_balance)
            .expect("Failed to append account_balance to account_div!");
    }

    // scroll to the top of the accounts_div
    js::set_timeout(
        Closure::once_into_js(move || {
            accounts_div.set_scroll_top(0);
        }),
        250,
    );
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
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to dyn_into #change_input");
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

/// enter_transaction_on_click() handles the enter key being pressed to enter a transaction.
pub fn enter_transaction_on_click() {
    let currently_loaded_account = accounts_manager::load_account_for_guid(currently_loaded_account_guid()
        .expect("Failed to get currently loaded account_guid in enter_transaction_on_click!"));
    let post_date_date = document_query_selector("#date_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to get post_date.")
        .value();
    let post_date_time = document_query_selector("#time_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to get post_time.")
        .value();

    let post_date = dhu::convert_string_to_date(
        &(post_date_date.replace("-", "") + &post_date_time.replace("-", "").replace(":", "")),
    );
    let post_date = match post_date {
        Ok(result) => result,
        Err(e) => {
            js::alert(&e);
            dhu::null_date()
        }
    };

    //Handle a null date
    if post_date.year() < 1 {
        js::alert(&format!(
            "{}{} doesn't make a valid time.",
            post_date_date.replace("-", ""),
            post_date_time.replace("-", "").replace(":", ""),
        ));
        return;
    }

    //handle a bad amount value
    let change_input = document_query_selector("#change_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to dyn_into #change_input");

    //Clear out some spaces, dollar signs, and commas
    change_input.set_value(&change_input.value().replace(" ", ""));
    change_input.set_value(&change_input.value().replace("$", ""));
    change_input.set_value(&change_input.value().replace(",", ""));
    change_input.set_value(&change_input.value().trim());

    match change_input.value().parse::<f64>() {
        Ok(_result) => {
            //we're good!
        }
        Err(_e) => {
            js::alert(&format!(
                "The given amount '{}' is not a valid number.",
                change_input.value()
            ));
            return;
        }
    }

    let amount = change_input
        .value()
        .parse::<f64>()
        .expect("Amount number is not valid!");

    //Get the commodity for this transaction to determine the units of the denom
    let commodity = commodities_manager::retrieve_commodity_for_guid(
        currently_loaded_account
            .commodity_guid
            .expect("Missing Commodity Guid!"),
    );

    let value_num = (amount * commodity.fraction as f64).round() as i64;

    //Get the account_name, and guid from the category select
    let category_select = document_query_selector("#category_select")
        .dyn_into::<web_sys::HtmlSelectElement>()
        .expect("Failed to find category select!");
    let options: web_sys::HtmlOptionsCollection = category_select.options();
    let mut account_name: String = String::from("");
    let mut account_guid = uuid::Uuid::nil();

    for i in 0..(options.length() - 1) {
        if category_select.selected_index() == i as i32 {
            let option = options
                .item(i)
                .expect("Failed to find option!")
                .dyn_into::<web_sys::HtmlOptionElement>()
                .expect("Failed to find option!");
            account_guid = dhu::convert_string_to_guid(option.value())
                .expect("Failed to convert category guid!");
            account_name = option.text();
            break;
        }
    }

    //Get the memo entered if any
    let memo = document_query_selector("#memo_textarea")
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .expect("Failed to convert memo_textarea!")
        .value();

    let txn = transactions_manager::TransactionWithSplitInformation {
        excluded_account_guid: currently_loaded_account.guid,
        excluded_account_name: currently_loaded_account.name,
        excluded_account_mnemonic: String::from(""),
        guid: uuid::Uuid::new_v4(), //guid is the GUID for this transaction
        currency_guid: commodity.guid,
        num: String::from(""), //Num is the invoice.id that this transaction belongs to.
        post_date: dhu::convert_date_to_string_format(post_date), //post_date is the date this transaction is posted. (Ex: '20120801040000' is 'Aug 1 2012')
        enter_date: dhu::convert_date_to_string_format(chrono::Local::now().naive_local()),
        description: document_query_selector("#description_input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to find description input!")
            .value(),
        value_num: value_num, //value_num is the numerator for the transaction
        value_denom: commodity.fraction,
        account_name: account_name,
        account_guid: account_guid,
        memo: memo,
    };

    match transactions_manager::save_transaction(txn) {
        Ok(_e) => {
            //Reload the transactions to see our newly entered one
            let account_guid_string = currently_loaded_account_guid_string().expect("Failed to find in save_transaction!");
            let account_guid = currently_loaded_account_guid().expect("Failed to find in save_transaction!");

            if display_transactions_older_than_one_year() {
                load_transactions_for_account_into_body_for_all_time(account_guid_string);
            } else {
                load_transactions_for_account_into_body_for_one_year_from_memory(account_guid);
            }

            //Clear the transaction editor now
            clear_transaction_editor();

            //Set focus on description to continue
            document_query_selector("#description_input")
                .focus()
                .expect("Failed to focus description_input!");
        }
        Err(e) => {
            js::alert(&e);
        }
    }
}

/// document_create_transaction_editor creates the transaction editor as a div.
pub fn document_create_transaction_editor(
    account_guid_currently_loaded: uuid::Uuid,
    transactions_to_prefill_description_with: Vec<
        transactions_manager::TransactionWithSplitInformation,
    >,
) -> web_sys::HtmlElement {
    let error_message: String = String::from("was not able to create transaction editor!");

    let transaction_editor_div = document_create_element("div");

    //Add the class to the transaction editor
    transaction_editor_div
        .class_list()
        .add_1("transaction_editor_div")
        .expect("Failed to add class to element.");

    //create the top row of the editor
    let transaction_editor_top_row = document_create_element("div");
    transaction_editor_top_row.set_id("transaction_editor_top_row");
    transaction_editor_div
        .append_child(&transaction_editor_top_row)
        .expect(&error_message);

    // create the currently loaded account guid hidden input
    {
        let currently_loaded_account_guid_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        currently_loaded_account_guid_input.set_id("currently_loaded_account_guid");
        currently_loaded_account_guid_input.set_type("hidden");
        currently_loaded_account_guid_input.set_value(&format!(
            "{}",
            dhu::convert_guid_to_sqlite_string(&account_guid_currently_loaded)
        ));
        let _ = currently_loaded_account_guid_input.set_attribute(
            "data-guid",
            &dhu::convert_guid_to_sqlite_string(&account_guid_currently_loaded),
        );

        transaction_editor_top_row
            .append_child(&currently_loaded_account_guid_input)
            .expect(&error_message);
    }

    // create the date input
    {
        let date_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        date_input.set_id("date_input");
        date_input.set_type("date");
        date_input.set_value(
            &chrono::Local::now()
                .naive_local()
                .format("%Y-%m-%d")
                .to_string(),
        );

        transaction_editor_top_row
            .append_child(&date_input)
            .expect(&error_message);
    }

    // create the time input
    {
        let time_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        time_input.set_id("time_input");
        time_input.set_type("time");
        time_input.set_step("1");
        time_input.set_value(&format!(
            "{}",
            &chrono::Local::now()
                .naive_local()
                .format("%H:%M:%S")
                .to_string()
        ));

        transaction_editor_top_row
            .append_child(&time_input)
            .expect(&error_message);
    }

    // create the description input and on_blur handler
    {
        let description_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        description_input.set_id("description_input");
        description_input.set_type("text");
        description_input.set_placeholder("Description");
        let _ = description_input
            .set_attribute("list", "description_datalist")
            .expect(&error_message);

        transaction_editor_top_row
            .append_child(&description_input)
            .expect(&error_message);

        let description_on_blur = Closure::wrap(Box::new(move || {
            load_last_transaction_for_account();
        }) as Box<dyn Fn()>);

        description_input.set_onblur(Some(description_on_blur.as_ref().unchecked_ref()));
        description_on_blur.forget();
    }

    // Setup the description_datalist
    {
        let description_datalist = document_create_element("datalist")
            .dyn_into::<web_sys::HtmlDataListElement>()
            .expect(&error_message);
        description_datalist.set_id("description_datalist");

        let mut options_for_datalist = Vec::new();
        for txn in transactions_to_prefill_description_with {
            if !options_for_datalist.contains(&txn.description) {
                options_for_datalist.push(txn.description);
            }
        }

        for option_for_datalist in options_for_datalist {
            let option = document_create_element("option")
                .dyn_into::<web_sys::HtmlOptionElement>()
                .expect(&error_message);
            option.set_value(&dhu::sanitize_string(option_for_datalist));

            description_datalist
                .append_child(&option)
                .expect(&error_message);
        }

        transaction_editor_top_row
            .append_child(&description_datalist)
            .expect(&error_message);
    }

    // Setup the category_select
    {
        let category_select = document_create_element("select")
            .dyn_into::<web_sys::HtmlSelectElement>()
            .expect(&error_message);
        category_select.set_id("category_select");

        let mut options = Vec::new();
        //Setup the categories to choose from now
        let mut accounts =
            accounts_manager::load_all_accounts_except_root_and_template_from_memory();
        accounts.sort_by(|a, b| a.name.cmp(&b.name));
        for account in accounts {
            //Don't load the current account we are in
            if account.guid != account_guid_currently_loaded {
                let option = document_create_element("option")
                    .dyn_into::<web_sys::HtmlOptionElement>()
                    .expect(&error_message);
                option.set_value(&dhu::sanitize_string(account.guid.to_string()));
                option.set_text_content(Some(&dhu::sanitize_string(account.name)));
                if !options.contains(&option) {
                    options.push(option);
                }
            }
        }

        for option in options {
            category_select.append_child(&option).expect(&error_message);
        }

        transaction_editor_top_row
            .append_child(&category_select)
            .expect(&error_message);
    }

    //Setup the change input next
    {
        let change_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        change_input.set_id("change_input");
        change_input.set_type("tel");
        change_input.set_placeholder("Amount");

        transaction_editor_top_row
            .append_child(&change_input)
            .expect(&error_message);

        //Setup the enter_transaction handler
        let change_input_on_input = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            //let key = event.key().as_str();
            //let input: web_sys::HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            //let value: f32 = input.value().parse().unwrap();
            //js::alert(&format!("{} is the value",event.key()));
            if event.key().as_str() == "Enter" {
                document_query_selector("#enter_transaction_input").click();
            }
        })
            as Box<dyn FnMut(web_sys::KeyboardEvent)>);

        change_input.set_onkeydown(Some(change_input_on_input.as_ref().unchecked_ref()));
        change_input_on_input.forget();
    }

    //Setup the bottom row
    let transaction_editor_bottom_row = document_create_element("div");
    transaction_editor_bottom_row.set_id("transaction_editor_bottom_row");
    transaction_editor_div
        .append_child(&transaction_editor_bottom_row)
        .expect(&error_message);

    //Create the memo input next
    {
        let memo_textarea = document_create_element("textarea")
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .expect(&error_message);
        memo_textarea.set_id("memo_textarea");
        memo_textarea.set_value("");
        memo_textarea.set_placeholder("Memo");
        transaction_editor_bottom_row
            .append_child(&memo_textarea)
            .expect(&error_message);
    }

    //Create the Enter Transaction input next
    {
        let enter_transaction_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        enter_transaction_input.set_type("button");
        enter_transaction_input.set_id("enter_transaction_input");
        enter_transaction_input.set_value("Enter");
        transaction_editor_bottom_row
            .append_child(&enter_transaction_input)
            .expect(&error_message);

        //Setup the enter_transaction handler
        let enter_transaction_on_click = Closure::wrap(Box::new(move || {
            enter_transaction_on_click();
        }) as Box<dyn Fn()>);

        enter_transaction_input
            .set_onclick(Some(enter_transaction_on_click.as_ref().unchecked_ref()));
        enter_transaction_on_click.forget();
    }

    return transaction_editor_div;
}

/// load_last_transaction_for_account loads the last transaction for the account.
pub fn load_last_transaction_for_account() {
    let error_message: String = String::from("Failed to load last transaction for account");
    let currently_loaded_account_guid = document_query_selector("#currently_loaded_account_guid")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect(&error_message);
    let currently_loaded_account_guid =
        dhu::convert_string_to_guid(currently_loaded_account_guid.value()).expect(&format!(
            "The given account_guid is not valid! '{}'",
            currently_loaded_account_guid.value()
        ));

    let current_description = document_query_selector("#description_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect(&error_message)
        .value();

    let transaction = transactions_manager::retrieve_transaction_with_split_information_for_account_guid_and_description(
                                                                                                        currently_loaded_account_guid, current_description);
    //If the transaction is just one, you can continue
    if transaction.len() == 1 {
        //Setup the change amount
        let change_input = document_query_selector("#change_input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect(&error_message);
        change_input.set_value(&format!(
            "{:2}",
            (((transaction[0].value_num as f64 / transaction[0].value_denom as f64) as f64)
                * -1.00)
        ));
        //Set the category part
        let category_select = document_query_selector("#category_select")
            .dyn_into::<web_sys::HtmlSelectElement>()
            .expect(&error_message);
        let options: web_sys::HtmlOptionsCollection = category_select.options();
        for i in 0..(options.length() - 1) {
            let guid = options
                .item(i)
                .expect(&error_message)
                .dyn_into::<web_sys::HtmlOptionElement>()
                .expect(&error_message)
                .value();
            let guid = dhu::convert_string_to_guid(guid.clone()).expect(&format!(
                "Failed to convert option value '{}' to guid.",
                guid.clone()
            ));
            if guid == transaction[0].account_guid {
                category_select.set_selected_index(i as i32);
                break;
            }
        }
    }
}

/// load_transactions_for_account_into_body_for_one_year_from_memory loads the transactions for the
/// given account_guid for the last year into the body of the form for display.
pub fn load_transactions_for_account_into_body_for_one_year_from_memory(account_guid: Uuid) {
    js::log(&format!("The next step is to load the transactions for account with guid:{}",account_guid.clone().to_string()));

    let transactions_with_split_information = 
        transactions_manager::retrieve_transactions_with_split_information_for_account_guid_for_past_year(account_guid.clone())
            .expect("Failed to retrieve transactions!");

    if transactions_with_split_information.iter().count() < 1 {
        js::alert(
            "No transactions were found that matched your request. Perhaps they are more than a year old?",
        );
        return;
    }

    transactions_screen::load_transactions_into_body(transactions_with_split_information.clone());

    let footer_div = document_query_selector("#footer");
    let transaction_editor =
        document_create_transaction_editor(account_guid.clone(), transactions_with_split_information.clone());
    footer_div
        .append_child(&transaction_editor)
        .expect("Failed to setup transaction editor!");

    // scroll to the bottom of the div
    js::set_timeout(
        Closure::once_into_js(move || {
            js::scroll_to_the_bottom(document_query_selector("#transaction_div"))
        }),
        500,
    );
}

/// load_transactions_for_account_into_body_for_all_time loads the transactions for the given account
/// guid from the beginning of time into the body of the form for display
pub fn load_transactions_for_account_into_body_for_all_time(account_guid_string: String) {
    //js::log(&format!("The next step is to load the transactions for account with guid:{}",account_guid));

    if crate::DATABASE.lock().unwrap().len() == 0 {
        unsafe { js::alert("Please select a database in order to view the account by the given guid.") };
        return;
    }

    //Get the date we want to limit results to, which is the the max SQLite Date value
    let date_to_use = chrono::NaiveDateTime::new(
        NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
        NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap(),
    );

    //Get the balance, and account information for the previous year
    let mut accounts = Vec::new();
    {
        let stmt =
            crate::DATABASE.lock().unwrap()[0].prepare(&shu::load_account_with_balance_for_guid());

        let binding_object = serde_wasm_bindgen::to_value(&vec![&account_guid_string]).unwrap();

        stmt.bind(binding_object.clone());

        while stmt.step() {
            let row = stmt.getAsObject();
            //js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let mut account: Account = serde_wasm_bindgen::from_value(row.clone()).unwrap();
            let tags: serde_json::Value =
                serde_json::from_str(js::stringify(row.clone()).as_str()).unwrap();

            let balance = format!("{}", tags["balance"])
                .parse::<f64>()
                .expect("Balance is not valid!");
            account
                .tags
                .insert("balance".to_string(), balance.to_string());

            let mnemonic: String =
                dhu::remove_first_and_last_double_quotes_from_string(tags["mnemonic"].to_string());
            account
                .tags
                .insert("mnemonic".to_string(), mnemonic.clone());

            accounts.push(account);
        }

        //Free the memory for the statement, and the bindings
        stmt.free();
        stmt.freemem();

        //Exit if there were no results returned
        if accounts.len() != 1 {
            js::alert(&format!(
                "Cannot continue! There were {} accounts retrieved for guid '{}', as of '{}'.",
                accounts.len().to_string(),
                &date_to_use.to_string(),
                &account_guid_string
            ));
            return;
        }
    }

    //Next now that we have a single account record, we can continue, and get the transactions loaded for the past year
    let mut transactions_with_splits = Vec::new();

    {
        let stmt =
            crate::DATABASE.lock().unwrap()[0].prepare(&shu::load_transactions_for_account());

        let binding_object = serde_wasm_bindgen::to_value(&vec![
            &account_guid_string,
            &account_guid_string,
            &account_guid_string,
            &account_guid_string,
        ])
        .unwrap();

        stmt.bind(binding_object.clone());

        while stmt.step() {
            let row = stmt.getAsObject();
            //js::log(&("Here is a row: ".to_owned() + &js::stringify(row.clone()).to_owned()));

            let txn: transactions_manager::TransactionWithSplitInformation =
                serde_wasm_bindgen::from_value(row.clone()).unwrap();

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
    let transaction_editor =
        document_create_transaction_editor(accounts[0].guid, transactions_with_splits.clone());
    footer_div
        .append_child(&transaction_editor)
        .expect("Failed to setup transaction editor!");

    // scroll to the bottom of the div
    js::set_timeout(
        Closure::once_into_js(move || {
            js::scroll_to_the_bottom(document_query_selector("#transaction_div"))
        }),
        500,
    );
}
