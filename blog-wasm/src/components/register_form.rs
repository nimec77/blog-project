//! Register form component.

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use blog_shared::RegisterRequest;

use crate::api;

/// Register form properties.
#[derive(Properties, PartialEq)]
pub struct RegisterFormProps {
    /// Callback when registration succeeds.
    pub on_success: Callback<String>,
}

/// Register form component.
#[function_component(RegisterForm)]
pub fn register_form(props: &RegisterFormProps) -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_success = props.on_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let on_success = on_success.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                let req = RegisterRequest {
                    username: username_val,
                    email: email_val,
                    password: password_val,
                };

                match api::register(req).await {
                    Ok(response) => {
                        api::set_token(&response.token);
                        on_success.emit(response.user.username);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="auth-container">
            <h2>{"Register"}</h2>

            if let Some(ref err) = *error {
                <div class="message message-error">{err}</div>
            }

            <form {onsubmit}>
                <div class="form-group">
                    <label for="username">{"Username"}</label>
                    <input
                        type="text"
                        id="username"
                        value={(*username).clone()}
                        oninput={on_username_change}
                        disabled={*loading}
                        required=true
                    />
                </div>

                <div class="form-group">
                    <label for="email">{"Email"}</label>
                    <input
                        type="email"
                        id="email"
                        value={(*email).clone()}
                        oninput={on_email_change}
                        disabled={*loading}
                        required=true
                    />
                </div>

                <div class="form-group">
                    <label for="password">{"Password"}</label>
                    <input
                        type="password"
                        id="password"
                        value={(*password).clone()}
                        oninput={on_password_change}
                        disabled={*loading}
                        required=true
                    />
                </div>

                <button type="submit" class="btn btn-primary" disabled={*loading}>
                    if *loading {
                        {"Registering..."}
                    } else {
                        {"Register"}
                    }
                </button>
            </form>

            <p style="margin-top: 1rem; text-align: center; color: var(--color-text-muted);">
                {"Already have an account? "}
                <a href="/login">{"Login"}</a>
            </p>
        </div>
    }
}
