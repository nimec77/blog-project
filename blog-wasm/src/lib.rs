//! Blog WASM frontend.
//!
//! Yew-based web application for the blog platform.

mod api;
mod components;
mod constants;

use yew::prelude::*;

use components::{LoginForm, RegisterForm};

/// Application view/page.
#[derive(Clone, PartialEq)]
enum Page {
    Posts,
    Login,
    Register,
}

/// Main application component.
#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Posts);
    let username = use_state(|| None::<String>);
    let is_authenticated = use_state(|| api::is_authenticated());

    // Check for existing token on mount
    {
        let username = username.clone();
        let is_authenticated = is_authenticated.clone();
        use_effect_with((), move |_| {
            if api::is_authenticated() {
                is_authenticated.set(true);
                // We don't have username stored, so leave it as None
            }
            || ()
        });
    }

    let on_logout = {
        let page = page.clone();
        let username = username.clone();
        let is_authenticated = is_authenticated.clone();
        Callback::from(move |_| {
            username.set(None);
            is_authenticated.set(false);
            page.set(Page::Posts);
        })
    };

    let on_auth_success = {
        let page = page.clone();
        let username = username.clone();
        let is_authenticated = is_authenticated.clone();
        Callback::from(move |name: String| {
            username.set(Some(name));
            is_authenticated.set(true);
            page.set(Page::Posts);
        })
    };

    let on_login_click = {
        let page = page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            page.set(Page::Login);
        })
    };

    let on_register_click = {
        let page = page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            page.set(Page::Register);
        })
    };

    let on_posts_click = {
        let page = page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            page.set(Page::Posts);
        })
    };

    let main_content = match *page {
        Page::Posts => html! {
            <div class="post-list">
                <p class="empty-state">{"Posts will be displayed here"}</p>
            </div>
        },
        Page::Login => html! {
            <LoginForm on_success={on_auth_success.clone()} />
        },
        Page::Register => html! {
            <RegisterForm on_success={on_auth_success.clone()} />
        },
    };

    html! {
        <div class="app">
            <header class="header">
                <h1>{"Blog Platform"}</h1>
                <nav>
                    <a href="/" onclick={on_posts_click}>{"Posts"}</a>
                    if *is_authenticated {
                        <div class="user-info">
                            if let Some(ref name) = *username {
                                <span>{format!("Hi, {}", name)}</span>
                            }
                            <button class="btn btn-secondary" onclick={
                                let on_logout = on_logout.clone();
                                move |_| {
                                    api::clear_token();
                                    on_logout.emit(());
                                }
                            }>
                                {"Logout"}
                            </button>
                        </div>
                    } else {
                        <>
                            <a href="/login" onclick={on_login_click}>{"Login"}</a>
                            <a href="/register" onclick={on_register_click}>{"Register"}</a>
                        </>
                    }
                </nav>
            </header>
            <main class="main">
                {main_content}
            </main>
        </div>
    }
}

/// WASM entry point.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
