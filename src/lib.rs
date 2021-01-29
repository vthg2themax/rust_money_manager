extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod accounts_manager;
mod css_helper_utility;
mod html_helper_utility;
mod js_helper_utility;
mod sql_helper_utility;

// Create a static mutable byte buffer.
// We will use for passing memory between js and wasm.
// NOTE: global `static mut` means we will have "unsafe" code
// but for passing memory between js and wasm should be fine.
static mut DATABASE : Vec<Database> = Vec::new();

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen()]
extern "C" {
    pub type Database;

    #[wasm_bindgen(constructor, js_namespace = sqlContext)]
    fn new(array: js_sys::Uint8Array) -> Database;

    #[wasm_bindgen(method)]
    fn prepare(this: &Database, s: &str) -> Statement;
    
}

#[wasm_bindgen]
extern "C" {
    
    pub type Statement;

    #[wasm_bindgen(constructor)]
    fn new() -> Statement;

    #[wasm_bindgen(method)]
    fn bind(this: &Statement);

    #[wasm_bindgen(method)]
    fn step(this: &Statement) -> bool;

    #[wasm_bindgen(method)]
    fn getAsObject(this: &Statement) -> JsValue;

}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
   
    #[wasm_bindgen(js_namespace = JSON)]
    fn stringify(obj: JsValue) -> String;
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

    init_panic_hook();

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

    //Setup the refresh accounts handler
    //Set the onchange handler for the money_manager_file_input        
    let main_menu_refresh_accounts_on_click = Closure::wrap(Box::new(move || {        
        let money_manager_file_input = web_sys::window().expect("should have a window")
                                    .document().expect("should have a document")
                                    .query_selector("#money_manager_file_input")
                                    .expect("should have a file input")
                                    .expect("should have a file input")
                                    .dyn_into::<web_sys::HtmlInputElement>()
                                    .unwrap();

        

        //load_accounts_from_file_with_balances(money_manager_file_input);
        load_accounts_from_memory_with_balances();

    }) as Box<dyn Fn()>);
    
    //Set the Accounts handler to show all the accounts
    let main_menu_accounts = web_sys::window().expect("should have a window")
                                    .document().expect("should have a document")
                                    .query_selector("#main_menu_refresh_accounts")
                                    .expect("should have a main_menu_refresh_accounts")
                                    .expect("should have a main_menu_refresh_accounts")
                                    .dyn_into::<web_sys::HtmlElement>()
                                    .unwrap();
    main_menu_accounts.set_onclick(Some(main_menu_refresh_accounts_on_click.as_ref().unchecked_ref()));
    main_menu_refresh_accounts_on_click.forget();
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn load_accounts_from_memory_with_balances() {
    unsafe {
        if DATABASE.len() == 0 {
            panic!("The DATABASE has a length of 0.");
        }

        //Prepare a statement
        let stmt : Statement = DATABASE[0].prepare(&sql_helper_utility::sql_load_accounts_with_balances());
        stmt.getAsObject();

        // Bind new values
        stmt.bind();

        let mut accounts = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            //log(&("Here is a row: ".to_owned() + &stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                        stringify(row.clone()).as_str()                                    
                                ).unwrap();

            let balance = format!("{}",tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            
            account.tags.insert("balance".to_string(), balance.to_string());
    
                
            log(format!("The balance is: {}", balance).as_str());
            
            accounts.push(account);
        }
    
        html_helper_utility::load_accounts(accounts);
    
    }
}

#[wasm_bindgen]
pub fn load_accounts_from_file_with_balances(file_input : web_sys::HtmlInputElement) {
    //Check the file list from the input
    let filelist = file_input.files().expect("Failed to get filelist from File Input!");
    //Do not allow blank inputs
    if filelist.length() < 1 {
        alert("Please select at least one file.");
        return;
    }
    if filelist.get(0) == None {
        alert("Please select a valid file");
        return;
    }
    
    let file = filelist.get(0).expect("Failed to get File from filelist!");

    let file_reader : web_sys::FileReader = match web_sys::FileReader::new() {
        Ok(f) => f,
        Err(e) => {
            alert("There was an error creating a file reader");
            log(&JsValue::as_string(&e).expect("error converting jsvalue to string."));
            web_sys::FileReader::new().expect("")
        }
    };

    let fr_c = file_reader.clone();
    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        let len = array.byte_length() as usize;
        log(&format!("Blob received {}bytes: {:?}", len, array.to_vec()));
        // here you can for example use the received image/png data
        let db : Database = Database::new(array.clone());
        
        unsafe {
            DATABASE.push(Database::new(array.clone()));
        }

        //Prepare a statement
        let stmt : Statement = db.prepare(&sql_helper_utility::sql_load_accounts_with_balances());
        stmt.getAsObject();

        // Bind new values
        stmt.bind();

        let mut accounts = Vec::new();

        while stmt.step() {
            let row = stmt.getAsObject();
            //log(&("Here is a row: ".to_owned() + &stringify(row.clone()).to_owned()));

            let mut account : accounts_manager::Account = row.clone().into_serde().unwrap();
            let tags : serde_json::Value = serde_json::from_str(                                    
                                        stringify(row.clone()).as_str()                                    
                                ).unwrap();

            let balance = format!("{}",tags["balance"])
                            .parse::<f64>()
                            .expect("Balance is not valid!");
            
            account.tags.insert("balance".to_string(), balance.to_string());

            //log(format!("The balance is: {}", balance).as_str());
            accounts.push(account);
        }

        html_helper_utility::load_accounts(accounts);

    }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&file).expect("blob not readable");
    onloadend_cb.forget();

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

