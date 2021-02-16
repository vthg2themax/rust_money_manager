/// js_helper_utility will be all the functions that have to do with JavaScript functions.
/// The only reason something should be here is if it interacts with the user via javascript,
/// such as alert, or if it does a confirm or soemthing that can only be handled with javascript, but
/// not HTML. Also this file will contain functions that are exposed publicly from Rust to the global JS.
/// As always the exception to this will be context specific things that are better suited elsewhere, such
/// as a transaction specific function.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen]
    pub fn alert(s: &str);

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

/// The following are globally accessible JavaScript Rust bindings.

// Reverse a string coming from JS 
#[wasm_bindgen]
pub fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

#[wasm_bindgen]
pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

/// convert_blob_to_base64_string converts a blob of data to a base64 string for ease of use.
pub fn convert_blob_to_base64_string(incoming_blob : js_sys::Uint8Array) -> String {
    return String::from("data:application/SQLITE FORMAT 3;base64,") + &btoa(incoming_blob);
}

pub fn get_default_page_js() -> String {
    let bytes = include_bytes!("../scripts/app.js");
    String::from_utf8_lossy(bytes).to_string()
    // r#"
    
    // load_accounts_from_file(file_input) {
    //   var r = new FileReader();
    //   r.onload = function() {
    //     var Uints = new Uint8Array(r.result);
    //     db = new sqlcontext.Database(Uints);
    //     // Prepare a statement
    //     var stmt = db.prepare("SELECT * FROM accounts WHERE hidden = $hidden AND name LIKE $name");
    //     stmt.getAsObject({$hidden:1, $name:1}); // {col1:1, col2:111}
  
    //     // Bind new values
    //     stmt.bind({$hidden:0, $name:'%c%'});
    //     while(stmt.step()) { //
    //       var row = stmt.getAsObject();
    //       console.log('Here is a row: ' + JSON.stringify(row));
    //     }
    //   }
    //   r.readAsArrayBuffer(file_input.files[0]);
    // }
    
    // "#.to_string()
  }