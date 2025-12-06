//! Blog service for post operations.

use std::sync::Arc;

use blog_shared::{CreatePostRequest, PostDto, PostListResponse, UpdatePostRequest};
use tracing::{info, instrument};

use crate::data::PostRepository;
use crate::domain::{AppError, Post};

/// Service for blog post operations.
#[derive(Clone)]
pub struct BlogService {
    post_repo: Arc<PostRepository>,
}

impl BlogService {
    /// Creates a new BlogService.
    pub fn new(post_repo: Arc<PostRepository>) -> Self {
        Self { post_repo }
    }

    /// Creates a new post.
    #[instrument(skip(self, req), fields(author_id = author_id))]
    pub async fn create_post(
        &self,
        author_id: i64,
        req: CreatePostRequest,
    ) -> Result<PostDto, AppError> {
        let post = self
            .post_repo
            .create(&req.title, &req.content, author_id)
            .await?;
        let author_username = self.post_repo.find_author_username(post.author_id).await?;

        info!(post_id = post.id, "Post created");

        Ok(post_to_dto(&post, author_username))
    }

    /// Gets a post by ID.
    #[instrument(skip(self))]
    pub async fn get_post(&self, id: i64) -> Result<PostDto, AppError> {
        let post = self
            .post_repo
            .find_by_id(id)
            .await?
            .ok_or(AppError::PostNotFound)?;
        let author_username = self.post_repo.find_author_username(post.author_id).await?;

        Ok(post_to_dto(&post, author_username))
    }

    /// Lists posts with pagination.
    #[instrument(skip(self))]
    pub async fn list_posts(&self, limit: i64, offset: i64) -> Result<PostListResponse, AppError> {
        let posts = self.post_repo.list(limit, offset).await?;
        let total = self.post_repo.count().await?;

        // Convert posts to DTOs with author usernames
        let mut post_dtos = Vec::with_capacity(posts.len());
        for post in posts {
            let author_username = self.post_repo.find_author_username(post.author_id).await?;
            post_dtos.push(post_to_dto(&post, author_username));
        }

        Ok(PostListResponse {
            posts: post_dtos,
            total,
        })
    }

    /// Updates a post. Only the author can update their own posts.
    #[instrument(skip(self, req), fields(post_id = id, user_id = user_id))]
    pub async fn update_post(
        &self,
        id: i64,
        user_id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostDto, AppError> {
        // Check if post exists and user is the author
        let post = self
            .post_repo
            .find_by_id(id)
            .await?
            .ok_or(AppError::PostNotFound)?;

        if post.author_id != user_id {
            return Err(AppError::Forbidden);
        }

        let updated_post = self
            .post_repo
            .update(id, req.title.as_deref(), req.content.as_deref())
            .await?;
        let author_username = self
            .post_repo
            .find_author_username(updated_post.author_id)
            .await?;

        info!("Post updated");

        Ok(post_to_dto(&updated_post, author_username))
    }

    /// Deletes a post. Only the author can delete their own posts.
    #[instrument(skip(self), fields(post_id = id, user_id = user_id))]
    pub async fn delete_post(&self, id: i64, user_id: i64) -> Result<(), AppError> {
        // Check if post exists and user is the author
        let post = self
            .post_repo
            .find_by_id(id)
            .await?
            .ok_or(AppError::PostNotFound)?;

        if post.author_id != user_id {
            return Err(AppError::Forbidden);
        }

        self.post_repo.delete(id).await?;

        info!("Post deleted");

        Ok(())
    }
}

/// Converts a Post domain entity to PostDto.
fn post_to_dto(post: &Post, author_username: String) -> PostDto {
    PostDto {
        id: post.id,
        title: post.title.clone(),
        content: post.content.clone(),
        author_id: post.author_id,
        author_username,
        created_at: post.created_at,
        updated_at: post.updated_at,
    }
}
