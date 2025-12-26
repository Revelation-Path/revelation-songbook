use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::SongHistoryEntry;

/// User song history operations
pub trait SongHistory: Send + Sync {
    fn list_recent(
        &self,
        user_id: Uuid,
        limit: i64
    ) -> impl Future<Output = AppResult<Vec<SongHistoryEntry>>> + Send;
}
