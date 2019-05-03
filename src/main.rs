// Specify the Windows subsystem to eliminate console window.
// Requires Rust 1.18.
#![windows_subsystem="windows"]

extern crate sciter;
extern crate rusqlite;
extern crate chrono;

mod accounts_manager;
mod database_helper_utility;
mod sciter_helper_utility;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use chrono::prelude::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;

fn main() -> Result<()> {
    let file_path = "/home/vince/Documents/Vinces_Money.gnucash.bak";
    //let file_path = "Y:/Vinces_Money.gnucash.bak";
    let mut html_string : String = String::from("<html>Vince Rules!</html>");

    if std::fs::metadata(file_path).is_ok() {
        html_string = String::from("<html>");
        let accounts = accounts_manager::retrieve_by_guid(file_path, 
                                                          dhu::convert_string_to_guid(
                                                              "f737a4904dac6736c7d8fe7b765ee354".to_string())
                                                          .expect("bad guid"))
                        .expect(&"Error Finding File!");
        
        for account in accounts {
            html_string = [html_string,
                          account.name,
                          String::from(" ")].join(", ");
            
        }
        html_string = [html_string,
                      String::from("</html>")].join("");
    }

    
    println!("Here's a null guid '{0}'", dhu::_null_guid());

    let dt : NaiveDateTime = Local::now().naive_local();
    println!("Here's a string date val: '{0}'",
              dhu::convert_date_to_string_format(dt));

    let mut nt : NaiveDateTime = Local::now().naive_local();
    println!("Current nt value is: {0}", nt);
    let new_string_val : String = String::from("20190415165254");
    let is_valid_date : bool = dhu::convert_string_to_date_format(&mut nt, &new_string_val);
    
println!("The value '{0}' is {1}-ly a valid date. Returned Date is: '{2}",
new_string_val, is_valid_date, nt);

	// Step 1: Include the 'minimal.html' file as a byte array.
	// Hint: Take a look into 'minimal.html' which contains some tiscript code.
	//let html = include_bytes!("minimal.htm");
    let html = html_string.as_bytes();
    
	// Step 2: Enable the features we need in our tiscript code.
	sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
		sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8 | // Enables Sciter.machineName()
			sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8 | // Enables opening file dialog (view.selectFile())
			sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8)).unwrap(); // Enables connecting to the inspector via Ctrl+Shift+I

	// Step 3: Create a new main sciter window of type `sciter::Window`.
	// Hint: The sciter Window wrapper (src/window.rs) contains more
	// interesting functions to open or attach to another existing window.
	let mut frame = sciter::Window::new();
    

	// Step 4: Load HTML byte array from memory to `sciter::Window`.
	// Hint: second parameter is an optional uri, it can be `None` in simple cases,
	// but it is useful for debugging purposes (check the Inspector tool from the Sciter SDK).
	// Also you can use a `load_file` method, but it requires an absolute path
	// of the main document to resolve HTML resources properly.
	frame.load_html(html, Some("example://minimal.htm"));

	// Step 5: Show window and run the main app message loop until window been closed.
	frame.run_app();

    Ok(())
}


#[derive(Debug)]
struct Cat {
    name: String,
    color: String
}

fn main3() -> Result<()> {	
    let conn = Connection::open("cats.db")?;
    
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors{
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
    let last_id : String = conn.last_insert_rowid().to_string();

    for cat in catnames{
        conn.execute(
            "INSERT INTO cats (name, color_id) values (?1, ?2)",
            &[&cat.to_string(), &last_id],
        )?;
        }
    }
    let mut stmt = conn.prepare("SELECT c.name, cc.name from cats c 
                                 INNER JOIN cat_colors cc ON cc.id = c.color_id;")?;
    
	let cats = stmt
        .query_map(NO_PARAMS, |row| 
			Ok( 
                Cat {
					name: row.get(0)?,
					color: row.get(1)?,
				}
			)
		)?;	
    
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}


// fn valid_date(incoming_date_string : String) -> bool {	
// 	let return_value : bool = DateTime::parse_from_rfc3339(&incoming_date_string).is_ok();
	
// 	return return_value;
// }

fn main2() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}