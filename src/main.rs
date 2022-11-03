use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hello, world!"}</h2>
        </div>
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "c" {
    #[wasm_bindgen(js_name = invokHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}
