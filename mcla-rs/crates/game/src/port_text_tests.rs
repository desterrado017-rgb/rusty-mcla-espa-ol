//! Rendering checks that do not require the player's original game resources.

use super::{text, Text, ENTRIES};
use mcla_config::Language;
use std::collections::BTreeSet;

fn fallback_bitmap_characters() -> BTreeSet<char> {
    // Read the actual existing bitmap-font inventory, rather than duplicating
    // it here or assuming that every Unicode character has a drawable glyph.
    let source = include_str!("hud.rs");
    let rows = source.split_once("const FONT_ROWS: [&str; 10] = [").expect("bitmap font inventory").1.split_once("];").expect("end of font inventory").0;
    let mut characters = BTreeSet::new();
    for row in rows.lines().map(str::trim).filter(|row| row.starts_with('"')) {
        let literal = row.strip_suffix(',').unwrap_or(row).strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        let mut chars = literal.chars();
        while let Some(ch) = chars.next() {
            let decoded = match ch {
                '\\' => match chars.next().expect("escape character") {
                    '"' => '"',
                    '\\' => '\\',
                    'u' => {
                        assert_eq!(chars.next(), Some('{'));
                        let hex: String = chars.by_ref().take_while(|&ch| ch != '}').collect();
                        char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap()
                    }
                    other => panic!("unexpected font inventory escape: {other}"),
                },
                other => other,
            };
            characters.insert(decoded);
        }
    }
    assert!(!characters.is_empty());
    characters
}

#[test]
fn in_game_port_text_has_glyphs_in_the_existing_bitmap_fallback() {
    let characters = fallback_bitmap_characters();
    for &(key, _, _, _) in ENTRIES {
        // These are native OS dialogs; they do not use hud::TextFont.
        if matches!(key, Text::GpuDriverTitle | Text::GpuDriverMessage | Text::GpuDriverHelp
            | Text::GpuUnsupportedTitle | Text::GpuUnsupportedMessage | Text::GpuRequirements
            | Text::GpuAlternative | Text::GpuGraphicsLabel | Text::GpuNone | Text::GpuOpenGlOnly
            | Text::GpuDetailsLabel | Text::GpuLogLocation) {
            continue;
        }
        for language in [Language::English, Language::Spanish] {
            let original = text(language, key);
            // Game::show_message uppercases these status messages before drawing.
            let displayed = if matches!(key, Text::BuildStreetRaceFailed | Text::VehicleReady | Text::VehicleLoadFailed) {
                original.to_uppercase()
            } else {
                original.to_string()
            };
            let missing: BTreeSet<_> = displayed.chars().filter(|ch| !ch.is_whitespace()
                && !characters.contains(ch) && !characters.contains(&ch.to_ascii_uppercase())).collect();
            assert!(missing.is_empty(), "{key:?} ({language:?}) uses unsupported bitmap glyphs: {missing:?}");
        }
    }
}
