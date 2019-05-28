use crate::{
    accounts_manager, books_manager, commodities_manager, database_helper_utility, 
    sciter_helper_utility, versions_manager, lots_manager, slots_manager
    };

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use chrono::prelude::*;
use guid_create::GUID;
use crate::database_helper_utility as dhu;

pub fn get_active_accounts_with_balances(file_path : &str) -> Result<String> {    
    let mut return_value = String::from("");
    let accounts = accounts_manager::retrieve_active_accounts_with_balances(file_path)
                        .expect(&"Error Finding Accounts!");

    return_value = r#"<script type='text/tiscript'>
      view.caption = "I like cheese!";
      var (x, y, width, height) = view.screenBox(view.screen, #workarea);
      var new_width = (0.8 * width).toInteger();
      var new_height = (0.5 * height).toInteger();
      x = (width * 0.1).toInteger();
      y = (height * 0.25).toInteger();
      view.move(x,y,new_width,new_height,true);
      //$(#machine).text = String.printf("%s ", view.screenBox(view.screen, #workarea));

      $(#post).on("click", : {
        $(#message).postEvent(Event.CHANGE, 0, this, view.screenBox(view.screen, #workarea) );
        view.move(250,250,600,700, true); 
      });

      $(#message).on("change", function(e) {
         this.text = String.printf("Event from `%s`: %v\n", e.source.id, e.data);
      });

    </script>
    <body>
    <h1>Fire event</h1>
  <p>Running on <em #machine/> machine</p>

  <button id="post">Post event</button>
  <button id="send">Send event</button>
  <button id="fire">Fire event</button>

  <div id="message"></div>
  <style>
    td {
      behavior:htmlarea; // selection
    }
  </style>
    "#.to_string();

    return_value = [return_value,
        "<table>".to_string(),
    ].join("");

    for account in accounts {
        return_value = [return_value,
                        String::from("<tr>"),
                        String::from("<td>"),
                        account.name,
                        String::from("</td>"),
                        String::from("<td>"),
                        account.account_type.to_string(),
                        String::from("</td>"),
                        String::from("<td>"),
                        account.description,
                        String::from("</td>"),
                        String::from("<td>"),
                        (account.tags.get("balance").expect("No balance tag!")).to_string(),
                        String::from("</td>"),
                        String::from("</tr>"),
                        String::from(" ")].join("");
        
        
    }

    return_value = [return_value,
        "</table></body>".to_string(),
    ].join("");

    Ok(return_value)
}