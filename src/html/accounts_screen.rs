use uuid::Uuid;
use wasm_bindgen::prelude::Closure;
use std::collections::HashMap;
use std::convert::TryInto;
use crate::database_tables::{accounts_manager, commodities_manager};
use crate::html::transactions_screen::{currently_loaded_account_guid_string, load_transactions_for_account_into_body_for_all_time, load_transactions_for_account_into_body_for_one_year_from_memory};
use crate::utility::html_helper_utility::{display_transactions_older_than_one_year, document_create_body_table_header, document_create_element, document_query_selector, show_loading_message};
use crate::utility::{database_helper_utility as dhu, js_helper_utility as js, sql_helper_utility as shu};
use crate::{
    database_tables::accounts_manager::Account, utility::html_helper_utility,
};
use crate::wasm_bindgen::JsCast;

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
                load_transactions_for_account_into_body_for_all_time(dhu::convert_guid_to_sqlite_string(&account_guid));
            } else {
                load_transactions_for_account_into_body_for_one_year_from_memory(account_guid.clone());
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
        js::log(&format!("mnenomic is '{}'", mnemonic));

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

/// save_account_with_guid saves the account with a given guid value, and the values on the form.
pub fn save_account_with_guid(account_guid: Uuid) {
    let account_name = document_query_selector("#account_editor_account_name_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();
    let account_type_select = document_query_selector("#account_editor_account_type_select")
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap();

    let option = account_type_select
        .get(
            (account_type_select.selected_index() as i32)
                .try_into()
                .unwrap(),
        )
        .unwrap()
        .dyn_into::<web_sys::HtmlOptionElement>()
        .unwrap();

    let account_type = match option.value().as_str() {
        "ASSET" => accounts_manager::AccountType::ASSET,
        "BANK" => accounts_manager::AccountType::BANK,
        "CASH" => accounts_manager::AccountType::CASH,
        "CREDIT" => accounts_manager::AccountType::CREDIT,
        "EQUITY" => accounts_manager::AccountType::EQUITY,
        "EXPENSE" => accounts_manager::AccountType::EXPENSE,
        "INCOME" => accounts_manager::AccountType::INCOME,
        "LIABILITY" => accounts_manager::AccountType::LIABILITY,
        "RECEIVABLE" => accounts_manager::AccountType::RECEIVABLE,
        _ => {
            panic!("The option {} is not valid!", option.value());
        }
    };

    let commodity_select = document_query_selector("#account_editor_account_commodity_select")
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap();
    let option = commodity_select
        .get(
            (commodity_select.selected_index() as i32)
                .try_into()
                .unwrap(),
        )
        .unwrap()
        .dyn_into::<web_sys::HtmlOptionElement>()
        .unwrap();

    let commodity_guid = dhu::convert_string_to_guid(option.value())
        .expect(&format!("Failed to turn {} into GUID!", option.value()));

    let account_code = document_query_selector("#account_editor_account_code_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let account_description = document_query_selector("#account_editor_account_description_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();
    let account_hidden = {
        if document_query_selector("#account_editor_account_hidden_input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked()
        {
            accounts_manager::Bool::True
        } else {
            accounts_manager::Bool::False
        }
    };

    let account_placeholder = {
        if document_query_selector("#account_editor_account_placeholder_input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked()
        {
            accounts_manager::Bool::True
        } else {
            accounts_manager::Bool::False
        }
    };

    let parent_guid = accounts_manager::retrieve_account_for_account_type(account_type.to_string())
        .unwrap()
        .guid;

    let account_to_save = Account {
        guid: account_guid,
        name: account_name,
        account_type: account_type, //Account_Type is the account type. (Ex: 'ROOT' or 'CREDIT')
        commodity_guid: Some(commodity_guid), //Commodity_Guid is the commodity guid the account uses. Ex: USD or YEN.
        commodity_scu: 100,                   //Commodity_Scu is the commodity scu. 100 for USD.
        non_std_scu: -1,                      //Non_Std_Scu is the non std scu. -1 by default
        parent_guid: Some(parent_guid), //Parent_Guid is the parent of this account's GUID. null guid by default
        code: account_code,             //Code is the code for this account. Blank by default
        description: account_description, //Description is the description for this account. Blank by default.
        hidden: account_hidden, //Hidden is a bit field whether this account is hidden or not.
        placeholder: account_placeholder, //Placeholder is whether this account is a placeholder account. (1 for yes, 0 for no)
        tags: {
            let return_value = HashMap::new();
            return_value
        },
    };

    js::alert(&format!("Let's save a guid: {guid}, name: {name}, account_type: {account_type:?}, 
                        commodity_guid: {commodity_guid}, commodity_scu: {commodity_scu}, non_std_scu: {non_std_scu},
                        parent_guid: {parent_guid}, code: {code}, description: {description}, hidden: {hidden:?},
                        placeholder: {placeholder:?}",
                        guid=account_to_save.guid,
                        name=account_to_save.name,
                        account_type=account_to_save.account_type,
                        commodity_guid=account_to_save.commodity_guid.unwrap(),
                        commodity_scu=account_to_save.commodity_scu,
                        non_std_scu=account_to_save.non_std_scu,
                        parent_guid=account_to_save.parent_guid.unwrap(),
                        code=account_to_save.code,
                        description=account_to_save.description,
                        hidden=account_to_save.hidden,
                        placeholder=account_to_save.placeholder,
                    ));
    match accounts_manager::save_new_and_delete_current(account_to_save) {
        Ok(_e) => {
            js::alert("Success!");
        }
        Err(e) => {
            js::alert(&format!("There was an error: '{}'", e));
        }
    }
}

/// load_account_editor_into_body loads the account editor into the body
/// for the given account_guid. Pass in a new GUID if you want to create
/// a new account instead of editing one.
pub fn load_account_editor_into_body(account_guid: Uuid) {
    //Check for the account with a given account guid
    let result = accounts_manager::retrieve_account_for_guid(account_guid);
    let mut account = Account {
        guid: account_guid,   //guid is the GUID for this account.
        name: "".to_string(), //Name is the name of the account.
        account_type: accounts_manager::AccountType::ASSET, //Account_Type is the account type. (Ex: 'ROOT' or 'CREDIT')
        commodity_guid: Some(Uuid::nil()), //Commodity_Guid is the commodity guid the account uses. Ex: USD or YEN.
        commodity_scu: 0,                  //Commodity_Scu is the commodity scu. 100 for USD.
        non_std_scu: 0,                    //Non_Std_Scu is the non std scu. -1 by default
        parent_guid: Some(Uuid::nil()), //Parent_Guid is the parent of this account's GUID. null guid by default
        code: "".to_string(),           //Code is the code for this account. Blank by default
        description: "".to_string(), //Description is the description for this account. Blank by default.
        hidden: accounts_manager::Bool::False, //Hidden is a bit field whether this account is hidden or not.
        placeholder: accounts_manager::Bool::False, //Placeholder is whether this account is a placeholder account. (1 for yes, 0 for no)
        tags: {
            let return_value = HashMap::new();
            return_value
        },
    };
    //If the result is ok, then we can setup the account for the retrieved value
    if result.is_ok() {
        let result_account = result.unwrap();
        account.name = result_account.name;
        account.account_type = result_account.account_type;
        js::log(&format!(
            "Result is okay, and account_type is: '{:?}'.",
            account.account_type
        ));
        account.commodity_guid = result_account.commodity_guid;
        account.commodity_scu = result_account.commodity_scu;
        account.non_std_scu = result_account.non_std_scu;
        account.parent_guid = result_account.parent_guid;
        account.code = result_account.code;
        account.description = result_account.description;
        account.hidden = result_account.hidden;
        account.placeholder = result_account.placeholder;
        account.tags = result_account.tags;
    }

    //Clear out the body, and footer first
    let body_div = document_query_selector("#body");
    body_div.set_inner_html("");
    let footer_div = document_query_selector("#footer");
    footer_div.set_inner_html("");

    //Create the new account form
    let account_type = document_create_element("div");
    let account_type_html: String = format!(
        "
        <label id='account_editor_account_type_label'>Account Type: 
            <select id='account_editor_account_type_select'>
                <option {asset}>ASSET</option>
                <option {bank}>BANK</option>
                <option {cash}>CASH</option>
                <option {credit}>CREDIT</option>
                <option {equity}>EQUITY</option>
                <option {expense}>EXPENSE</option>
                <option {income}>INCOME</option>
                <option {liability}>LIABILITY</option>
                <option {receivable}>RECEIVABLE</option>
            </select>
        </label>",
        asset = if account.account_type == accounts_manager::AccountType::ASSET {
            "SELECTED"
        } else {
            ""
        },
        bank = if account.account_type == accounts_manager::AccountType::BANK {
            "SELECTED"
        } else {
            ""
        },
        cash = if account.account_type == accounts_manager::AccountType::CASH {
            "SELECTED"
        } else {
            ""
        },
        credit = if account.account_type == accounts_manager::AccountType::CREDIT {
            "SELECTED"
        } else {
            ""
        },
        equity = if account.account_type == accounts_manager::AccountType::EQUITY {
            "SELECTED"
        } else {
            ""
        },
        expense = if account.account_type == accounts_manager::AccountType::EXPENSE {
            "SELECTED"
        } else {
            ""
        },
        income = if account.account_type == accounts_manager::AccountType::INCOME {
            "SELECTED"
        } else {
            ""
        },
        liability = if account.account_type == accounts_manager::AccountType::LIABILITY {
            "SELECTED"
        } else {
            ""
        },
        receivable = if account.account_type == accounts_manager::AccountType::RECEIVABLE {
            "SELECTED"
        } else {
            ""
        },
    );
    account_type.set_inner_html(&account_type_html);

    body_div
        .append_child(&account_type)
        .expect("Failed to append child to body_div");

    //Create the account name elements
    {
        let account_name_div = document_create_element("div");
        body_div
            .append_child(&account_name_div)
            .expect("Failed to append account_name_div to body!");

        let account_name_label = document_create_element("label");
        account_name_label.set_id("account_editor_account_name_label");
        account_name_label.set_text_content(Some("Name: "));
        account_name_div
            .append_child(&account_name_label)
            .expect("Failed to append child to account_editor_account_div!");

        let account_name_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to convert to HTMLInputElement!");
        account_name_input.set_id("account_editor_account_name_input");
        account_name_input.set_value(&account.name);
        account_name_label
            .append_child(&account_name_input)
            .unwrap();
    }

    //Create the account code elements
    {
        let account_code_div = document_create_element("div");
        body_div.append_child(&account_code_div).unwrap();

        let account_code_label = document_create_element("label");
        account_code_label.set_id("account_editor_account_code_label");
        account_code_label.set_text_content(Some("Code: "));
        account_code_div.append_child(&account_code_label).unwrap();

        let account_code_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to convert to HTMLInputElement!");
        account_code_input.set_id("account_editor_account_code_input");
        account_code_input.set_value(&account.code);
        account_code_label
            .append_child(&account_code_input)
            .unwrap();
    }

    //Create the account description elements
    {
        let account_description_div = document_create_element("div");
        body_div.append_child(&account_description_div).unwrap();

        let account_description_label = document_create_element("label");
        account_description_label.set_id("account_editor_account_description_label");
        account_description_label.set_text_content(Some("Description: "));
        account_description_div
            .append_child(&account_description_label)
            .unwrap();

        let account_description_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to convert to HTMLInputElement!");
        account_description_input.set_id("account_editor_account_description_input");
        account_description_input.set_value(&account.description);
        account_description_label
            .append_child(&account_description_input)
            .unwrap();
    }

    //Create the account commodities elements
    {
        let commodities = commodities_manager::retrieve_all_commodities();
        let account_commodity_div = document_create_element("div");
        body_div.append_child(&account_commodity_div).unwrap();

        let mut account_commodity_html: String = "
            <label id='account_editor_account_commodity_label'>Commodity: 
                <select id='account_editor_account_commodity_select'>
        "
        .to_string();

        for commodity in commodities {
            account_commodity_html += &format!(
                "<option {selected} value='{guid}'>{mnemonic}</option>",
                selected = {
                    if account.commodity_guid.is_some()
                        && account.commodity_guid.unwrap() == commodity.guid
                    {
                        "SELECTED"
                    } else {
                        ""
                    }
                },
                mnemonic = commodity.mnemonic,
                guid = dhu::convert_guid_to_sqlite_string(&commodity.guid),
            );
        }

        account_commodity_html += "</select></label>";
        account_commodity_div.set_inner_html(&account_commodity_html);
    }

    //Create the account hidden elements
    {
        let account_hidden_div = document_create_element("div");
        body_div.append_child(&account_hidden_div).unwrap();

        let account_hidden_label = document_create_element("label");
        account_hidden_label.set_id("account_editor_account_hidden_label");
        account_hidden_label.set_text_content(Some("Hidden?: "));
        account_hidden_div
            .append_child(&account_hidden_label)
            .unwrap();

        let account_hidden_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to convert to HTMLInputElement!");
        account_hidden_input.set_id("account_editor_account_hidden_input");
        account_hidden_input.set_type("checkbox");
        if account.hidden == accounts_manager::Bool::True {
            account_hidden_input.set_checked(true);
        }
        account_hidden_label
            .append_child(&account_hidden_input)
            .unwrap();
    }

    //Create the account placeholder elements
    {
        let account_placeholder_div = document_create_element("div");
        body_div.append_child(&account_placeholder_div).unwrap();

        let account_placeholder_label = document_create_element("label");
        account_placeholder_label.set_id("account_editor_account_placeholder_label");
        account_placeholder_label.set_text_content(Some("Placeholder?: "));
        account_placeholder_div
            .append_child(&account_placeholder_label)
            .unwrap();

        let account_placeholder_input = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to convert to HTMLInputElement!");
        account_placeholder_input.set_id("account_editor_account_placeholder_input");
        account_placeholder_input.set_type("checkbox");
        if account.placeholder == accounts_manager::Bool::True {
            account_placeholder_input.set_checked(true);
        }
        account_placeholder_label
            .append_child(&account_placeholder_input)
            .unwrap();
    }

    //Create the Okay, and Cancel Buttons
    {
        let okay_and_cancel_button_div = document_create_element("div");
        body_div.append_child(&okay_and_cancel_button_div).unwrap();

        let cancel_button = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        cancel_button.set_type("button");
        cancel_button.set_id("account_editor_cancel_button");
        cancel_button.set_value("Cancel");

        okay_and_cancel_button_div
            .append_child(&cancel_button)
            .unwrap();

        //Setup the cancel_button handler
        let cancel_button_on_click = Closure::wrap(Box::new(move || {
            load_accounts_with_balances_from_memory();
        }) as Box<dyn Fn()>);

        cancel_button.set_onclick(Some(cancel_button_on_click.as_ref().unchecked_ref()));
        cancel_button_on_click.forget();

        let okay_button = document_create_element("input")
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        okay_button.set_type("button");
        okay_button.set_id("account_editor_okay_button");
        okay_button.set_value("Okay");

        okay_and_cancel_button_div
            .append_child(&okay_button)
            .unwrap();

        //Setup the okay_button handler
        let okay_button_on_click = Closure::wrap(Box::new(move || {
            save_account_with_guid(account_guid);
        }) as Box<dyn Fn()>);

        okay_button.set_onclick(Some(okay_button_on_click.as_ref().unchecked_ref()));
        okay_button_on_click.forget();
    }
}

/// load_accounts_with_balances_from_memory loads all the accounts with balances from memory.
/// This includes transactions in the future.
pub fn load_accounts_with_balances_from_memory() {
    if crate::DATABASE.lock().unwrap().len() == 0 {
        js::alert("Please select a database to refresh your accounts view.");
        return;
    }

    //Prepare a statement
    let stmt: dhu::Statement =
        crate::DATABASE.lock().unwrap()[0].prepare(&shu::load_accounts_with_balances());
    stmt.getAsObject();

    let mut accounts = Vec::new();

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

    stmt.free();

    load_accounts_into_body(accounts);
}
