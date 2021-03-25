/// js_helper_utility will be all the functions that have to do with JavaScript functions.
/// The only reason something should be here is if it interacts with the user via javascript,
/// such as alert, or if it does a confirm or soemthing that can only be handled with javascript, but
/// not HTML. Also this file will contain functions that are exposed publicly from Rust to the global JS.
/// As always the exception to this will be context specific things that are better suited elsewhere, such
/// as a transaction specific function.

use wasm_bindgen::prelude::*;

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
    pub fn new(number : f64) -> Number;

    #[wasm_bindgen(method)]
    pub fn toLocaleString(this: &Number) -> String;
}

