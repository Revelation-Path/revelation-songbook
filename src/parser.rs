//! ChordPro format parser
//!
//! Parses ChordPro format songs into structured data.
//! Format reference: <https://www.chordpro.org/chordpro/>

use std::sync::LazyLock;

use regex::Regex;

use super::{Chord, ParsedSong, PositionedChord, SongLine, SongSection, SongSectionType};

/// Regex patterns for ChordPro parsing
static DIRECTIVE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{(\w+)(?::\s*([^}]*))?\}").unwrap());

static CHORD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

static SECTION_START_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\{(start_of_|so)(verse|chorus|bridge|tab|grid|abc|ly|textblock)(?::\s*([^}]*))?\}"
    )
    .unwrap()
});

static SECTION_END_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\{(end_of_|eo)(verse|chorus|bridge|tab|grid|abc|ly|textblock)\}").unwrap()
});

/// ChordPro format parser
pub struct ChordProParser;

impl ChordProParser {
    /// Parse ChordPro content into structured song
    pub fn parse(content: &str) -> ParsedSong {
        let mut song = ParsedSong {
            title:          None,
            subtitle:       None,
            artist:         None,
            composer:       None,
            key:            None,
            tempo:          None,
            time_signature: None,
            capo:           None,
            sections:       Vec::new()
        };

        let mut current_section: Option<SongSection> = None;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                if let Some(ref mut section) = current_section {
                    section.lines.push(SongLine {
                        text:   String::new(),
                        chords: Vec::new()
                    });
                }
                continue;
            }

            if let Some(caps) = SECTION_START_RE.captures(trimmed) {
                if let Some(section) = current_section.take()
                    && !section.lines.is_empty()
                {
                    song.sections.push(section);
                }

                let section_type = Self::parse_section_type(&caps[2]);
                let label = caps.get(3).map(|m| m.as_str().trim().to_string());

                current_section = Some(SongSection {
                    section_type,
                    label,
                    lines: Vec::new()
                });
                continue;
            }

            if SECTION_END_RE.is_match(trimmed) {
                if let Some(section) = current_section.take()
                    && !section.lines.is_empty()
                {
                    song.sections.push(section);
                }
                continue;
            }

            if let Some(caps) = DIRECTIVE_RE.captures(trimmed) {
                let directive = caps[1].to_lowercase();
                let value = caps.get(2).map(|m| m.as_str().trim().to_string());

                match directive.as_str() {
                    "title" | "t" => song.title = value,
                    "subtitle" | "st" => song.subtitle = value,
                    "artist" | "a" => song.artist = value,
                    "composer" => song.composer = value,
                    "key" => song.key = value,
                    "tempo" => {
                        song.tempo = value.and_then(|v| v.parse().ok());
                    }
                    "time" => song.time_signature = value,
                    "capo" => song.capo = value.and_then(|v| v.parse().ok()),
                    "c" | "comment" => {
                        if let Some(ref mut section) = current_section
                            && let Some(text) = value
                        {
                            section.lines.push(SongLine {
                                text,
                                chords: Vec::new()
                            });
                        }
                    }
                    _ => {}
                }
                continue;
            }

            let song_line = Self::parse_line(trimmed);

