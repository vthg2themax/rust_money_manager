use base64::{Engine, engine::general_purpose};
use wasm_bindgen::JsCast;

use crate::{
    database_tables::transactions_manager::TransactionWithSplitInformation,
    utility::html_helper_utility::{document_create_element, document_query_selector},
};

pub fn export_transactions_to_csv(
    account_guid_string: String,
    transactions_with_split_information: Vec<TransactionWithSplitInformation>,
) {
    let mut writer = csv::Writer::from_writer(vec![]);
    for txn in transactions_with_split_information {
        writer
            .serialize(txn)
            .expect("failed to serialize transaction!");
    }
    let csv_data = writer.into_inner().expect("Failed to get inner data!");

    /*     let mut wtr = Writer::from_writer(vec![]);
    ///     wtr.write_record(&["a", "b", "c"])?;
    ///     wtr.write_record(&["x", "y", "z"])?;
    ///
    ///     let data = String::from_utf8(wtr.into_inner()?)?;
    ///     assert_eq!(data, "a,b,c\nx,y,z\n");
    ///     Ok(())
     */

    //let blob = crate::DATABASE.lock().unwrap()[0].export();
    let csv_data_input = document_create_element("input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("Failed to create csv_data_input!");
    csv_data_input.set_type("hidden");
    csv_data_input.set_id("csv_data_input");
    csv_data_input.set_value("csv_data_export.csv");

    let body = document_query_selector("#body");
    body.append_child(&csv_data_input).unwrap();

    let b64 = general_purpose::STANDARD_NO_PAD.encode(csv_data);

    let filename = document_query_selector("#csv_data_input")
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let div = document_create_element("div");
    div.set_inner_html(
            &format!("<a download='{filename}' id='csv_data_export_button' 
                        href='data:application/octet-stream;base64,{base64_string}' target='_self'>Download</a>",
                        base64_string = b64,
                        filename = filename,
                    )
    );

    body.append_child(&div).unwrap();

    document_query_selector("#csv_data_export_button").click();

    div.set_inner_html("");
}
