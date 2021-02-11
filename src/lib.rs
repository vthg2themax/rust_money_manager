extern crate wasm_bindgen;
extern crate web_sys;
extern crate unicode_segmentation;

use wasm_bindgen::prelude::*;

mod database_tables;
mod utility;
use utility::database_helper_utility as dhu;
use utility::html_helper_utility as hhu;

// Create a static mutable byte buffer.
// We will use for passing memory between js and wasm.
// NOTE: global `static mut` means we will have "unsafe" code
// but for passing memory between js and wasm should be fine.
static mut DATABASE : Vec<dhu::Database> = Vec::new();

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    // let window = web_sys::window().expect("no global `window` exists");
    
    // let document = window.document().expect("should have a document on window");
    // let head = document.head().expect("document should have a head");
    
    // let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    // let cookie = html_document.cookie().unwrap();
    
    // //Check if we have already done this
    // if cookie != "" {
    //     return Ok(())    
    // }

    // html_document.write(&js_sys::Array::from(&JsValue::from(&html_helper_utility::get_default_page_html())))?;
    // html_document.set_cookie("written")?;
    // html_document.close()?;

    // let style = document.create_element("style")?;
    // style.set_inner_html(&html_helper_utility::get_default_page_css());
    
    // head.append_child(&style)?;

    // let script = document.create_element("script")?;
    // script.set_inner_html(&html_helper_utility::get_default_page_js());

    // head.append_child(&script)?;

    // let body = document.body().expect("document should have a body");

    // body.set_inner_html(&html_helper_utility::get_default_page_html());    

    init_panic_hook();

    hhu::wireup_controls();

    Ok(())
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

