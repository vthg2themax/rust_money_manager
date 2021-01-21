
pub fn get_default_page_css() -> String {
    let bytes = include_bytes!("css/app.css");
    String::from_utf8_lossy(bytes).to_string()  
  }