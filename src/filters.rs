use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SongCategory;

/// Song list filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongFilters {
    pub songbook_id: Option<Uuid>,
    pub category:    Option<SongCategory>,
    pub tag_id:      Option<Uuid>,
    pub key:         Option<String>,
    pub search:      Option<String>,
    pub limit:       Option<i64>,
    pub offset:      Option<i64>,
    pub sort_by:     Option<SongSortBy>
}

/// Sort options for songs
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SongSortBy {
    #[default]
    Title,
    Number,
    ViewsDesc,
    FavoritesDesc,
    RecentlyAdded,
    HasChordsFirst,
    NoChordsFirst
}
