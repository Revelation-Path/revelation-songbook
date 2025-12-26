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
