use std::future::Future;

use masterror::AppResult;
use uuid::Uuid;

use crate::{AddToPlaylist, CreatePlaylist, PlaylistItem, SongPlaylist};

/// Playlist operations
pub trait PlaylistRepository: Send + Sync {
    fn list_playlists(
        &self,
        user_id: Uuid
    ) -> impl Future<Output = AppResult<Vec<SongPlaylist>>> + Send;

    fn create_playlist(
        &self,
        user_id: Uuid,
        playlist: CreatePlaylist
    ) -> impl Future<Output = AppResult<SongPlaylist>> + Send;

    fn get_playlist(
        &self,
        id: Uuid,
        user_id: Uuid
    ) -> impl Future<Output = AppResult<SongPlaylist>> + Send;

    fn get_playlist_items(
        &self,
        playlist_id: Uuid,
        user_id: Uuid
    ) -> impl Future<Output = AppResult<Vec<PlaylistItem>>> + Send;

    fn add_to_playlist(
        &self,
        playlist_id: Uuid,
        item: AddToPlaylist
    ) -> impl Future<Output = AppResult<()>> + Send;

    fn remove_from_playlist(
        &self,
        playlist_id: Uuid,
        item_id: Uuid
    ) -> impl Future<Output = AppResult<()>> + Send;

    fn delete_playlist(
        &self,
        id: Uuid,
        user_id: Uuid
    ) -> impl Future<Output = AppResult<()>> + Send;
}
