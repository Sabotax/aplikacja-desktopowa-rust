use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
// use web_sys::window;
// use yew::prelude::*;
pub mod view;

use view::main_app::App;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::Renderer::<App>::new().render();
}