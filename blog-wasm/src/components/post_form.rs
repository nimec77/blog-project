//! Post form component for creating and editing posts.

use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use blog_shared::{CreatePostRequest, PostDto, UpdatePostRequest};

use crate::api;

/// Post form properties.
#[derive(Properties, PartialEq)]
pub struct PostFormProps {
    /// Existing post to edit (None for create mode).
    #[prop_or_default]
    pub post: Option<PostDto>,
    /// Callback when form is submitted successfully.
    pub on_success: Callback<PostDto>,
    /// Callback when cancel is clicked.
    #[prop_or_default]
    pub on_cancel: Option<Callback<()>>,
}

/// Post form component.
#[function_component(PostForm)]
pub fn post_form(props: &PostFormProps) -> Html {
    let is_edit = props.post.is_some();

    let title = use_state(|| {
        props
            .post
            .as_ref()
            .map(|p| p.title.clone())
            .unwrap_or_default()
    });
    let content = use_state(|| {
        props
            .post
            .as_ref()
            .map(|p| p.content.clone())
            .unwrap_or_default()
    });
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_change = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            content.set(target.value());
        })
    };

    let onsubmit = {
        let title = title.clone();
        let content = content.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_success = props.on_success.clone();
        let post_id = props.post.as_ref().map(|p| p.id);

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let title_val = (*title).clone();
            let content_val = (*content).clone();

            // Validate
            if title_val.trim().is_empty() {
                error.set(Some("Title is required".into()));
                return;
            }
            if content_val.trim().is_empty() {
                error.set(Some("Content is required".into()));
                return;
            }

            let error = error.clone();
            let loading = loading.clone();
            let on_success = on_success.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                let result = if let Some(id) = post_id {
                    // Update existing post
                    api::update_post(
                        id,
                        UpdatePostRequest {
                            title: Some(title_val),
                            content: Some(content_val),
                        },
                    )
                    .await
                } else {
                    // Create new post
                    api::create_post(CreatePostRequest {
                        title: title_val,
                        content: content_val,
                    })
                    .await
                };

                match result {
                    Ok(post) => {
                        on_success.emit(post);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ref cb) = on_cancel {
                cb.emit(());
            }
        })
    };

    html! {
        <div class="post-form-container">
            <h2>{if is_edit { "Edit Post" } else { "Create New Post" }}</h2>

            if let Some(ref err) = *error {
                <div class="message message-error">{err}</div>
            }

            <form {onsubmit} class="post-form">
                <div class="form-group">
                    <label for="title">{"Title"}</label>
                    <input
                        type="text"
                        id="title"
                        value={(*title).clone()}
                        oninput={on_title_change}
                        disabled={*loading}
                        placeholder="Enter post title..."
                        required=true
                    />
                </div>

                <div class="form-group">
                    <label for="content">{"Content"}</label>
                    <textarea
                        id="content"
                        value={(*content).clone()}
                        oninput={on_content_change}
                        disabled={*loading}
                        placeholder="Write your post content..."
                        rows="12"
                        required=true
                    />
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary" disabled={*loading}>
                        if *loading {
                            {"Saving..."}
                        } else if is_edit {
                            {"Update Post"}
                        } else {
                            {"Create Post"}
                        }
                    </button>
                    if props.on_cancel.is_some() {
                        <button
                            type="button"
                            class="btn btn-secondary"
                            onclick={on_cancel_click}
                            disabled={*loading}
                        >
                            {"Cancel"}
                        </button>
                    }
                </div>
            </form>
        </div>
    }
}