            if let Some(ref mut section) = current_section {
                section.lines.push(song_line);
            } else if !song_line.text.is_empty() || !song_line.chords.is_empty() {
                current_section = Some(SongSection {
                    section_type: SongSectionType::Verse,
                    label:        None,
                    lines:        vec![song_line]
                });
            }
        }

        if let Some(section) = current_section
            && !section.lines.is_empty()
        {
            song.sections.push(section);
        }

        song
    }

    /// Parse a single line with inline chords
    fn parse_line(line: &str) -> SongLine {
        let mut chords = Vec::new();
        let mut text = String::new();
        let mut last_end = 0;

        for caps in CHORD_RE.captures_iter(line) {
            let m = caps.get(0).unwrap();

            text.push_str(&line[last_end..m.start()]);

            let chord_str = &caps[1];
            if let Some(chord) = Chord::parse(chord_str) {
                chords.push(PositionedChord {
                    position: text.chars().count(),
                    chord
                });
            }

            last_end = m.end();
        }

        text.push_str(&line[last_end..]);

        SongLine {
            text,
            chords
        }
    }

    /// Parse section type from string
    fn parse_section_type(s: &str) -> SongSectionType {
        match s.to_lowercase().as_str() {
            "verse" | "v" => SongSectionType::Verse,
            "chorus" | "c" => SongSectionType::Chorus,
            "bridge" | "b" => SongSectionType::Bridge,
            "prechorus" | "pre-chorus" | "pc" => SongSectionType::PreChorus,
            "intro" => SongSectionType::Intro,
            "outro" => SongSectionType::Outro,
            "interlude" => SongSectionType::Interlude,
            "tag" => SongSectionType::Tag,
            "ending" | "coda" => SongSectionType::Ending,
            _ => SongSectionType::Other
        }
    }

    /// Strip chords from ChordPro content, returning plain text
    pub fn strip_chords(content: &str) -> String {
        let mut result = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                if let Some(caps) = DIRECTIVE_RE.captures(trimmed) {
                    let directive = caps[1].to_lowercase();
                    if (directive == "c" || directive == "comment")
                        && let Some(value) = caps.get(2)
                    {
                        result.push_str(value.as_str().trim());
                        result.push('\n');
                    }
                }
                continue;
            }

            let plain = CHORD_RE.replace_all(trimmed, "");
            let plain = plain.trim();

            if !plain.is_empty() {
                result.push_str(plain);
                result.push('\n');
            }
        }

        result.trim().to_string()
    }

    /// Extract first line of lyrics (for search indexing)
    pub fn extract_first_line(content: &str) -> String {
        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || (trimmed.starts_with('{') && trimmed.ends_with('}')) {
                continue;
            }

            let plain = CHORD_RE.replace_all(trimmed, "");
            let plain = plain.trim();

            if !plain.is_empty() {
                return plain.to_string();
            }
        }

        String::new()
    }

    /// Extract title from ChordPro content
    pub fn extract_title(content: &str) -> Option<String> {
        for line in content.lines() {
            if let Some(caps) = DIRECTIVE_RE.captures(line.trim()) {
                let directive = caps[1].to_lowercase();
                if directive == "title" || directive == "t" {
                    return caps.get(2).map(|m| m.as_str().trim().to_string());
                }
            }
        }
        None
    }

    /// Extract key from ChordPro content
    pub fn extract_key(content: &str) -> Option<String> {
        for line in content.lines() {
            if let Some(caps) = DIRECTIVE_RE.captures(line.trim()) {
                let directive = caps[1].to_lowercase();
                if directive == "key" {
                    return caps.get(2).map(|m| m.as_str().trim().to_string());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_song() {
        let content = r#"
{title: Amazing Grace}
{key: G}

{start_of_verse: 1}
[G]Amazing [G7]grace, how [C]sweet the [G]sound
That [G]saved a [Em]wretch like [D]me
{end_of_verse}

{start_of_chorus}
[G]I once was [C]lost, but [G]now am [D]found
Was [G]blind but [C]now I [G]see
{end_of_chorus}
"#;

        let song = ChordProParser::parse(content);

        assert_eq!(song.title, Some("Amazing Grace".to_string()));
        assert_eq!(song.key, Some("G".to_string()));
        assert_eq!(song.sections.len(), 2);

        let verse = &song.sections[0];
        assert_eq!(verse.section_type, SongSectionType::Verse);
        assert_eq!(verse.label, Some("1".to_string()));
        assert_eq!(verse.lines.len(), 2);

        let first_line = &verse.lines[0];
        assert_eq!(first_line.text, "Amazing grace, how sweet the sound");
        assert_eq!(first_line.chords.len(), 4);
        assert_eq!(first_line.chords[0].chord.root, "G");
    }

    #[test]
    fn test_strip_chords() {
        let content = r#"
{title: Test Song}
[Am]Hello [G]world
[C]Second line
"#;

        let plain = ChordProParser::strip_chords(content);
        assert_eq!(plain, "Hello world\nSecond line");
    }

    #[test]
    fn test_extract_first_line() {
        let content = r#"
{title: Test Song}
{key: Am}

[Am]Первая строка [G]песни
[C]Вторая строка
"#;

        let first = ChordProParser::extract_first_line(content);
        assert_eq!(first, "Первая строка песни");
    }

    #[test]
    fn test_parse_chord() {
        let chord = Chord::parse("Am7").unwrap();
        assert_eq!(chord.root, "A");
        assert_eq!(chord.quality, "m7");
        assert!(chord.bass.is_none());

        let chord = Chord::parse("C/G").unwrap();
        assert_eq!(chord.root, "C");
        assert_eq!(chord.quality, "");
        assert_eq!(chord.bass, Some("G".to_string()));

        let chord = Chord::parse("F#m7/C#").unwrap();
        assert_eq!(chord.root, "F#");
        assert_eq!(chord.quality, "m7");
        assert_eq!(chord.bass, Some("C#".to_string()));
    }
}
