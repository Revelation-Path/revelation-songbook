use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Songbook (collection of songs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Songbook {
    pub id:                      Uuid,
    pub code:                    String,
    pub name:                    String,
    pub name_ru:                 String,
    pub description:             Option<String>,
    pub cover_url:               Option<String>,
    pub songs_count:             i32,
    pub songs_with_chords_count: i32,
    pub is_public:               bool,
    pub year_first_published:    Option<i16>,
    pub year_latest_edition:     Option<i16>,
    pub edition_name:            Option<String>,
    pub total_songs_in_print:    Option<i32>,
    pub publisher:               Option<String>,
    pub editor:                  Option<String>,
    pub isbn:                    Option<String>,
    pub language:                Option<String>,
    pub country:                 Option<String>,
    pub denomination:            Option<String>,
    pub website_url:             Option<String>,
    pub purchase_url:            Option<String>,
    pub history:                 Option<String>,
    pub notes:                   Option<String>
}
