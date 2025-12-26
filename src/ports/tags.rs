// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use std::future::Future;

use masterror::AppResult;

use crate::SongTag;

/// Tag operations
pub trait SongTags: Send + Sync {
    fn list_tags(&self) -> impl Future<Output = AppResult<Vec<SongTag>>> + Send;
}
