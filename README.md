<!--
SPDX-FileCopyrightText: 2025 Revelation Team

SPDX-License-Identifier: MIT
-->

# revelation-songbook

Songbook domain with ChordPro parser for the Revelation project.

## Features

- ChordPro format parsing
- Chord transposition
- Song, Songbook, Playlist entities
- `db` - SQLx database support

## Usage

```toml
[dependencies]
revelation-songbook = "0.1"
```

## ChordPro Example

```rust
use revelation_songbook::ChordProParser;

let content = r#"
{title: Amazing Grace}
{key: G}

{start_of_verse}
[G]Amazing [G7]grace, how [C]sweet the [G]sound
{end_of_verse}
"#;

let song = ChordProParser::parse(content);
```

## License

MIT
