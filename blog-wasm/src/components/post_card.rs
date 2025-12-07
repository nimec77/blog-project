//! Post card component for displaying a single post.

use yew::prelude::*;

use blog_shared::PostDto;

/// Post card properties.
#[derive(Properties, PartialEq)]
pub struct PostCardProps {
    /// The post to display.
    pub post: PostDto,
    /// Whether the current user owns this post.
    #[prop_or_default]
    pub is_owner: bool,
    /// Callback when edit button is clicked.
    #[prop_or_default]
    pub on_edit: Option<Callback<i64>>,
    /// Callback when delete button is clicked.
    #[prop_or_default]
    pub on_delete: Option<Callback<i64>>,
}

/// Post card component.
#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let post = &props.post;

    let on_edit_click = {
        let post_id = post.id;
        let on_edit = props.on_edit.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ref cb) = on_edit {
                cb.emit(post_id);
            }
        })
    };

    let on_delete_click = {
        let post_id = post.id;
        let on_delete = props.on_delete.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(ref cb) = on_delete {
                cb.emit(post_id);
            }
        })
    };

    let formatted_date = post.created_at.format("%B %d, %Y").to_string();

    html! {
        <article class="post-card">
            <header class="post-card-header">
                <h2 class="post-card-title">{&post.title}</h2>
                <div class="post-card-meta">
                    <span class="post-card-author">{"by "}{&post.author_username}</span>
                    <span class="post-card-date">{formatted_date}</span>
                </div>
            </header>
            <div class="post-card-content">
                <p>{truncate_content(&post.content, 200)}</p>
            </div>
            <footer class="post-card-footer">
                <a href={format!("/posts/{}", post.id)} class="btn btn-link">{"Read more"}</a>
                if props.is_owner {
                    <div class="post-card-actions">
                        <button class="btn btn-secondary btn-sm" onclick={on_edit_click}>
                            {"Edit"}
                        </button>
                        <button class="btn btn-danger btn-sm" onclick={on_delete_click}>
                            {"Delete"}
                        </button>
                    </div>
                }
            </footer>
        </article>
    }
}

/// Truncates content to a maximum length, adding ellipsis if needed.
fn truncate_content(content: &str, max_len: usize) -> String {
    if content.len() <= max_len {
        content.to_string()
    } else {
        let truncated: String = content.chars().take(max_len).collect();
        format!("{}...", truncated.trim_end())
    }
}
