// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Song category enum matching database type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "song_category", rename_all = "snake_case")
)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub enum SongCategory {
    Praise,
    Worship,
    Christmas,
    Easter,
    Wedding,
    Funeral,
    Youth,
    Children,
    Communion,
    Baptism,
    Prayer,
    Thanksgiving,
    Evangelism,
    Repentance,
    Faith,
    Hope,
    Love,
    SecondComing,
    Heaven,
    Trinity,
    HolySpirit,
    Salvation
}

impl SongCategory {
    /// Get Russian display name
    pub fn name_ru(&self) -> &'static str {
        match self {
            Self::Praise => "Прославление",
            Self::Worship => "Поклонение",
            Self::Christmas => "Рождественские",
            Self::Easter => "Пасхальные",
            Self::Wedding => "Свадебные",
            Self::Funeral => "Похоронные",
            Self::Youth => "Молодёжные",
            Self::Children => "Детские",
            Self::Communion => "Вечеря Господня",
            Self::Baptism => "Крещение",
            Self::Prayer => "Молитвенные",
            Self::Thanksgiving => "Благодарственные",
            Self::Evangelism => "Евангелизационные",
            Self::Repentance => "Покаяние",
            Self::Faith => "Вера",
            Self::Hope => "Надежда",
            Self::Love => "Любовь",
            Self::SecondComing => "Второе пришествие",
            Self::Heaven => "Небеса",
            Self::Trinity => "Троица",
            Self::HolySpirit => "Святой Дух",
            Self::Salvation => "Спасение"
        }
    }

    /// Get all categories
    pub fn all() -> &'static [SongCategory] {
        &[
            Self::Praise,
            Self::Worship,
            Self::Christmas,
            Self::Easter,
            Self::Wedding,
            Self::Funeral,
            Self::Youth,
            Self::Children,
            Self::Communion,
            Self::Baptism,
            Self::Prayer,
            Self::Thanksgiving,
            Self::Evangelism,
            Self::Repentance,
            Self::Faith,
            Self::Hope,
            Self::Love,
            Self::SecondComing,
            Self::Heaven,
            Self::Trinity,
            Self::HolySpirit,
            Self::Salvation
        ]
    }
}
