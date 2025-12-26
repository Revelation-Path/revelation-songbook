use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::{Songbook, SongbookEdition};

/// Songbook read operations
pub trait SongbookRead: Send + Sync {
    fn list_songbooks(&self) -> impl Future<Output = AppResult<Vec<Songbook>>> + Send;

    fn get_songbook(&self, id: Uuid) -> impl Future<Output = AppResult<Songbook>> + Send;

    fn get_songbook_by_code(&self, code: &str)
    -> impl Future<Output = AppResult<Songbook>> + Send;

    fn get_editions(
        &self,
        songbook_id: Uuid
    ) -> impl Future<Output = AppResult<Vec<SongbookEdition>>> + Send;
}
