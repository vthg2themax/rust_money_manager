//app.js is a holder for all the javascript functions for this app.
// only use javascript functions where it would not make sense to use
// webassembly functions, such as interacting with other libraries such as sql.js.

///load_accounts_from_file loads the accounts for the given file_input type
function load_accounts_from_file(file_input) {
  var r = new FileReader();
  r.onload = function() {
    var Uints = new Uint8Array(r.result);
    db = new sqlContext.Database(Uints);
    // Prepare a statement
    var stmt = db.prepare("SELECT * FROM accounts WHERE hidden = $hidden AND name LIKE $name");
    stmt.getAsObject({$hidden:1, $name:1}); // {col1:1, col2:111}

    // Bind new values
    stmt.bind({$hidden:0, $name:'%c%'});
    while(stmt.step()) { //
      var row = stmt.getAsObject();
      console.log('Here is a row: ' + JSON.stringify(row));
    }
  }
  r.readAsArrayBuffer(file_input.files[0]);
}

function load_accounts_from_file_with_balances(file_input) {
  var r = new FileReader();
  r.onload = function() {
    var Uints = new Uint8Array(r.result);
    db = new sqlContext.Database(Uints);
    // Prepare a statement
    var stmt = db.prepare(money_manager.sql_load_accounts_with_balances());
    stmt.getAsObject();

    // Bind new values
    stmt.bind();
    while(stmt.step()) { //
      var row = stmt.getAsObject();
      console.log('Here is a row: ' + JSON.stringify(row));
    }
  }
  r.readAsArrayBuffer(file_input.files[0]);  
}
    //var db = new sqlContext.Database();
    //// Run a query without reading the results
    //db.run("CREATE TABLE test (col1, col2);");
    //// Insert two rows: (1,111) and (2,222)
    //db.run("INSERT INTO test VALUES (?,?), (?,?)", [1, 111, 2, 222]);

document.addEventListener("DOMContentLoaded", function(){
  WireUpControls();
});

///WireUpControls wires up all the event listeners for the controls on the page.
function WireUpControls() {
  //Setup the load file main menu piece
  document.querySelector('#MainMenuLoadFile').addEventListener('click', function(){
    document.querySelector('#file_input').click();
  });

  //setup the load file input piece
  document.querySelector('#file_input').addEventListener('change', function() {
    load_accounts_from_file_with_balances(this);
  });

}