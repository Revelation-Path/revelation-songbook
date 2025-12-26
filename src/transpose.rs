// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

//! Chord transposition logic
//!
//! Transposes chords in ChordPro content by a given number of semitones.

use std::sync::LazyLock;

use regex::Regex;

use super::Note;

/// Regex for matching chords in brackets
static CHORD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

/// Regex for matching key directive
static KEY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\{key:\s*([^}]+)\}").unwrap());

/// Transpose all chords in ChordPro content by given semitones
///
/// # Arguments
/// * `content` - ChordPro formatted content
/// * `semitones` - Number of semitones to transpose (positive = up, negative =
///   down)
///
/// # Returns
/// Content with all chords transposed
pub fn transpose_content(content: &str, semitones: i32) -> String {
    if semitones == 0 {
        return content.to_string();
    }

    let use_flats = should_use_flats(content, semitones);

    let content = KEY_RE.replace(content, |caps: &regex::Captures| {
        let key = caps[1].trim();
        let transposed = transpose_key(key, semitones, use_flats);
        format!("{{key: {}}}", transposed)
    });

    CHORD_RE
        .replace_all(&content, |caps: &regex::Captures| {
            let chord = &caps[1];
            let transposed = transpose_chord(chord, semitones, use_flats);
            format!("[{}]", transposed)
        })
        .to_string()
}

/// Transpose a single key notation
pub fn transpose_key(key: &str, semitones: i32, use_flats: bool) -> String {
    transpose_chord(key, semitones, use_flats)
}

/// Transpose a single chord
fn transpose_chord(chord: &str, semitones: i32, use_flats: bool) -> String {
    let chord = chord.trim();
    if chord.is_empty() {
        return chord.to_string();
    }

    if let Some(slash_pos) = chord.rfind('/') {
        let main = &chord[..slash_pos];
        let bass = &chord[slash_pos + 1..];

        let transposed_main = transpose_single_chord(main, semitones, use_flats);
        let transposed_bass = transpose_note_in_string(bass, semitones, use_flats);

        return format!("{}/{}", transposed_main, transposed_bass);
    }

    transpose_single_chord(chord, semitones, use_flats)
}

/// Transpose a single chord (without slash)
fn transpose_single_chord(chord: &str, semitones: i32, use_flats: bool) -> String {
    let chars: Vec<char> = chord.chars().collect();
    if chars.is_empty() {
        return chord.to_string();
    }

    let (root_len, root_str) = if chars.len() >= 2 && (chars[1] == '#' || chars[1] == 'b') {
        (2, format!("{}{}", chars[0], chars[1]))
    } else {
        (1, chars[0].to_string())
    };

    if let Some((note, _)) = Note::parse(&root_str) {
        let transposed = note.transpose(semitones);
        let new_root = if use_flats {
            transposed.to_flat_string()
        } else {
            transposed.to_sharp_string()
        };

        let quality: String = chars[root_len..].iter().collect();
        format!("{}{}", new_root, quality)
    } else {
        chord.to_string()
    }
}

/// Transpose just the note part of a string
fn transpose_note_in_string(s: &str, semitones: i32, use_flats: bool) -> String {
    let s = s.trim();
    if s.is_empty() {
        return s.to_string();
    }

    let chars: Vec<char> = s.chars().collect();

    let (root_len, root_str) = if chars.len() >= 2 && (chars[1] == '#' || chars[1] == 'b') {
        (2, format!("{}{}", chars[0], chars[1]))
    } else {
        (1, chars[0].to_string())
    };

    if let Some((note, _)) = Note::parse(&root_str) {
        let transposed = note.transpose(semitones);
        let new_root = if use_flats {
            transposed.to_flat_string()
        } else {
            transposed.to_sharp_string()
        };

        let rest: String = chars[root_len..].iter().collect();
        format!("{}{}", new_root, rest)
    } else {
        s.to_string()
    }
}

/// Determine if we should use flat notation based on the key
fn should_use_flats(content: &str, semitones: i32) -> bool {
    if let Some(caps) = KEY_RE.captures(content) {
        let key = caps[1].trim();

        if let Some((note, original_is_flat)) = Note::parse(key) {
            let transposed = note.transpose(semitones);

            if original_is_flat {
                return true;
            }

            return matches!(transposed, Note::DSharp | Note::GSharp | Note::ASharp);
        }
    }

    false
}

/// Get the number of semitones between two keys
pub fn semitones_between(from: &str, to: &str) -> Option<i32> {
    let (from_note, _) = Note::parse(from)?;
    let (to_note, _) = Note::parse(to)?;

    let from_semi = from_note.to_semitone() as i32;
    let to_semi = to_note.to_semitone() as i32;

    Some((to_semi - from_semi).rem_euclid(12))
}

/// List of common keys for quick transposition UI
pub const COMMON_KEYS: &[&str] = &[
    "C", "C#", "Db", "D", "D#", "Eb", "E", "F", "F#", "Gb", "G", "G#", "Ab", "A", "A#", "Bb", "B",
    "Cm", "C#m", "Dm", "D#m", "Ebm", "Em", "Fm", "F#m", "Gm", "G#m", "Am", "A#m", "Bbm", "Bm"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_content() {
        let content = r#"
{title: Test Song}
{key: C}

[C]Hello [G]world
[Am]Second [F]line
"#;

        let transposed = transpose_content(content, 2);

        assert!(transposed.contains("{key: D}"));
        assert!(transposed.contains("[D]Hello"));
        assert!(transposed.contains("[A]world"));
        assert!(transposed.contains("[Bm]Second"));
        assert!(transposed.contains("[G]line"));
    }

    #[test]
    fn test_transpose_with_slash_chords() {
        let content = "[C/G]Hello [Am/E]world";
        let transposed = transpose_content(content, 2);
        assert_eq!(transposed, "[D/A]Hello [Bm/F#]world");
    }

    #[test]
    fn test_transpose_chord_qualities() {
        let content = "[Cmaj7] [Dm7] [G7sus4] [Fdim] [Eaug]";
        let transposed = transpose_content(content, 2);
        assert_eq!(transposed, "[Dmaj7] [Em7] [A7sus4] [Gdim] [F#aug]");
    }

    #[test]
    fn test_transpose_negative() {
        let content = "[D]Hello [A]world";
        let transposed = transpose_content(content, -2);
        assert_eq!(transposed, "[C]Hello [G]world");
    }

    #[test]
    fn test_semitones_between() {
        assert_eq!(semitones_between("C", "D"), Some(2));
        assert_eq!(semitones_between("C", "G"), Some(7));
        assert_eq!(semitones_between("G", "C"), Some(5));
        assert_eq!(semitones_between("A", "A"), Some(0));
    }
}
