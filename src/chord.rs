// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::Note;

/// Parsed chord
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Chord {
    pub root:    String,
    pub quality: String,
    pub bass:    Option<String>
}

impl Chord {
    /// Parse chord from string like "Am7", "C#dim", "G/B"
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        let (main, bass) = if let Some(idx) = s.rfind('/') {
            let bass_part = &s[idx + 1..];
            if Note::parse(bass_part).is_some() {
                (&s[..idx], Some(bass_part.to_string()))
            } else {
                (s, None)
            }
        } else {
            (s, None)
        };

        let chars: Vec<char> = main.chars().collect();
        if chars.is_empty() {
            return None;
        }

        let (root_end, root) = if chars.len() >= 2 && (chars[1] == '#' || chars[1] == 'b') {
            (2, format!("{}{}", chars[0], chars[1]))
        } else {
            (1, chars[0].to_string())
        };

        Note::parse(&root)?;

        let quality = main[root_end..].to_string();

        Some(Self {
            root,
            quality,
            bass
        })
    }

    /// Transpose chord by semitones
    pub fn transpose(&self, semitones: i32, use_flats: bool) -> Self {
        let transpose_note = |note_str: &str| -> String {
            if let Some((note, _)) = Note::parse(note_str) {
                let transposed = note.transpose(semitones);
                if use_flats {
                    transposed.to_flat_string().to_string()
                } else {
                    transposed.to_sharp_string().to_string()
                }
            } else {
                note_str.to_string()
            }
        };

        Self {
            root:    transpose_note(&self.root),
            quality: self.quality.clone(),
            bass:    self.bass.as_ref().map(|b| transpose_note(b))
        }
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.bass {
            Some(bass) => write!(f, "{}{}/{}", self.root, self.quality, bass),
            None => write!(f, "{}{}", self.root, self.quality)
        }
    }
}

/// Chord with position in text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct PositionedChord {
    pub position: usize,
    pub chord:    Chord
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_chord() {
        let chord = Chord::parse("Am").unwrap();
        assert_eq!(chord.root, "A");
        assert_eq!(chord.quality, "m");
        assert!(chord.bass.is_none());
    }

    #[test]
    fn test_parse_chord_with_sharp() {
        let chord = Chord::parse("C#m7").unwrap();
        assert_eq!(chord.root, "C#");
        assert_eq!(chord.quality, "m7");
        assert!(chord.bass.is_none());
    }

    #[test]
    fn test_parse_chord_with_flat() {
        let chord = Chord::parse("Bbmaj7").unwrap();
        assert_eq!(chord.root, "Bb");
        assert_eq!(chord.quality, "maj7");
        assert!(chord.bass.is_none());
    }

    #[test]
    fn test_parse_chord_with_bass() {
        let chord = Chord::parse("G/B").unwrap();
        assert_eq!(chord.root, "G");
        assert_eq!(chord.quality, "");
        assert_eq!(chord.bass, Some("B".to_string()));
    }

    #[test]
    fn test_parse_complex_chord_with_bass() {
        let chord = Chord::parse("Am7/G").unwrap();
        assert_eq!(chord.root, "A");
        assert_eq!(chord.quality, "m7");
        assert_eq!(chord.bass, Some("G".to_string()));
    }

    #[test]
    fn test_parse_empty_string() {
        assert!(Chord::parse("").is_none());
        assert!(Chord::parse("   ").is_none());
    }

    #[test]
    fn test_parse_invalid_chord() {
        assert!(Chord::parse("X").is_none());
        assert!(Chord::parse("123").is_none());
    }

    #[test]
    fn test_parse_slash_with_invalid_bass() {
        let chord = Chord::parse("Am/X").unwrap();
        assert_eq!(chord.root, "A");
        assert_eq!(chord.quality, "m/X");
        assert!(chord.bass.is_none());
    }

    #[test]
    fn test_transpose_chord() {
        let chord = Chord::parse("C").unwrap();
        let transposed = chord.transpose(2, false);
        assert_eq!(transposed.root, "D");
    }

    #[test]
    fn test_transpose_chord_with_flats() {
        let chord = Chord::parse("C").unwrap();
        let transposed = chord.transpose(1, true);
        assert_eq!(transposed.root, "Db");
    }

    #[test]
    fn test_transpose_chord_with_bass() {
        let chord = Chord::parse("C/G").unwrap();
        let transposed = chord.transpose(2, false);
        assert_eq!(transposed.root, "D");
        assert_eq!(transposed.bass, Some("A".to_string()));
    }

    #[test]
    fn test_display_simple_chord() {
        let chord = Chord::parse("Am").unwrap();
        assert_eq!(format!("{}", chord), "Am");
    }

    #[test]
    fn test_display_chord_with_bass() {
        let chord = Chord::parse("G/B").unwrap();
        assert_eq!(format!("{}", chord), "G/B");
    }
}
