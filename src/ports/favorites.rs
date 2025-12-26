use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::SongSummary;

/// User favorites operations
pub trait SongFavorites: Send + Sync {
    fn list_favorites(
        &self,
        user_id: Uuid
    ) -> impl Future<Output = AppResult<Vec<SongSummary>>> + Send;

    fn add_favorite(
        &self,
        user_id: Uuid,
        song_id: Uuid
    ) -> impl Future<Output = AppResult<()>> + Send;

    fn remove_favorite(
        &self,
        user_id: Uuid,
        song_id: Uuid
    ) -> impl Future<Output = AppResult<()>> + Send;
}
