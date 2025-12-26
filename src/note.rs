// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

/// Musical note (for transposition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B
}

impl Note {
    /// Parse note from string (C, C#, Db, etc.)
    pub fn parse(s: &str) -> Option<(Self, bool)> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        let chars: Vec<char> = s.chars().collect();
        let base = chars[0].to_ascii_uppercase();
        let modifier = chars.get(1).copied();

        let (note, is_flat) = match (base, modifier) {
            ('C', Some('#')) => (Note::CSharp, false),
            ('C', Some('b')) => (Note::B, true),
            ('C', _) => (Note::C, false),
            ('D', Some('#')) => (Note::DSharp, false),
            ('D', Some('b')) => (Note::CSharp, true),
            ('D', _) => (Note::D, false),
            ('E', Some('#')) => (Note::F, false),
            ('E', Some('b')) => (Note::DSharp, true),
            ('E', _) => (Note::E, false),
            ('F', Some('#')) => (Note::FSharp, false),
            ('F', Some('b')) => (Note::E, true),
            ('F', _) => (Note::F, false),
            ('G', Some('#')) => (Note::GSharp, false),
            ('G', Some('b')) => (Note::FSharp, true),
            ('G', _) => (Note::G, false),
            ('A', Some('#')) => (Note::ASharp, false),
            ('A', Some('b')) => (Note::GSharp, true),
            ('A', _) => (Note::A, false),
            ('B', Some('#')) => (Note::C, false),
            ('B', Some('b')) => (Note::ASharp, true),
            ('B', _) => (Note::B, false),
            ('H', _) => (Note::B, false),
            _ => return None
        };

