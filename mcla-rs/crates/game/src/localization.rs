//! Read the player's original tables without changing them. Boot and career must use
//! the same merge order, and select the language only after all DLC tables are present.

use mcla_config::Language;
use mcla_formats::strtbl::StringTable;
use std::path::Path;

pub fn load(data_dir: &str, language: Language) -> StringTable {
    let ui = Path::new(data_dir).join("ui");
    let mut strings = match std::fs::read(ui.join("0xa4c1e8fa")) {
        Ok(bytes) => match StringTable::parse(&bytes) {
            Ok(table) => table,
            Err(e) => {
                log::warn!("string table: {}", e);
                StringTable::default()
            }
        },
        Err(_) => StringTable::default(),
    };
    if let Ok(rd) = std::fs::read_dir(&ui) {
        // Same files as v0.13.0: main first, then South Central, tips and police
        // in name order. The other excluded file is not a string table.
        let mut files: Vec<_> = rd.flatten().map(|e| e.path()).filter(|p| p.is_file() && p.file_name().is_some_and(|n| n != "0xa4c1e8fa" && n != "0xac5297d5")).collect();
        files.sort();
        for file in files {
            if let Some(table) = std::fs::read(&file).ok().and_then(|d| StringTable::parse(&d).ok()) {
                strings.merge(table);
            }
        }
    }
    strings.set_lang(language.table_index());
    log::info!("text language: {} (table block {})", language.word(), strings.lang);
    strings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct Fixture(std::path::PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU32 = AtomicU32::new(0);
            let root = std::env::temp_dir().join(format!("mcla-language-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir_all(root.join("ui")).unwrap();
            Self(root)
        }

        fn path(&self) -> &str {
            self.0.to_str().unwrap()
        }

        fn write(&self, name: &str, en: &[(&str, &str)], es: &[(&str, &str)]) -> Vec<u8> {
            // Original binary layout, but entirely synthetic data: no disc text.
            let mut bytes: Vec<u8> = [2u32, 0, 0, 1, 0].into_iter().flat_map(u32::to_le_bytes).collect();
            for (i, rows) in [en, es].into_iter().enumerate() {
                let offset = bytes.len() as u32;
                bytes[4 + i * 4..8 + i * 4].copy_from_slice(&offset.to_le_bytes());
                bytes.extend_from_slice(&(rows.len() as u32).to_le_bytes());
                for (key, text) in rows {
                    bytes.extend_from_slice(&mcla_formats::hash::joaat(key).to_le_bytes());
                    bytes.extend_from_slice(&0u16.to_le_bytes()); // flags
                    bytes.extend_from_slice(&0u32.to_le_bytes()); // font name length
                    let units: Vec<_> = text.encode_utf16().chain(std::iter::once(0)).collect();
                    bytes.extend_from_slice(&(units.len() as u32).to_le_bytes());
                    bytes.extend(units.into_iter().flat_map(u16::to_le_bytes));
                    bytes.extend_from_slice(&[0; 10]); // scales and trailing flags
                }
            }
            std::fs::write(self.0.join("ui").join(name), &bytes).unwrap();
            bytes
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn language_is_selected_after_south_central_and_tip_tables_are_merged() {
        let fixture = Fixture::new();
        let main = fixture.write("0xa4c1e8fa", &[("main", "EN_MAIN"), ("shared", "EN_MAIN_SHARED")], &[]);
        let dlc = fixture.write("0x10000001", &[("dlc", "EN_DLC"), ("shared", "EN_DLC_SHARED")], &[("dlc", "ES_DLC_\u{f1}")]);
        fixture.write("0x10000002", &[("tip", "EN_TIP")], &[("tip", "ES_TIP"), ("dlc", "ES_LATER")]);
        fixture.write("0xac5297d5", &[("excluded", "EXCLUDED")], &[]);
        let strings = load(fixture.path(), Language::Spanish);
        assert_eq!(strings.lang, 1, "the main table alone has no Spanish block");
        assert_eq!(strings.get("main"), Some("EN_MAIN"));
        assert_eq!(strings.get("dlc"), Some("ES_DLC_\u{f1}"));
        assert_eq!(strings.get("tip"), Some("ES_TIP"));
        assert_eq!(strings.get("shared"), Some("EN_MAIN_SHARED"));
        assert_eq!(strings.get("excluded"), None);
        let english = load(fixture.path(), Language::English);
        assert_eq!(english.get("dlc"), Some("EN_DLC"));
        assert_eq!(std::fs::read(fixture.0.join("ui/0xa4c1e8fa")).unwrap(), main);
        assert_eq!(std::fs::read(fixture.0.join("ui/0x10000001")).unwrap(), dlc);
    }

    #[test]
    fn missing_main_table_does_not_hide_dlc_spanish() {
        let fixture = Fixture::new();
        fixture.write("0x10000001", &[("dlc", "EN_DLC")], &[("dlc", "ES_DLC")]);
        assert_eq!(load(fixture.path(), Language::Spanish).get("dlc"), Some("ES_DLC"));
    }

    #[test]
    fn english_only_and_missing_data_are_safe() {
        let fixture = Fixture::new();
        let empty = load(fixture.path(), Language::Spanish);
        assert_eq!(empty.get("missing"), None);
        fixture.write("0xa4c1e8fa", &[("only", "EN_ONLY")], &[]);
        let strings = load(fixture.path(), Language::Spanish);
        assert_eq!(strings.lang, 0);
        assert_eq!(strings.get("only"), Some("EN_ONLY"));
    }
}
