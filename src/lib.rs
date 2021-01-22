extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


mod css_helper_utility;
mod html_helper_utility;
mod js_helper_utility;
mod sql_helper_utility;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern {
    fn load_accounts_from_file_with_balances(file_input: web_sys::HtmlInputElement);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

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


    Ok(())
}

///wireup_controls wires up the controls for the form.
#[wasm_bindgen]
pub fn wireup_controls() {

    //Get the elements we need
    let main_menu_load_file = web_sys::window().expect("should have a window")
                                .document().expect("should have a document")
                                .query_selector("#main_menu_load_file")
                                .expect("should have a main_menu_load_file")
                                .expect("should have a mani_menu_load_file")
                                .dyn_into::<web_sys::HtmlElement>()
                                .unwrap();
    
    let main_menu_load_file_on_click = Closure::wrap(Box::new(move || {
        //Get the input we need
        let money_manager_file_input = web_sys::window().expect("no global `window` exists")
                                        .document().expect("Should have a document on window")
                                        .query_selector("#money_manager_file_input")
                                        .expect("should have a file input")
                                        .expect("should have a file input")
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();
        money_manager_file_input.click();        
    }) as Box<dyn Fn()>);

    //Set the onClick handler for the main menu button
    main_menu_load_file.set_onclick(Some(main_menu_load_file_on_click.as_ref().unchecked_ref()));
    main_menu_load_file_on_click.forget();

    //Set the onchange handler for the money_manager_file_input        
    let money_manager_file_input_on_change = Closure::wrap(Box::new(move || {        
        //Get the input we need
        let money_manager_file_input = web_sys::window().expect("no global `window` exists")
                                        .document().expect("Should have a document on window")
                                        .query_selector("#money_manager_file_input")
                                        .expect("should have a file input")
                                        .expect("should have a file input")
                                        .dyn_into::<web_sys::HtmlInputElement>()
                                        .unwrap();
        
        load_accounts_from_file_with_balances(money_manager_file_input);

    }) as Box<dyn Fn()>);


    let money_manager_file_input = web_sys::window().expect("should have a window")
                                    .document().expect("should have a document")
                                    .query_selector("#money_manager_file_input")
                                    .expect("should have a file input")
                                    .expect("should have a file input")
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap();

    money_manager_file_input.set_onchange(Some(money_manager_file_input_on_change.as_ref().unchecked_ref()));
    money_manager_file_input_on_change.forget();

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

