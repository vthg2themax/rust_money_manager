use js_sys::Array;
/// js_helper_utility will be all the functions that have to do with JavaScript functions.
/// The only reason something should be here is if it interacts with the user via javascript,
/// such as alert, or if it does a confirm or soemthing that can only be handled with javascript, but
/// not HTML. Also this file will contain functions that are exposed publicly from Rust to the global JS.
/// As always the exception to this will be context specific things that are better suited elsewhere, such
/// as a transaction specific function.
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen]
    pub fn alert(s: &str);

    #[wasm_bindgen]
    pub fn confirm(s: &str) -> bool;

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = JSON)]
    pub fn stringify(obj: JsValue) -> String;

    #[wasm_bindgen(js_namespace = ["window", "URL"])]
    pub fn createObjectURL(obj: js_sys::Uint8Array) -> JsValue;

    #[wasm_bindgen]
    pub fn btoa(obj: js_sys::Uint8Array) -> String;
}

#[wasm_bindgen()]
extern "C" {
    pub type Number;

    #[wasm_bindgen(constructor)]
    pub fn new(number: f64) -> Number;

    #[wasm_bindgen(method)]
    pub fn toLocaleString(this: &Number) -> String;
}

/// scroll_to_the_bottom scrolls to the bottom of the element.
pub fn scroll_to_the_bottom(incoming_element: HtmlElement) {
    incoming_element.set_scroll_top(incoming_element.scroll_height());
}

pub fn set_timeout(incoming_once_into_js_closure: JsValue, timeout_in_milliseconds: i32) {    
    let window = web_sys::window().expect("no global 'window' exists");
    let arguments_array = Array::new();

    let closure_to_use = incoming_once_into_js_closure
        .dyn_ref()
        .expect("Failed to convert closure.");

    let _windows_timeout_set_result = window.set_timeout_with_callback_and_timeout_and_arguments(
        closure_to_use,
        timeout_in_milliseconds,
        &arguments_array,
    );
}

/*


        //scroll to the bottom of the transaction_div
        let window = web_sys::window().expect("no global 'window' exists");

        let arguments_array = Array::new();

        let function_to_call = Closure::once_into_js(move || {
            js::log("This happens hopefully later...");
            js::scroll_to_the_bottom(document_query_selector("#transaction_div"));
        });

        let _windows_timeout_set_result = window
            .set_timeout_with_callback_and_timeout_and_arguments(
                function_to_call
                    .dyn_ref()
                    .expect("NOT A REAL FUNCTION TO CALL"),
                500,
                &arguments_array,
            );


*/