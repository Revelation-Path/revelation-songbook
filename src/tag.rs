use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Song tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongTag {
    pub id:          Uuid,
    pub name:        String,
    pub name_ru:     String,
    pub usage_count: i32
}
