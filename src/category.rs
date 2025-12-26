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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_categories() {
        let all = SongCategory::all();
        assert_eq!(all.len(), 22);
        assert_eq!(all[0], SongCategory::Praise);
        assert_eq!(all[21], SongCategory::Salvation);
    }

    #[test]
    fn test_name_ru() {
        assert_eq!(SongCategory::Praise.name_ru(), "Прославление");
        assert_eq!(SongCategory::Worship.name_ru(), "Поклонение");
        assert_eq!(SongCategory::Christmas.name_ru(), "Рождественские");
        assert_eq!(SongCategory::Easter.name_ru(), "Пасхальные");
        assert_eq!(SongCategory::Wedding.name_ru(), "Свадебные");
        assert_eq!(SongCategory::Funeral.name_ru(), "Похоронные");
        assert_eq!(SongCategory::Youth.name_ru(), "Молодёжные");
        assert_eq!(SongCategory::Children.name_ru(), "Детские");
        assert_eq!(SongCategory::Communion.name_ru(), "Вечеря Господня");
        assert_eq!(SongCategory::Baptism.name_ru(), "Крещение");
        assert_eq!(SongCategory::Prayer.name_ru(), "Молитвенные");
        assert_eq!(SongCategory::Thanksgiving.name_ru(), "Благодарственные");
        assert_eq!(SongCategory::Evangelism.name_ru(), "Евангелизационные");
        assert_eq!(SongCategory::Repentance.name_ru(), "Покаяние");
        assert_eq!(SongCategory::Faith.name_ru(), "Вера");
        assert_eq!(SongCategory::Hope.name_ru(), "Надежда");
        assert_eq!(SongCategory::Love.name_ru(), "Любовь");
        assert_eq!(SongCategory::SecondComing.name_ru(), "Второе пришествие");
        assert_eq!(SongCategory::Heaven.name_ru(), "Небеса");
        assert_eq!(SongCategory::Trinity.name_ru(), "Троица");
        assert_eq!(SongCategory::HolySpirit.name_ru(), "Святой Дух");
        assert_eq!(SongCategory::Salvation.name_ru(), "Спасение");
    }
}
