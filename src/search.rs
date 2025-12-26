// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::SongSummary;

/// Search result with highlight info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongSearchResult {
    pub song:          SongSummary,
    pub songbook_name: Option<String>,
    pub highlight:     Option<String>,
    pub rank:          f32
}
