use gloo_console::log;
use reqwasm::http::{Request, Headers,ReferrerPolicy,RequestMode};



pub async fn test_sieci() {
    //let endpoint = "http://localhost:80";
    //let endpoint = "https://api.weather.gov/gridpoints/DTX/65/33/forecast";
    let endpoint = "http://localhost";
    let headers = Headers::new();
    
    // headers.append("Access-Control-Allow-Origin", "http://localhost:8080");

    let fetched_data = Request::get(&endpoint)
        .headers(headers)
        // .referrer_policy(ReferrerPolicy::None)
        .mode(RequestMode::NoCors)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    log!(format!("fetched data1: {fetched_data}"));
}