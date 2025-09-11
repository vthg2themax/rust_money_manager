use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{database_tables::transactions_manager, utility::{html_helper_utility::*, js_helper_utility}};
use crate::utility::database_helper_utility as dhu;


/// load_transaction_into_body loads the transactions for the given transactions into the body.
pub fn load_transactions_into_body(
    transactions_with_splits: Vec<transactions_manager::TransactionWithSplitInformation>,
) {
    //Clear out the body, and footer first
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    let account_header_div = document_create_element("div");
    account_header_div.set_id("account_header_div");
    account_header_div.set_inner_html(&format!(
        "Transactions for Account: {}<br>",
        dhu::sanitize_string(transactions_with_splits[0].clone().excluded_account_name)
    ));
    body_div
        .append_child(&account_header_div)
        .expect("Failed to append account_header_div to transactions_div!");

    //Create the transactions header first
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
                js_helper_utility::alert("The given guid is not valid!");
                return;
            }
            if dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).is_ok()
                == false
            {
                js_helper_utility::alert("The given guid is not valid!");
                return;
            }
            let txn_guid =
                dhu::convert_string_to_guid(delete_link.dataset().get("guid").unwrap()).unwrap();

            if js_helper_utility::confirm("Are you sure you want to delete this transaction?") == true {
                match transactions_manager::delete_transaction(txn_guid) {
                    Ok(_e) => {
                        js_helper_utility::alert("The transaction was successfully deleted.");
                        //Reload the transactions to see our newly entered one
                        let account_element =
                            document_query_selector("#currently_loaded_account_guid");

                        if display_transactions_older_than_one_year() {
                            load_transactions_for_account_into_body_for_all_time(
                                account_element
                                    .dataset()
                                    .get("guid")
                                    .expect("Failed to find account for given guid."),
                            );
                        } else {
                            load_transactions_for_account_into_body_for_one_year_from_memory(
                                account_element
                                    .dataset()
                                    .get("guid")
                                    .expect("Failed to find account for given guid."),
                            );
                        }

                        //Clear the transaction editor now
                        clear_transaction_editor();

                        //Set focus on description to continue
                        document_query_selector("#description_input")
                            .focus()
                            .expect("Failed to focus description_input!");
                    }
                    Err(e) => {
                        js_helper_utility::alert(&format!(
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
                js_helper_utility::alert(&memo);
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
