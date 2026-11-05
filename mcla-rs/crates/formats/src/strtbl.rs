//! The game's localised string table (`ui/0xa4c1e8fa`, little-endian):
//! `u32 language count, u32 offset per language, u32 version, u32 key count, keys
//! {u32 length, bytes, NUL}`; every language block is `u32 count` followed by
//! `{u32 joaat(key), u16 flags, u32 font name length, font name, u32 UTF-16 length
//! (with terminator), UTF-16LE text, f32 scale x, f32 scale y, u16}` entries.
//! Language 0 is English.

use crate::hash::joaat;
use crate::{bail, Result};
use hashbrown::HashMap;

#[derive(Default, Clone)]
pub struct StringTable {
    pub keys: Vec<String>,
    pub langs: Vec<HashMap<u32, String>>,
    /// the language block `get` reads (0 English, 1 Spanish, 2 French, 3 German, 4 Italian)
    pub lang: usize,
}

impl StringTable {
    pub fn parse(d: &[u8]) -> Result<StringTable> {
        let u32at = |o: usize| -> Result<usize> {
            if o + 4 > d.len() {
                bail!("string table truncated");
            }
            Ok(u32::from_le_bytes(d[o..o + 4].try_into().unwrap()) as usize)
        };
        let n = u32at(0)?;
        if n == 0 || n > 64 {
            bail!("not a string table");
        }
        let mut offs = Vec::new();
        for i in 0..n {
            offs.push(u32at(4 + i * 4)?);
        }
        let mut p = 4 + 4 * n + 4;
        let count = u32at(p)?;
        p += 4;
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            let l = u32at(p)?;
            p += 4;
            if p + l + 1 > d.len() {
                bail!("string table keys truncated");
            }
            keys.push(String::from_utf8_lossy(&d[p..p + l]).to_string());
            p += l + 1;
        }
        let mut langs = Vec::new();
        let mut seen: Vec<usize> = Vec::new();
        for &o in &offs {
            // unused languages point at the end of the file or repeat a block
            if o + 4 > d.len() || seen.contains(&o) {
                langs.push(HashMap::new());
                continue;
            }
            seen.push(o);
            let c = u32at(o)?;
            let mut q = o + 4;
            let mut m = HashMap::with_capacity(c);
            for _ in 0..c {
                if q + 10 > d.len() {
                    break;
                }
                let h = u32at(q)? as u32;
                let fl = u32at(q + 6)?;
                q += 10 + fl;
                let tl = u32at(q)?;
                q += 4;
                if q + tl * 2 + 10 > d.len() {
                    break;
                }
                let units: Vec<u16> = d[q..q + tl * 2].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).take_while(|&u| u != 0).collect();
                m.insert(h, String::from_utf16_lossy(&units));
                q += tl * 2 + 10;
            }
            langs.push(m);
        }
        Ok(StringTable { keys, langs, lang: 0 })
    }

    /// Add the entries of another table (DLC and tip tables next to the main one).
    pub fn merge(&mut self, other: StringTable) {
        self.keys.extend(other.keys);
        for (i, l) in other.langs.into_iter().enumerate() {
            if i >= self.langs.len() {
                self.langs.push(l);
            } else {
                for (k, v) in l {
                    self.langs[i].entry(k).or_insert(v);
                }
            }
        }
    }

    /// The language to read: a block the table has and that holds text; anything else is
    /// English.
    pub fn set_lang(&mut self, lang: usize) {
        self.lang = if self.langs.get(lang).map_or(false, |m| !m.is_empty()) { lang } else { 0 };
    }

    /// Text of a key in the table's language, in English where that has no entry or
    /// an empty entry (keys are case-insensitive). Text is returned unmodified.
    pub fn get(&self, key: &str) -> Option<&str> {
        let h = joaat(key);
        self.langs.get(self.lang).and_then(|m| m.get(&h)).filter(|s| !s.is_empty()).or_else(|| self.langs.first()?.get(&h)).map(|s| s.as_str())
    }

    pub fn get_or<'a>(&'a self, key: &'a str) -> &'a str {
        self.get(key).unwrap_or(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic sentinels, not translations or copies of the disc's strings.
    fn table(en: &[(&str, &str)], es: &[(&str, &str)]) -> StringTable {
        StringTable {
            keys: Vec::new(),
            langs: [en, es].into_iter().map(|rows| rows.iter().map(|(k, v)| (joaat(k), (*v).into())).collect()).collect(),
            lang: 0,
        }
    }

    #[test]
    fn spanish_is_verbatim_with_english_fallback_per_key() {
        let text = "ES_\u{a1}\u{bf}\u{e1}\u{e9}\u{ed}\u{f3}\u{fa}\u{fc}\u{f1}\n<token> %s ~1~";
        let mut t = table(&[("present", "EN_PRESENT"), ("missing", "EN_MISSING"), ("empty", "EN_EMPTY")], &[("present", text), ("empty", "")]);
        let original = t.langs.clone();
        t.set_lang(1);
        assert_eq!(t.get("PRESENT"), Some(text));
        assert_eq!(t.get("missing"), Some("EN_MISSING"));
        assert_eq!(t.get("empty"), Some("EN_EMPTY"));
        assert_eq!(t.get("unknown"), None);
        assert_eq!(t.get_or("unknown"), "unknown");
        t.set_lang(0);
        assert_eq!(t.get("present"), Some("EN_PRESENT"));
        assert_eq!(t.langs, original, "selecting and reading never rewrites original text");
    }

    #[test]
    fn absent_language_falls_back_to_english() {
        let mut t = table(&[("only", "EN_ONLY")], &[]);
        for lang in [1, 12, usize::MAX] {
            t.set_lang(lang);
            assert_eq!(t.lang, 0);
            assert_eq!(t.get("only"), Some("EN_ONLY"));
        }
        assert_eq!(StringTable::default().get("missing"), None);
    }

    #[test]
    fn select_language_after_merging_dlc_and_keep_main_precedence() {
        let mut t = table(&[("main", "EN_MAIN"), ("shared", "EN_MAIN_SHARED")], &[]);
        // The main table alone has no Spanish, but the DLC supplies that block.
        t.merge(table(&[("dlc", "EN_DLC"), ("shared", "EN_DLC_SHARED")], &[("dlc", "ES_DLC")]));
        t.merge(table(&[("tip", "EN_TIP")], &[("dlc", "ES_LATER"), ("tip", "ES_TIP")]));
        t.set_lang(1);
        assert_eq!(t.lang, 1);
        assert_eq!(t.get("main"), Some("EN_MAIN"));
        assert_eq!(t.get("dlc"), Some("ES_DLC"));
        assert_eq!(t.get("shared"), Some("EN_MAIN_SHARED"));
        assert_eq!(t.get("tip"), Some("ES_TIP"));
    }
}
