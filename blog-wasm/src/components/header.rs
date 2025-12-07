//! Header component with navigation.

use yew::prelude::*;

use crate::api;

/// Header properties.
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    /// Callback when user logs out.
    pub on_logout: Callback<()>,
    /// Whether user is authenticated.
    pub is_authenticated: bool,
    /// Current username if authenticated.
    pub username: Option<String>,
}

/// Header component with navigation.
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_logout_click = {
        let on_logout = props.on_logout.clone();
        Callback::from(move |_: MouseEvent| {
            api::clear_token();
            on_logout.emit(());
        })
    };

    html! {
        <header class="header">
            <h1>{"Blog Platform"}</h1>
            <nav>
                <a href="/">{"Posts"}</a>
                if props.is_authenticated {
                    <div class="user-info">
                        if let Some(ref username) = props.username {
                            <span>{format!("Hi, {}", username)}</span>
                        }
                        <button class="btn btn-secondary" onclick={on_logout_click}>
                            {"Logout"}
                        </button>
                    </div>
                } else {
                    <>
                        <a href="/login">{"Login"}</a>
                        <a href="/register">{"Register"}</a>
                    </>
                }
            </nav>
        </header>
    }
}
