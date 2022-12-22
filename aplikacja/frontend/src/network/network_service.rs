use gloo_console::log;
use reqwasm::http::{Request, Headers,ReferrerPolicy,RequestMode};
use yew::UseStateHandle;
use reqwasm::{Error};

// pub async fn test_sieci() {
//     //let endpoint = "http://localhost:80";
//     //let endpoint = "https://api.weather.gov/gridpoints/DTX/65/33/forecast";
//     let endpoint = "http://localhost";
//     let headers = Headers::new();
    
//     // headers.append("Access-Control-Allow-Origin", "http://localhost:8080");

//     let fetched_data = Request::get(&endpoint)
//         .headers(headers)
//         // .referrer_policy(ReferrerPolicy::None)
//         .mode(RequestMode::NoCors)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     log!(format!("fetched data1: {fetched_data}"));
// }

// pub fn make_request<T>(handle: UseStateHandle<T>,endpoint: &str)-> Result<Box<T>, Error> {
//     todo!();
//     let handle = handle.clone();
//     let headers = Headers::new();
//     wasm_bindgen_futures::spawn_local( async move {
//         let fetched_1 = Request::get(endpoint)
//             .header("Allow-Control-Access-Origin", "*")
//             .mode(RequestMode::Cors)
//             .send()
//             .await
//             .unwrap_or()
//             .text()
//             .await
//             .unwrap();

//         // log!(format!("{} fetched data1: {fetched_data}",Date::get_milliseconds(&Date::new_0()).to_string()));
//         // data.set(fetched_data);
//         // log!(format!("{} data2: {}",Date::get_milliseconds(&Date::new_0()).to_string(),(*data).clone()));
//     });
// }