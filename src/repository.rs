//! Song repository for database operations

use masterror::prelude::*;
use revelation_shared::{
    AddToPlaylist, CreatePlaylist, CreateSong, PlaylistItem, Song, SongCategory, SongFilters,
    SongHistoryEntry, SongPlaylist, SongSearchResult, SongSortBy, SongSummary, SongTag, Songbook,
    SongbookEdition, UpdateSong
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::parser::ChordProParser;

/// Song repository for database operations
#[derive(Clone)]
pub struct SongRepository {
    pool: PgPool
}

impl SongRepository {
    /// Create new repository
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    // ========================================================================
    // Songbooks
    // ========================================================================

    /// List all visible public songbooks
    pub async fn list_songbooks(&self) -> AppResult<Vec<Songbook>> {
        let songbooks = sqlx::query_as!(
            Songbook,
            r#"
            SELECT
                sb.id, sb.code, sb.name, sb.name_ru, sb.description, sb.cover_url,
                sb.songs_count,
                COALESCE((SELECT COUNT(*) FROM songs s WHERE s.songbook_id = sb.id AND s.has_chords = true), 0)::int as "songs_with_chords_count!",
                sb.is_public,
                sb.year_first_published, sb.year_latest_edition, sb.edition_name, sb.total_songs_in_print,
                sb.publisher, sb.editor, sb.isbn, sb.language, sb.country, sb.denomination,
                sb.website_url, sb.purchase_url, sb.history, sb.notes
            FROM songbooks sb
            WHERE sb.is_public = true
            ORDER BY sb.name_ru
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(songbooks)
    }

    /// Get songbook by ID
    pub async fn get_songbook(&self, id: Uuid) -> AppResult<Songbook> {
        let songbook = sqlx::query_as!(
            Songbook,
            r#"
            SELECT
                sb.id, sb.code, sb.name, sb.name_ru, sb.description, sb.cover_url,
                sb.songs_count,
                COALESCE((SELECT COUNT(*) FROM songs s WHERE s.songbook_id = sb.id AND s.has_chords = true), 0)::int as "songs_with_chords_count!",
                sb.is_public,
                sb.year_first_published, sb.year_latest_edition, sb.edition_name, sb.total_songs_in_print,
                sb.publisher, sb.editor, sb.isbn, sb.language, sb.country, sb.denomination,
                sb.website_url, sb.purchase_url, sb.history, sb.notes
            FROM songbooks sb
            WHERE sb.id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(songbook)
    }

    /// Get songbook by code
    pub async fn get_songbook_by_code(&self, code: &str) -> AppResult<Songbook> {
        let songbook = sqlx::query_as!(
            Songbook,
            r#"
            SELECT
                sb.id, sb.code, sb.name, sb.name_ru, sb.description, sb.cover_url,
                sb.songs_count,
                COALESCE((SELECT COUNT(*) FROM songs s WHERE s.songbook_id = sb.id AND s.has_chords = true), 0)::int as "songs_with_chords_count!",
                sb.is_public,
                sb.year_first_published, sb.year_latest_edition, sb.edition_name, sb.total_songs_in_print,
                sb.publisher, sb.editor, sb.isbn, sb.language, sb.country, sb.denomination,
                sb.website_url, sb.purchase_url, sb.history, sb.notes
            FROM songbooks sb
            WHERE sb.code = $1
            "#,
            code
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(songbook)
    }

    /// Get songbook editions
    pub async fn get_songbook_editions(
        &self,
        songbook_id: Uuid
    ) -> AppResult<Vec<SongbookEdition>> {
        let editions = sqlx::query_as!(
            SongbookEdition,
            r#"
            SELECT id, songbook_id, edition_name, year_published, songs_count, publisher, isbn, notes
            FROM songbook_editions
            WHERE songbook_id = $1
            ORDER BY year_published DESC
            "#,
            songbook_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(editions)
    }

    // ========================================================================
    // Songs - Read
    // ========================================================================

    /// List songs with filters
    pub async fn list_songs(
        &self,
        filters: &SongFilters,
        user_id: Option<Uuid>
    ) -> AppResult<Vec<SongSummary>> {
        let limit = filters.limit.unwrap_or(50).min(100);
        let offset = filters.offset.unwrap_or(0);

        let order_by = match filters.sort_by.unwrap_or_default() {
            SongSortBy::Title => "s.title ASC",
            SongSortBy::Number => "s.number ASC NULLS LAST, s.title ASC",
            SongSortBy::ViewsDesc => "s.views_count DESC, s.title ASC",
            SongSortBy::FavoritesDesc => "s.favorites_count DESC, s.title ASC",
            SongSortBy::RecentlyAdded => "s.created_at DESC",
            SongSortBy::HasChordsFirst => "s.has_chords DESC, s.title ASC",
            SongSortBy::NoChordsFirst => "s.has_chords ASC, s.title ASC"
        };

        // Build dynamic query
        let songs = sqlx::query_as::<_, SongSummaryRow>(&format!(
            r#"
            SELECT
                s.id,
                s.songbook_id,
                sb.code as songbook_code,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                ARRAY_AGG(DISTINCT sc.category) FILTER (WHERE sc.category IS NOT NULL) as categories
            FROM songs s
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc ON s.id = sc.song_id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $1
            WHERE 1=1
                AND ($2::uuid IS NULL OR s.songbook_id = $2)
                AND ($3::song_category IS NULL OR sc.category = $3)
                AND ($4::uuid IS NULL OR EXISTS (
                    SELECT 1 FROM song_tag_assignments sta WHERE sta.song_id = s.id AND sta.tag_id = $4
                ))
                AND ($5::text IS NULL OR s.original_key = $5)
            GROUP BY s.id, sb.code, uf.user_id
            ORDER BY {}
            LIMIT $6 OFFSET $7
            "#,
            order_by
        ))
        .bind(user_id)
        .bind(filters.songbook_id)
        .bind(filters.category)
        .bind(filters.tag_id)
        .bind(&filters.key)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(songs.into_iter().map(|r| r.into()).collect())
    }

    /// Search songs by text
    pub async fn search_songs(
        &self,
        query: &str,
        limit: i64,
        user_id: Option<Uuid>
    ) -> AppResult<Vec<SongSearchResult>> {
        let limit = limit.min(100);

        let results = sqlx::query_as::<_, SongSearchRow>(
            r#"
            SELECT
                s.id,
                s.songbook_id,
                sb.code as songbook_code,
                sb.name_ru as songbook_name,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                ARRAY_AGG(DISTINCT sc.category) FILTER (WHERE sc.category IS NOT NULL) as categories,
                ts_rank(s.content_search, websearch_to_tsquery('russian', $1)) as rank,
                ts_headline('russian', s.content_plain, websearch_to_tsquery('russian', $1),
                    'StartSel=<mark>, StopSel=</mark>, MaxWords=30, MinWords=15') as highlight
            FROM songs s
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc ON s.id = sc.song_id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $2
            WHERE s.content_search @@ websearch_to_tsquery('russian', $1)
               OR s.title ILIKE '%' || $1 || '%'
               OR s.first_line ILIKE '%' || $1 || '%'
            GROUP BY s.id, sb.code, sb.name_ru, uf.user_id
            ORDER BY
                CASE WHEN s.title ILIKE $1 || '%' THEN 0 ELSE 1 END,
                rank DESC,
                s.views_count DESC
            LIMIT $3
            "#
        )
        .bind(query)
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(|r| r.into()).collect())
    }

    /// Get song by ID
    pub async fn get_song(&self, id: Uuid, user_id: Option<Uuid>) -> AppResult<Song> {
        // Get base song data
        let row = sqlx::query_as::<_, SongRow>(
            r#"
            SELECT
                s.id, s.songbook_id, s.number, s.title, s.title_alt,
                s.author_lyrics, s.author_music, s.translator, s.year_written,
                s.copyright, s.original_key, s.tempo, s.time_signature,
                s.content, s.first_line, s.views_count, s.favorites_count,
                sb.code as songbook_code,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                COALESCE(uh.transpose_semitones, 0)::smallint as user_transpose
            FROM songs s
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $2
            LEFT JOIN (
                SELECT DISTINCT ON (song_id) song_id, transpose_semitones
                FROM user_song_history
                WHERE user_id = $2
                ORDER BY song_id, viewed_at DESC
            ) uh ON s.id = uh.song_id
            WHERE s.id = $1
            "#
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Get categories
        let categories = sqlx::query_scalar::<_, SongCategory>(
            "SELECT category FROM song_categories WHERE song_id = $1"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        // Get tags
        let tags = sqlx::query_as!(
            SongTag,
            r#"
            SELECT t.id, t.name, t.name_ru, t.usage_count
            FROM song_tags t
            JOIN song_tag_assignments sta ON t.id = sta.tag_id
            WHERE sta.song_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        // Increment view count
        sqlx::query("UPDATE songs SET views_count = views_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Record in history if user is authenticated
        if let Some(uid) = user_id {
            sqlx::query(
                r#"
                INSERT INTO user_song_history (user_id, song_id, viewed_at)
                VALUES ($1, $2, NOW())
                "#
            )
            .bind(uid)
            .bind(id)
            .execute(&self.pool)
            .await?;
        }

        Ok(row.into_song(categories, tags))
    }

    /// Get song by songbook and number
    pub async fn get_song_by_number(
        &self,
        songbook_id: Uuid,
        number: i32,
        user_id: Option<Uuid>
    ) -> AppResult<Song> {
        let id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM songs WHERE songbook_id = $1 AND number = $2"
        )
        .bind(songbook_id)
        .bind(number)
        .fetch_one(&self.pool)
        .await?;

        self.get_song(id, user_id).await
    }

    // ========================================================================
    // Songs - Write
    // ========================================================================

    /// Create a new song
    pub async fn create_song(&self, song: CreateSong) -> AppResult<Song> {
        let content_plain = ChordProParser::strip_chords(&song.content);
        let first_line = ChordProParser::extract_first_line(&song.content);

        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO songs (
                songbook_id, number, title, title_alt, author_lyrics, author_music,
                translator, year_written, copyright, original_key, tempo, time_signature,
                content, content_plain, first_line, source_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING id
            "#
        )
        .bind(song.songbook_id)
        .bind(song.number)
        .bind(&song.title)
        .bind(&song.title_alt)
        .bind(&song.author_lyrics)
        .bind(&song.author_music)
        .bind(&song.translator)
        .bind(song.year_written)
        .bind(&song.copyright)
        .bind(&song.original_key)
        .bind(song.tempo)
        .bind(&song.time_signature)
        .bind(&song.content)
        .bind(&content_plain)
        .bind(&first_line)
        .bind(&song.source_url)
        .fetch_one(&self.pool)
        .await?;

        // Add categories
        for category in &song.categories {
            sqlx::query("INSERT INTO song_categories (song_id, category) VALUES ($1, $2)")
                .bind(id)
                .bind(category)
                .execute(&self.pool)
                .await?;
        }

        // Add tags
        for tag_id in &song.tag_ids {
            sqlx::query("INSERT INTO song_tag_assignments (song_id, tag_id) VALUES ($1, $2)")
                .bind(id)
                .bind(tag_id)
                .execute(&self.pool)
                .await?;
        }

        self.get_song(id, None).await
    }

    /// Update an existing song
    pub async fn update_song(&self, id: Uuid, song: UpdateSong) -> AppResult<Song> {
        // Build update query dynamically
        let mut query = String::from("UPDATE songs SET updated_at = NOW()");
        let mut param_count = 1;

        if song.title.is_some() {
            param_count += 1;
            query.push_str(&format!(", title = ${}", param_count));
        }

        if song.content.is_some() {
            param_count += 1;
            query.push_str(&format!(", content = ${}", param_count));
            param_count += 1;
            query.push_str(&format!(", content_plain = ${}", param_count));
            param_count += 1;
            query.push_str(&format!(", first_line = ${}", param_count));
        }

        // ... continue for other fields

        query.push_str(" WHERE id = $1");

        // Execute update
        sqlx::query(&query).bind(id).execute(&self.pool).await?;

        // Update categories if provided
        if let Some(categories) = song.categories {
            sqlx::query("DELETE FROM song_categories WHERE song_id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;

            for category in categories {
                sqlx::query("INSERT INTO song_categories (song_id, category) VALUES ($1, $2)")
                    .bind(id)
                    .bind(category)
                    .execute(&self.pool)
                    .await?;
            }
        }

        // Update tags if provided
        if let Some(tag_ids) = song.tag_ids {
            sqlx::query("DELETE FROM song_tag_assignments WHERE song_id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;

            for tag_id in tag_ids {
                sqlx::query("INSERT INTO song_tag_assignments (song_id, tag_id) VALUES ($1, $2)")
                    .bind(id)
                    .bind(tag_id)
                    .execute(&self.pool)
                    .await?;
            }
        }

        self.get_song(id, None).await
    }

    /// Delete a song
    pub async fn delete_song(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM songs WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Favorites
    // ========================================================================

    /// List user's favorite songs
    pub async fn list_favorites(&self, user_id: Uuid) -> AppResult<Vec<SongSummary>> {
        let songs = sqlx::query_as::<_, SongSummaryRow>(
            r#"
            SELECT
                s.id,
                s.songbook_id,
                sb.code as songbook_code,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                true as is_favorite,
                ARRAY_AGG(DISTINCT sc.category) FILTER (WHERE sc.category IS NOT NULL) as categories
            FROM songs s
            JOIN user_favorite_songs uf ON s.id = uf.song_id
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc ON s.id = sc.song_id
            WHERE uf.user_id = $1
            GROUP BY s.id, sb.code, uf.created_at
            ORDER BY uf.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(songs.into_iter().map(|r| r.into()).collect())
    }

    /// Add song to favorites
    pub async fn add_favorite(&self, user_id: Uuid, song_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO user_favorite_songs (user_id, song_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, song_id) DO NOTHING
            "#
        )
        .bind(user_id)
        .bind(song_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Remove song from favorites
    pub async fn remove_favorite(&self, user_id: Uuid, song_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM user_favorite_songs WHERE user_id = $1 AND song_id = $2")
            .bind(user_id)
            .bind(song_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // History
    // ========================================================================

    /// List user's recent songs
    pub async fn list_recent(
        &self,
        user_id: Uuid,
        limit: i64
    ) -> AppResult<Vec<SongHistoryEntry>> {
        let entries = sqlx::query_as::<_, SongHistoryRow>(
            r#"
            SELECT DISTINCT ON (s.id)
                s.id,
                s.songbook_id,
                sb.code as songbook_code,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                ARRAY_AGG(sc.category) FILTER (WHERE sc.category IS NOT NULL) OVER (PARTITION BY s.id) as categories,
                uh.transpose_semitones,
                uh.viewed_at
            FROM user_song_history uh
            JOIN songs s ON uh.song_id = s.id
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc ON s.id = sc.song_id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $1
            WHERE uh.user_id = $1
            ORDER BY s.id, uh.viewed_at DESC
            LIMIT $2
            "#
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries.into_iter().map(|r| r.into()).collect())
    }

    // ========================================================================
    // Playlists
    // ========================================================================

    /// List user's playlists
    pub async fn list_playlists(&self, user_id: Uuid) -> AppResult<Vec<SongPlaylist>> {
        let playlists = sqlx::query_as!(
            SongPlaylist,
            r#"
            SELECT
                p.id,
                p.user_id,
                p.church_id,
                p.name,
                p.description,
                p.is_public,
                p.event_date,
                (SELECT COUNT(*) FROM song_playlist_items WHERE playlist_id = p.id)::int as "songs_count!",
                p.created_at,
                p.updated_at
            FROM song_playlists p
            WHERE p.user_id = $1
            ORDER BY p.updated_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(playlists)
    }

    /// Create a new playlist
    pub async fn create_playlist(
        &self,
        user_id: Uuid,
        playlist: CreatePlaylist
    ) -> AppResult<SongPlaylist> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO song_playlists (user_id, church_id, name, description, is_public, event_date)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(user_id)
        .bind(playlist.church_id)
        .bind(&playlist.name)
        .bind(&playlist.description)
        .bind(playlist.is_public)
        .bind(playlist.event_date)
        .fetch_one(&self.pool)
        .await?;

        self.get_playlist(id, user_id).await
    }

    /// Get playlist by ID
    pub async fn get_playlist(&self, id: Uuid, user_id: Uuid) -> AppResult<SongPlaylist> {
        let playlist = sqlx::query_as!(
            SongPlaylist,
            r#"
            SELECT
                p.id,
                p.user_id,
                p.church_id,
                p.name,
                p.description,
                p.is_public,
                p.event_date,
                (SELECT COUNT(*) FROM song_playlist_items WHERE playlist_id = p.id)::int as "songs_count!",
                p.created_at,
                p.updated_at
            FROM song_playlists p
            WHERE p.id = $1 AND (p.user_id = $2 OR p.is_public = true)
            "#,
            id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(playlist)
    }

    /// Get playlist items
    pub async fn get_playlist_items(
        &self,
        playlist_id: Uuid,
        user_id: Uuid
    ) -> AppResult<Vec<PlaylistItem>> {
        let items = sqlx::query_as::<_, PlaylistItemRow>(
            r#"
            SELECT
                pi.id,
                pi.position,
                pi.transpose_semitones,
                pi.notes,
                s.id as song_id,
                s.songbook_id,
                sb.code as songbook_code,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                ARRAY_AGG(DISTINCT sc.category) FILTER (WHERE sc.category IS NOT NULL) as categories
            FROM song_playlist_items pi
            JOIN songs s ON pi.song_id = s.id
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc ON s.id = sc.song_id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $2
            WHERE pi.playlist_id = $1
            GROUP BY pi.id, s.id, sb.code, uf.user_id
            ORDER BY pi.position
            "#
        )
        .bind(playlist_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(items.into_iter().map(|r| r.into()).collect())
    }

    /// Add song to playlist
    pub async fn add_to_playlist(&self, playlist_id: Uuid, item: AddToPlaylist) -> AppResult<()> {
        // Get next position
        let next_pos = sqlx::query_scalar::<_, i16>(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM song_playlist_items WHERE playlist_id = $1"
        )
        .bind(playlist_id)
        .fetch_one(&self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO song_playlist_items (playlist_id, song_id, position, transpose_semitones, notes)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(playlist_id)
        .bind(item.song_id)
        .bind(next_pos)
        .bind(item.transpose_semitones.unwrap_or(0))
        .bind(&item.notes)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Remove song from playlist
    pub async fn remove_from_playlist(&self, playlist_id: Uuid, item_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM song_playlist_items WHERE id = $1 AND playlist_id = $2")
            .bind(item_id)
            .bind(playlist_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delete playlist
    pub async fn delete_playlist(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM song_playlists WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ========================================================================
    // Tags
    // ========================================================================

    /// List all tags
    pub async fn list_tags(&self) -> AppResult<Vec<SongTag>> {
        let tags = sqlx::query_as!(
            SongTag,
            r#"
            SELECT id, name, name_ru, usage_count
            FROM song_tags
            ORDER BY usage_count DESC, name_ru
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    // ========================================================================
    // Categories
    // ========================================================================

    /// List songs by category
    pub async fn list_by_category(
        &self,
        category: SongCategory,
        limit: i64,
        user_id: Option<Uuid>
    ) -> AppResult<Vec<SongSummary>> {
        let songs = sqlx::query_as::<_, SongSummaryRow>(
            r#"
            SELECT
                s.id,
                s.songbook_id,
                sb.code as songbook_code,
                s.number,
                s.title,
                s.author_lyrics,
                s.first_line,
                s.original_key,
                s.has_chords,
                s.views_count,
                s.favorites_count,
                CASE WHEN uf.user_id IS NOT NULL THEN true ELSE false END as is_favorite,
                ARRAY_AGG(DISTINCT sc2.category) FILTER (WHERE sc2.category IS NOT NULL) as categories
            FROM songs s
            JOIN song_categories sc ON s.id = sc.song_id AND sc.category = $1
            LEFT JOIN songbooks sb ON s.songbook_id = sb.id
            LEFT JOIN song_categories sc2 ON s.id = sc2.song_id
            LEFT JOIN user_favorite_songs uf ON s.id = uf.song_id AND uf.user_id = $2
            GROUP BY s.id, sb.code, uf.user_id
            ORDER BY s.views_count DESC
            LIMIT $3
            "#
        )
        .bind(category)
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(songs.into_iter().map(|r| r.into()).collect())
    }
}

// ============================================================================
// Row types for query mapping
// ============================================================================

#[derive(sqlx::FromRow)]
struct SongSummaryRow {
    id:              Uuid,
    songbook_id:     Option<Uuid>,
    songbook_code:   Option<String>,
    number:          Option<i32>,
    title:           String,
    author_lyrics:   Option<String>,
    first_line:      String,
    original_key:    Option<String>,
    has_chords:      bool,
    views_count:     i32,
    favorites_count: i32,
    is_favorite:     bool,
    categories:      Option<Vec<SongCategory>>
}

impl From<SongSummaryRow> for SongSummary {
    fn from(row: SongSummaryRow) -> Self {
        Self {
            id:              row.id,
            songbook_id:     row.songbook_id,
            songbook_code:   row.songbook_code,
            number:          row.number,
            title:           row.title,
            author_lyrics:   row.author_lyrics,
            first_line:      row.first_line,
            original_key:    row.original_key,
            has_chords:      row.has_chords,
            categories:      row.categories.unwrap_or_default(),
            is_favorite:     row.is_favorite,
            views_count:     row.views_count,
            favorites_count: row.favorites_count
        }
    }
}

#[derive(sqlx::FromRow)]
struct SongSearchRow {
    id:              Uuid,
    songbook_id:     Option<Uuid>,
    songbook_code:   Option<String>,
    songbook_name:   Option<String>,
    number:          Option<i32>,
    title:           String,
    author_lyrics:   Option<String>,
    first_line:      String,
    original_key:    Option<String>,
    has_chords:      bool,
    views_count:     i32,
    favorites_count: i32,
    is_favorite:     bool,
    categories:      Option<Vec<SongCategory>>,
    rank:            f32,
    highlight:       Option<String>
}

impl From<SongSearchRow> for SongSearchResult {
    fn from(row: SongSearchRow) -> Self {
        Self {
            song:          SongSummary {
                id:              row.id,
                songbook_id:     row.songbook_id,
                songbook_code:   row.songbook_code,
                number:          row.number,
                title:           row.title,
                author_lyrics:   row.author_lyrics,
                first_line:      row.first_line,
                original_key:    row.original_key,
                has_chords:      row.has_chords,
                categories:      row.categories.unwrap_or_default(),
                is_favorite:     row.is_favorite,
                views_count:     row.views_count,
                favorites_count: row.favorites_count
            },
            songbook_name: row.songbook_name,
            highlight:     row.highlight,
            rank:          row.rank
        }
    }
}

#[derive(sqlx::FromRow)]
struct SongRow {
    id:              Uuid,
    songbook_id:     Option<Uuid>,
    songbook_code:   Option<String>,
    number:          Option<i32>,
    title:           String,
    title_alt:       Option<String>,
    author_lyrics:   Option<String>,
    author_music:    Option<String>,
    translator:      Option<String>,
    year_written:    Option<i16>,
    copyright:       Option<String>,
    original_key:    Option<String>,
    tempo:           Option<i32>,
    time_signature:  Option<String>,
    content:         String,
    first_line:      String,
    views_count:     i32,
    favorites_count: i32,
    is_favorite:     bool,
    user_transpose:  i16
}

impl SongRow {
    fn into_song(self, categories: Vec<SongCategory>, tags: Vec<SongTag>) -> Song {
        Song {
            id: self.id,
            songbook_id: self.songbook_id,
            songbook_code: self.songbook_code,
            number: self.number,
            title: self.title,
            title_alt: self.title_alt,
            author_lyrics: self.author_lyrics,
            author_music: self.author_music,
            translator: self.translator,
            year_written: self.year_written,
            copyright: self.copyright,
            original_key: self.original_key,
            tempo: self.tempo,
            time_signature: self.time_signature,
            content: self.content,
            first_line: self.first_line,
            categories,
            tags,
            is_favorite: self.is_favorite,
            user_transpose: self.user_transpose,
            views_count: self.views_count,
            favorites_count: self.favorites_count
        }
    }
}

#[derive(sqlx::FromRow)]
struct SongHistoryRow {
    id:                  Uuid,
    songbook_id:         Option<Uuid>,
    songbook_code:       Option<String>,
    number:              Option<i32>,
    title:               String,
    author_lyrics:       Option<String>,
    first_line:          String,
    original_key:        Option<String>,
    has_chords:          bool,
    views_count:         i32,
    favorites_count:     i32,
    is_favorite:         bool,
    categories:          Option<Vec<SongCategory>>,
    transpose_semitones: i16,
    viewed_at:           chrono::DateTime<chrono::Utc>
}

impl From<SongHistoryRow> for SongHistoryEntry {
    fn from(row: SongHistoryRow) -> Self {
        Self {
            song:                SongSummary {
                id:              row.id,
                songbook_id:     row.songbook_id,
                songbook_code:   row.songbook_code,
                number:          row.number,
                title:           row.title,
                author_lyrics:   row.author_lyrics,
                first_line:      row.first_line,
                original_key:    row.original_key,
                has_chords:      row.has_chords,
                categories:      row.categories.unwrap_or_default(),
                is_favorite:     row.is_favorite,
                views_count:     row.views_count,
                favorites_count: row.favorites_count
            },
            transpose_semitones: row.transpose_semitones,
            viewed_at:           row.viewed_at
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlaylistItemRow {
    id:                  Uuid,
    position:            i16,
    transpose_semitones: i16,
    notes:               Option<String>,
    song_id:             Uuid,
    songbook_id:         Option<Uuid>,
    songbook_code:       Option<String>,
    number:              Option<i32>,
    title:               String,
    author_lyrics:       Option<String>,
    first_line:          String,
    original_key:        Option<String>,
    has_chords:          bool,
    views_count:         i32,
    favorites_count:     i32,
    is_favorite:         bool,
    categories:          Option<Vec<SongCategory>>
}

impl From<PlaylistItemRow> for PlaylistItem {
    fn from(row: PlaylistItemRow) -> Self {
        Self {
            id:                  row.id,
            song:                SongSummary {
                id:              row.song_id,
                songbook_id:     row.songbook_id,
                songbook_code:   row.songbook_code,
                number:          row.number,
                title:           row.title,
                author_lyrics:   row.author_lyrics,
                first_line:      row.first_line,
                original_key:    row.original_key,
                has_chords:      row.has_chords,
                categories:      row.categories.unwrap_or_default(),
                is_favorite:     row.is_favorite,
                views_count:     row.views_count,
                favorites_count: row.favorites_count
            },
            position:            row.position,
            transpose_semitones: row.transpose_semitones,
            notes:               row.notes
        }
    }
}
