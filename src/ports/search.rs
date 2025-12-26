use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::{SongCategory, SongSearchResult, SongSummary};

/// Song search operations
pub trait SongSearch: Send + Sync {
    fn search_songs(
        &self,
        query: &str,
        limit: i64,
        user_id: Option<Uuid>
    ) -> impl Future<Output = AppResult<Vec<SongSearchResult>>> + Send;

    fn list_by_category(
        &self,
        category: SongCategory,
        limit: i64,
        user_id: Option<Uuid>
    ) -> impl Future<Output = AppResult<Vec<SongSummary>>> + Send;
}