        Some((note, is_flat))
    }

    /// Convert to semitone index (0-11)
    pub fn to_semitone(self) -> u8 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11
        }
    }

    /// Create from semitone index
    pub fn from_semitone(semitone: u8) -> Self {
        match semitone % 12 {
            0 => Note::C,
            1 => Note::CSharp,
            2 => Note::D,
            3 => Note::DSharp,
            4 => Note::E,
            5 => Note::F,
            6 => Note::FSharp,
            7 => Note::G,
            8 => Note::GSharp,
            9 => Note::A,
            10 => Note::ASharp,
            11 => Note::B,
            _ => unreachable!()
        }
    }

    /// Transpose by semitones
    pub fn transpose(self, semitones: i32) -> Self {
        let current = self.to_semitone() as i32;
        let new = (current + semitones).rem_euclid(12) as u8;
        Self::from_semitone(new)
    }

    /// Convert to string with sharp notation
    pub fn to_sharp_string(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B"
        }
    }

    /// Convert to string with flat notation
    pub fn to_flat_string(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::CSharp => "Db",
            Note::D => "D",
            Note::DSharp => "Eb",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "Gb",
            Note::G => "G",
            Note::GSharp => "Ab",
            Note::A => "A",
            Note::ASharp => "Bb",
            Note::B => "B"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_natural_notes() {
        assert_eq!(Note::parse("C").unwrap().0, Note::C);
        assert_eq!(Note::parse("D").unwrap().0, Note::D);
        assert_eq!(Note::parse("E").unwrap().0, Note::E);
        assert_eq!(Note::parse("F").unwrap().0, Note::F);
        assert_eq!(Note::parse("G").unwrap().0, Note::G);
        assert_eq!(Note::parse("A").unwrap().0, Note::A);
        assert_eq!(Note::parse("B").unwrap().0, Note::B);
        assert_eq!(Note::parse("H").unwrap().0, Note::B);
    }

    #[test]
    fn test_parse_sharp_notes() {
        assert_eq!(Note::parse("C#").unwrap().0, Note::CSharp);
        assert_eq!(Note::parse("D#").unwrap().0, Note::DSharp);
        assert_eq!(Note::parse("E#").unwrap().0, Note::F);
        assert_eq!(Note::parse("F#").unwrap().0, Note::FSharp);
        assert_eq!(Note::parse("G#").unwrap().0, Note::GSharp);
        assert_eq!(Note::parse("A#").unwrap().0, Note::ASharp);
        assert_eq!(Note::parse("B#").unwrap().0, Note::C);
    }

    #[test]
    fn test_parse_flat_notes() {
        let (note, is_flat) = Note::parse("Cb").unwrap();
        assert_eq!(note, Note::B);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Db").unwrap();
        assert_eq!(note, Note::CSharp);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Eb").unwrap();
        assert_eq!(note, Note::DSharp);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Fb").unwrap();
        assert_eq!(note, Note::E);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Gb").unwrap();
        assert_eq!(note, Note::FSharp);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Ab").unwrap();
        assert_eq!(note, Note::GSharp);
        assert!(is_flat);

        let (note, is_flat) = Note::parse("Bb").unwrap();
        assert_eq!(note, Note::ASharp);
        assert!(is_flat);
    }

    #[test]
    fn test_parse_lowercase() {
        assert_eq!(Note::parse("c").unwrap().0, Note::C);
        assert_eq!(Note::parse("c#").unwrap().0, Note::CSharp);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(Note::parse("").is_none());
        assert!(Note::parse("X").is_none());
        assert!(Note::parse("1").is_none());
    }

    #[test]
    fn test_to_semitone() {
        assert_eq!(Note::C.to_semitone(), 0);
        assert_eq!(Note::CSharp.to_semitone(), 1);
        assert_eq!(Note::D.to_semitone(), 2);
        assert_eq!(Note::DSharp.to_semitone(), 3);
        assert_eq!(Note::E.to_semitone(), 4);
        assert_eq!(Note::F.to_semitone(), 5);
        assert_eq!(Note::FSharp.to_semitone(), 6);
        assert_eq!(Note::G.to_semitone(), 7);
        assert_eq!(Note::GSharp.to_semitone(), 8);
        assert_eq!(Note::A.to_semitone(), 9);
        assert_eq!(Note::ASharp.to_semitone(), 10);
        assert_eq!(Note::B.to_semitone(), 11);
    }

    #[test]
    fn test_from_semitone() {
        assert_eq!(Note::from_semitone(0), Note::C);
        assert_eq!(Note::from_semitone(1), Note::CSharp);
        assert_eq!(Note::from_semitone(12), Note::C);
        assert_eq!(Note::from_semitone(13), Note::CSharp);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(Note::C.transpose(2), Note::D);
        assert_eq!(Note::C.transpose(-2), Note::ASharp);
        assert_eq!(Note::B.transpose(1), Note::C);
        assert_eq!(Note::C.transpose(12), Note::C);
    }

    #[test]
    fn test_to_sharp_string() {
        assert_eq!(Note::C.to_sharp_string(), "C");
        assert_eq!(Note::CSharp.to_sharp_string(), "C#");
        assert_eq!(Note::D.to_sharp_string(), "D");
        assert_eq!(Note::DSharp.to_sharp_string(), "D#");
        assert_eq!(Note::E.to_sharp_string(), "E");
        assert_eq!(Note::F.to_sharp_string(), "F");
        assert_eq!(Note::FSharp.to_sharp_string(), "F#");
        assert_eq!(Note::G.to_sharp_string(), "G");
        assert_eq!(Note::GSharp.to_sharp_string(), "G#");
        assert_eq!(Note::A.to_sharp_string(), "A");
        assert_eq!(Note::ASharp.to_sharp_string(), "A#");
        assert_eq!(Note::B.to_sharp_string(), "B");
    }

    #[test]
    fn test_to_flat_string() {
        assert_eq!(Note::C.to_flat_string(), "C");
        assert_eq!(Note::CSharp.to_flat_string(), "Db");
        assert_eq!(Note::D.to_flat_string(), "D");
        assert_eq!(Note::DSharp.to_flat_string(), "Eb");
        assert_eq!(Note::E.to_flat_string(), "E");
        assert_eq!(Note::F.to_flat_string(), "F");
        assert_eq!(Note::FSharp.to_flat_string(), "Gb");
        assert_eq!(Note::G.to_flat_string(), "G");
        assert_eq!(Note::GSharp.to_flat_string(), "Ab");
        assert_eq!(Note::A.to_flat_string(), "A");
        assert_eq!(Note::ASharp.to_flat_string(), "Bb");
        assert_eq!(Note::B.to_flat_string(), "B");
    }
}
