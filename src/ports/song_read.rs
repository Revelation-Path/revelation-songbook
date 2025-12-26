use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::{Song, SongFilters, SongSummary};

/// Song read operations
pub trait SongRead: Send + Sync {
    fn list_songs(
        &self,
        filters: &SongFilters,
        user_id: Option<Uuid>
    ) -> impl Future<Output = AppResult<Vec<SongSummary>>> + Send;

    fn get_song(
        &self,
        id: Uuid,
        user_id: Option<Uuid>
    ) -> impl Future<Output = AppResult<Song>> + Send;

    fn get_song_by_number(
        &self,
        songbook_id: Uuid,
        number: i32,
        user_id: Option<Uuid>
    ) -> impl Future<Output = AppResult<Song>> + Send;
}
