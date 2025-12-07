//! Post list component for displaying a paginated list of posts.

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use blog_shared::PostDto;

use crate::api;
use crate::components::PostCard;

/// Post list properties.
#[derive(Properties, PartialEq)]
pub struct PostListProps {
    /// Current user's ID (if authenticated).
    #[prop_or_default]
    pub current_user_id: Option<i64>,
    /// Callback when a post is edited.
    #[prop_or_default]
    pub on_edit: Option<Callback<i64>>,
}

/// Post list component.
#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
    let posts = use_state(Vec::<PostDto>::new);
    let total = use_state(|| 0i64);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let page = use_state(|| 0i64);
    let limit = 10i64;

    // Fetch posts when page changes
    {
        let posts = posts.clone();
        let total = total.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page = *page;

        use_effect_with(page, move |page| {
            let page = *page;
            let posts = posts.clone();
            let total = total.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            spawn_local(async move {
                match api::list_posts(limit, page * limit).await {
                    Ok(response) => {
                        posts.set(response.posts);
                        total.set(response.total);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
                loading.set(false);
            });

            || ()
        });
    }

    let on_delete = {
        let posts = posts.clone();
        let error = error.clone();

        Callback::from(move |post_id: i64| {
            let posts = posts.clone();
            let error = error.clone();

            spawn_local(async move {
                match api::delete_post(post_id).await {
                    Ok(()) => {
                        // Remove the deleted post from the list
                        let updated: Vec<PostDto> = (*posts)
                            .iter()
                            .filter(|p| p.id != post_id)
                            .cloned()
                            .collect();
                        posts.set(updated);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                    }
                }
            });
        })
    };

    let on_prev_page = {
        let page = page.clone();
        Callback::from(move |_: MouseEvent| {
            if *page > 0 {
                page.set(*page - 1);
            }
        })
    };

    let on_next_page = {
        let page = page.clone();
        let total = total.clone();
        Callback::from(move |_: MouseEvent| {
            let max_page = (*total - 1) / limit;
            if *page < max_page {
                page.set(*page + 1);
            }
        })
    };

    let total_pages = (*total + limit - 1) / limit;
    let has_prev = *page > 0;
    let has_next = *page < total_pages - 1 && total_pages > 0;

    html! {
        <div class="post-list">
            if *loading {
                <div class="loading">{"Loading posts..."}</div>
            } else if let Some(ref err) = *error {
                <div class="message message-error">{err}</div>
            } else if posts.is_empty() {
                <div class="empty-state">
                    <p>{"No posts yet."}</p>
                    if props.current_user_id.is_some() {
                        <a href="/posts/new" class="btn btn-primary">{"Create your first post"}</a>
                    }
                </div>
            } else {
                <>
                    <div class="post-grid">
                        {for posts.iter().map(|post| {
                            let is_owner = props.current_user_id
                                .map(|uid| uid == post.author_id)
                                .unwrap_or(false);
                            html! {
                                <PostCard
                                    post={post.clone()}
                                    is_owner={is_owner}
                                    on_edit={props.on_edit.clone()}
                                    on_delete={Some(on_delete.clone())}
                                />
                            }
                        })}
                    </div>

                    if total_pages > 1 {
                        <div class="pagination">
                            <button
                                class="btn btn-secondary"
                                onclick={on_prev_page}
                                disabled={!has_prev}
                            >
                                {"← Previous"}
                            </button>
                            <span class="pagination-info">
                                {format!("Page {} of {}", *page + 1, total_pages)}
                            </span>
                            <button
                                class="btn btn-secondary"
                                onclick={on_next_page}
                                disabled={!has_next}
                            >
                                {"Next →"}
                            </button>
                        </div>
                    }
                </>
            }
        </div>
    }
}
