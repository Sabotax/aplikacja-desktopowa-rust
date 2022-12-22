use yew::{function_component, html, UseStateHandle,use_effect_with_deps};
use yew::{Html,use_state};
use reqwasm::http::{Headers,Request,RequestMode};
use gloo_console::log;
use js_sys::Date;

#[function_component(TestKomponent)]
pub fn test_komponent() -> Html {
    let data: UseStateHandle<String> = use_state(|| "".to_owned());
    {
        let data = data.clone();
        use_effect_with_deps(move |_| {
            let data = data.clone();
            let endpoint = "https://yew.rs/tutorial/data.json";
            let headers = Headers::new();
            wasm_bindgen_futures::spawn_local( async move {
                log!(format!("{} przed: ",Date::get_milliseconds(&Date::new_0()).to_string()));
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
    
                log!(format!("{} fetched data1: {fetched_data}",Date::get_milliseconds(&Date::new_0()).to_string()));
                data.set(fetched_data);
                log!(format!("{} data2: {}",Date::get_milliseconds(&Date::new_0()).to_string(),(*data).clone()));
            });
            || ()
        }, () );
    }
    html! {
            <div>
                <h3>{"Videos to watch"}</h3>
                    <p>{(*data).clone()}</p>     
            </div>
    }
}