extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod html_helper_utility;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    
    let document = window.document().expect("should have a document on window");
    let head = document.head().expect("document should have a head");
    
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();
    
    //Check if we have already done this
    if cookie != "" {
        return Ok(())    
    }

    html_document.write(&js_sys::Array::from(&JsValue::from(&html_helper_utility::get_default_page_html())))?;
    html_document.set_cookie("written")?;
    html_document.close()?;

    // let style = document.create_element("style")?;
    // style.set_inner_html(&html_helper_utility::get_default_page_css());
    
    // head.append_child(&style)?;

    // let script = document.create_element("script")?;
    // script.set_inner_html(&html_helper_utility::get_default_page_js());

    // head.append_child(&script)?;

    // let body = document.body().expect("document should have a body");

    // body.set_inner_html(&html_helper_utility::get_default_page_html());    


    Ok(())
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

