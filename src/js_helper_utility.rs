pub fn get_default_page_js() -> String {
    let bytes = include_bytes!("scripts/app.js");
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