//! Present the selected original collectable template without adding a second counter.
//!
//! Original string-table runtime variables use `<D>name</D>` (also filled by
//! `mc4b::fill_d` for goal times and the named bank-amount replacement in `game`).
//! Only the known collectable-count variable belongs to this caller. Wording,
//! punctuation, any denominator and unrelated variables remain in the resource.
//! The original count token uses underscores: `<D>NUM_COLLECTABLE_COLLECTED</D>`.

use mcla_formats::strtbl::StringTable;

pub fn found_message(strings: &StringTable, collected: usize, total: usize) -> String {
    match strings.get("COLLECTABLE_FOUND_MSG") {
        Some(template) => template.replace("<D>NUM_COLLECTABLE_COLLECTED</D>", &collected.to_string()),
        // Preserve the port's existing English fallback only when no resource is found.
        None => format!("COLLECTABLE FOUND   {collected} / {total}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcla_config::Language;
    use mcla_formats::hash::joaat;

    // Reduced in-memory tables; tests distinguish reported original text from synthetic sentinels.
    fn table(en: &[(&str, &str)], es: &[(&str, &str)]) -> StringTable {
        StringTable {
            keys: vec!["COLLECTABLE_FOUND_MSG".to_string()],
            langs: [en, es].into_iter().map(|rows| rows.iter().map(|(key, value)| (joaat(key), (*value).to_string())).collect()).collect(),
            lang: 0,
        }
    }

    #[test]
    fn authentic_reported_templates_preserve_counts_and_spanish_double_space() {
        // Transcribed from the user's INFORME_COLECCIONABLES_XBOX360.md, 2026-11-05.
        // Reported UI-table SHA-256:
        // c01f84edf4563d21084140839b9ead10405f97ae01949ad4abc90b5fb8a3965d
        // The test embeds no binary resource and does not claim to inspect that binary.
        let en = "COLLECTABLE <D>NUM_COLLECTABLE_COLLECTED</D> OF 60 FOUND";
        let es = "OBJETO COLECC.  <D>NUM_COLLECTABLE_COLLECTED</D> DE 60 ENCONTRADO";
        let mut strings = table(&[("COLLECTABLE_FOUND_MSG", en)], &[("COLLECTABLE_FOUND_MSG", es)]);
        for language in [Language::English, Language::Spanish] {
            strings.set_lang(language.table_index());
            let before = strings.clone();
            for count in [0, 1, 2, 3, 10, 20, 30, 40, 50, 60] {
                let expected = if language == Language::English {
                    format!("COLLECTABLE {count} OF 60 FOUND")
                } else {
                    format!("OBJETO COLECC.  {count} DE 60 ENCONTRADO")
                };
                assert_eq!(found_message(&strings, count, 60), expected);
                // The original text's denominator and Spanish double space remain verbatim.
                assert_eq!(found_message(&strings, count, 5000), expected);
            }
            assert_eq!(strings.keys, before.keys);
            assert_eq!(strings.langs, before.langs);
            assert_eq!(strings.lang, before.lang);
        }
    }

    #[test]
    fn original_language_template_gets_one_count_without_an_appended_counter() {
        // Synthetic sentinels exercise arbitrary layout and unrelated placeholders.
        let en = "EN_SENTINEL: <D>NUM_COLLECTABLE_COLLECTED</D> / 60 (%s)";
        let es = "ES_SENTINEL áéíóúñ: [<D>NUM_COLLECTABLE_COLLECTED</D>/60]\n¿? {marker}";
        let mut strings = table(&[("COLLECTABLE_FOUND_MSG", en)], &[("COLLECTABLE_FOUND_MSG", es)]);
        for (language, template) in [(Language::English, en), (Language::Spanish, es)] {
            strings.set_lang(language.table_index());
            let before = strings.clone();
            for count in [0, 1, 2, 3, 9, 10, 49, 50, 59, 60, 100] {
                let expected = if language == Language::English {
                    format!("EN_SENTINEL: {count} / 60 (%s)")
                } else {
                    format!("ES_SENTINEL áéíóúñ: [{count}/60]\n¿? {{marker}}")
                };
                assert_eq!(found_message(&strings, count, 60), expected);
                // A caller's total must not invent or replace a resource's denominator.
                assert_eq!(found_message(&strings, count, 1000), expected);
                assert_eq!(strings.get("COLLECTABLE_FOUND_MSG"), Some(template));
            }
            assert_eq!(strings.keys, before.keys);
            assert_eq!(strings.langs, before.langs);
            assert_eq!(strings.lang, before.lang);
        }
    }

    #[test]
    fn only_the_named_count_is_replaced_and_resource_layout_is_preserved() {
        for (template, expected) in [
            ("PREFIX <D>NUM_COLLECTABLE_COLLECTED</D> END", "PREFIX 9 END"),
            ("<D>NUM_COLLECTABLE_COLLECTED</D> + <D>NUM_COLLECTABLE_COLLECTED</D>", "9 + 9"),
            ("<D>UNRELATED_VALUE</D>: <D>NUM_COLLECTABLE_COLLECTED</D>%", "<D>UNRELATED_VALUE</D>: 9%"),
            ("VERBATIM_LITERAL_COUNTER 7 / 60", "VERBATIM_LITERAL_COUNTER 7 / 60"),
            ("<D>UNRELATED_VALUE</D>", "<D>UNRELATED_VALUE</D>"),
            // The old implementation guessed this spelling; variable names are not normalized.
            ("<D>NUM COLLECTABLE COLLECTED</D>", "<D>NUM COLLECTABLE COLLECTED</D>"),
            ("MALFORMED <D>NUM_COLLECTABLE_COLLECTED", "MALFORMED <D>NUM_COLLECTABLE_COLLECTED"),
        ] {
            let mut strings = table(&[("COLLECTABLE_FOUND_MSG", template)], &[("COLLECTABLE_FOUND_MSG", template)]);
            for language in [Language::English, Language::Spanish] {
                strings.set_lang(language.table_index());
                assert_eq!(found_message(&strings, 9, 60), expected);
            }
        }
    }

    #[test]
    fn missing_spanish_key_keeps_the_existing_english_resource_fallback() {
        let en = "EN_FALLBACK <D>NUM_COLLECTABLE_COLLECTED</D>/60";
        for es in [vec![("unrelated", "ES_SENTINEL")], vec![("COLLECTABLE_FOUND_MSG", "")]] {
            let mut strings = table(&[("COLLECTABLE_FOUND_MSG", en)], &es);
            strings.set_lang(Language::Spanish.table_index());
            assert_eq!(strings.lang, 1);
            assert_eq!(found_message(&strings, 10, 60), "EN_FALLBACK 10/60");
            assert_eq!(strings.lang, 1);
            assert_eq!(strings.get("COLLECTABLE_FOUND_MSG"), Some(en));
        }
    }

    #[test]
    fn absent_resources_preserve_the_previous_english_port_fallback() {
        let mut strings = table(&[("unrelated", "EN_SENTINEL")], &[("unrelated", "ES_SENTINEL")]);
        for language in [Language::English, Language::Spanish] {
            strings.set_lang(language.table_index());
            for (count, total) in [(0, 60), (1, 60), (2, 60), (3, 60), (9, 10), (49, 50), (59, 60), (60, 60), (100, 100)] {
                assert_eq!(found_message(&strings, count, total), format!("COLLECTABLE FOUND   {count} / {total}"));
            }
        }
    }
}
