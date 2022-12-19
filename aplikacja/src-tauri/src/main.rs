#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_api::http::{HttpRequestBuilder,ResponseType,make_request};

fn main() {
  println!("start back");
  let mut builder = HttpRequestBuilder::new("GET", "http://example.com");
  let option = builder.response_type(ResponseType::Text)
                      .follow_redirects(false)
                      .build();

  if let Ok(response) = make_request(option) {
    println!("Response: {}", response);
  } else {
    println!("Something Happened!");
  }

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
  // This is a very simplistic example but it shows how to return a Result
  // and use it in the front-end.
  if name.contains(' ') {
    Err("Name should not contain spaces".to_string())
  } else {
    Ok(format!("Hello, {}", name))
  }
}