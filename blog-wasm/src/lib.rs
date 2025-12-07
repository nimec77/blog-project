//! Blog WASM frontend.
//!
//! Yew-based web application for the blog platform.

mod api;
mod components;
mod constants;

use blog_shared::PostDto;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use components::{LoginForm, PostForm, PostList, RegisterForm};

/// Application view/page.
#[derive(Clone, PartialEq)]
enum Page {
    Posts,
    Login,
    Register,
    NewPost,
    EditPost(i64),
}

/// User info stored in app state.
#[derive(Clone, PartialEq, Default)]
struct UserInfo {
    id: Option<i64>,
    username: Option<String>,
}

/// Main application component.
#[function_component(App)]
fn app() -> Html {
    let page = use_state(|| Page::Posts);
    let user_info = use_state(UserInfo::default);
    let is_authenticated = use_state(api::is_authenticated);

    // Check for existing token on mount and restore user session
    {
        let is_authenticated = is_authenticated.clone();
        let user_info = user_info.clone();
        use_effect_with((), move |_| {
            if api::is_authenticated() {
                let is_authenticated = is_authenticated.clone();
                let user_info = user_info.clone();
                spawn_local(async move {
                    match api::get_me().await {
                        Ok(user) => {
                            user_info.set(UserInfo {
                                id: Some(user.id),
                                username: Some(user.username),
                            });
                            is_authenticated.set(true);
                        }
                        Err(_) => {
                            // Token is invalid, clear it
                            api::clear_token();
                            is_authenticated.set(false);
                        }
                    }
                });
            }
            || ()
        });
    }

    let on_logout = {
        let page = page.clone();
        let user_info = user_info.clone();
        let is_authenticated = is_authenticated.clone();
        Callback::from(move |_| {
            user_info.set(UserInfo::default());
            is_authenticated.set(false);
            page.set(Page::Posts);
        })
    };

    let on_auth_success = {
        let page = page.clone();
        let user_info = user_info.clone();
        let is_authenticated = is_authenticated.clone();
        Callback::from(move |(id, name): (i64, String)| {
            user_info.set(UserInfo {
                id: Some(id),
                username: Some(name),
            });
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

    let on_new_post_click = {
        let page = page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            page.set(Page::NewPost);
        })
    };

    let on_edit_post = {
        let page = page.clone();
        Callback::from(move |post_id: i64| {
            page.set(Page::EditPost(post_id));
        })
    };

    let on_post_created = {
        let page = page.clone();
        Callback::from(move |_post: PostDto| {
            page.set(Page::Posts);
        })
    };

    let on_post_cancel = {
        let page = page.clone();
        Callback::from(move |_| {
            page.set(Page::Posts);
        })
    };

    let main_content = match (*page).clone() {
        Page::Posts => html! {
            <PostList
                current_user_id={user_info.id}
                on_edit={Some(on_edit_post.clone())}
            />
        },
        Page::Login => html! {
            <LoginForm on_success={on_auth_success.clone()} />
        },
        Page::Register => html! {
            <RegisterForm on_success={on_auth_success.clone()} />
        },
        Page::NewPost => html! {
            <PostForm
                on_success={on_post_created.clone()}
                on_cancel={Some(on_post_cancel.clone())}
            />
        },
        Page::EditPost(post_id) => {
            html! {
                <PostForm
                    post_id={Some(post_id)}
                    on_success={on_post_created.clone()}
                    on_cancel={Some(on_post_cancel.clone())}
                />
            }
        }
    };

    html! {
        <div class="app">
            <header class="header">
                <h1>{"Blog Platform"}</h1>
                <nav>
                    <a href="/" onclick={on_posts_click.clone()}>{"Posts"}</a>
                    if *is_authenticated {
                        <>
                            <a href="/posts/new" onclick={on_new_post_click} class="btn btn-secondary btn-sm">
                                {"+ New Post"}
                            </a>
                            <div class="user-info">
                                if let Some(ref name) = user_info.username {
                                    <span class="username-greeting">{format!("Hi, {}", name)}</span>
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
                        </>
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
