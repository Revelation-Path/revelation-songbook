use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::{CreateSong, Song, UpdateSong};

/// Song write operations
pub trait SongWrite: Send + Sync {
    fn create_song(&self, song: CreateSong) -> impl Future<Output = AppResult<Song>> + Send;

    fn update_song(
        &self,
        id: Uuid,
        song: UpdateSong
    ) -> impl Future<Output = AppResult<Song>> + Send;

    fn delete_song(&self, id: Uuid) -> impl Future<Output = AppResult<()>> + Send;
}
