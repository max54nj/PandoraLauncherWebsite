#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod services;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routes::AppRoute> render={routes::switch} />
        </BrowserRouter>
    }
}

fn main() {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
