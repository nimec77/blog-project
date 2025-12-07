//! Blog WASM frontend.
//!
//! Yew-based web application for the blog platform.

mod constants;

use yew::prelude::*;

/// Main application component.
#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <header class="header">
                <h1>{"Blog Platform"}</h1>
                <nav>
                    <a href="/">{"Posts"}</a>
                    <a href="/login">{"Login"}</a>
                </nav>
            </header>
            <main class="main">
                <p>{"Welcome to the Blog Platform!"}</p>
            </main>
        </div>
    }
}

/// WASM entry point.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
