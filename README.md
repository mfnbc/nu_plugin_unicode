# nu_plugin_unicode

Thin Nushell bindings for the Rust unicode crates: character segmentation, grapheme clusters, and normalization.

This plugin is intentionally generic and parity-focused. It should stay close to the underlying Rust unicode behavior and avoid Hebrew-specific semantics.

## Features

- **`unicode chars`**: Split text into Unicode scalar values with codepoints
- **`unicode graphemes`**: Split text into grapheme clusters (user-perceived characters)
- **`unicode normalize`**: Normalize text (NFC, NFD, NFKC, NFKD)

## Installation

```
cargo install --path .
plugin add ./target/release/nu_plugin_unicode
```

## Usage

### Character Segmentation (Hebrew Example)

```
> "בְּרֵאשִׁית" | unicode chars
╭───┬────┬──────╮
│ # │ ch │  cp  │
├───┼────┼──────┤
│ 0 │ ב  │ 1489 │
│ 1 │ ְ  │ 1456 │
│ 2 │ ּ  │ 1468 │
│ 3 │ ר  │ 1512 │
│ 4 │ ֵ  │ 1461 │
│ 5 │ א  │ 1488 │
│ 6 │ שׁ │ 1513 │
│ 7 │ ִ  │ 1460 │
│ 8 │ י  │ 1497 │
│ 9 │ ת  │ 1514 │
╰───┴────┴──────╯
```

### Grapheme Clusters (Emoji Example)

```
> "👨‍👩‍👧‍👦" | unicode graphemes
╭───┬───────────┬───────────────────────────────────╮
│ # │ grapheme  │ cps                               │
├───┼───────────┼───────────────────────────────────┤
│ 0 │ 👨‍👩‍👧‍👦 │ [128104, 8205, 128105, 8205, ...] │
╰───┴───────────┴───────────────────────────────────╯
```

### Unicode Normalization (French Example)

```
> "café" | unicode normalize nfd | unicode chars | where cp == 769
╭───┬────┬─────╮
│ # │ ch │ cp  │
├───┼────┼─────┤
│ 0 │ ́  │ 769 │
╰───┴────┴─────╯
```

## Use in TE2 Pipeline

Use this plugin when TE2 needs direct Unicode primitives from Rust.

The public surface should stay limited to `unicode chars`, `unicode graphemes`, and `unicode normalize`.

## Technical Details

### Dependencies

- **unicode-segmentation 1.10**: Grapheme clustering
- **unicode-normalization 0.1**: Standard Unicode normalization forms
- **nu-plugin 0.89**: Nushell plugin framework
- **nu-protocol 0.89**: Nushell value types

### Commands

#### `unicode chars`

Splits a string into Unicode scalar values (Rust `char` type).

**Input**: `String`  
**Output**: `List<Record{ch: String, cp: Int}>`

Each record contains:
- `ch`: The character as a string
- `cp`: The Unicode codepoint as an integer

#### `unicode graphemes`

Splits a string into grapheme clusters according to Unicode Standard Annex #29.

**Input**: `String`  
**Output**: `List<Record{grapheme: String, cps: List<Int>}>`

Each record contains:
- `grapheme`: The grapheme cluster as a string
- `cps`: List of codepoints that make up the grapheme

#### `unicode normalize <form>`

Normalizes Unicode text to one of four standard forms.

**Input**: `String`  
**Output**: `String`

**Forms**:
- `nfc`: Canonical Composition (composed characters)
- `nfd`: Canonical Decomposition (decomposed characters)
- `nfkc`: Compatibility Composition
- `nfkd`: Compatibility Decomposition

## Testing

```
# Build and run tests
cargo build --release
chmod +x build_and_test.sh
./build_and_test.sh
```

## License

LGPL
