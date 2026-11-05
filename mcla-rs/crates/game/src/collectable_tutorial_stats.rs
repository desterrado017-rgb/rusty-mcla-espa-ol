//! Statistics/effect bridge for the verified Collectables.sco form-94 sequence.
//!
//! This is deliberately not connected to the live pickup path: the original
//! form's close controls and pause contract still need verification. A future
//! adapter must obtain `Completed` from the form's actual visible/result queries.
//! No new persistent flag, tutorial-list entry or save-file route is introduced.

use crate::collectable_tutorial::{Completed, Eligibility, Flow, Pickup, Start};
use crate::game::Game;

pub const AUTOSAVE_EVENT: &str = "CAREER_AUTOSAVE";

impl Game {
    /// The script's eligibility is xTCO, not gCOL or the number of collected items.
    /// This only starts a transient request; it never marks the tutorial seen.
    pub fn collectable_tutorial_begin(&self, flow: &mut Flow, gates: Eligibility, total: usize) -> Start {
        flow.begin(Pickup { collected: self.collected.len(), total }, self.mc4_stat("xTCO", 0), gates)
    }

    /// Consume one actual form completion: assign xTCO, request autosave, then post progress.
    ///
    /// The callback requests the existing autosave path; this backend does not write files.
    /// A future App adapter must retain that path's no-save/error behavior. The bool
    /// reports whether StatsManager accepted its assignment. As in the script, its
    /// result does not prevent the following autosave request and progress notice.
    /// A token from another/inactive flow is rejected without performing any effect.
    pub fn collectable_tutorial_finish(&mut self, flow: &mut Flow, completed: Completed, post_event: impl FnOnce(&Game, &str)) -> bool {
        if !flow.completion_pending(&completed) {
            return false;
        }
        let pickup = completed.pickup();
        let written = self.mc4_stat_submit("xTCO", 0, 3, 1.0);
        post_event(self, AUTOSAVE_EVENT);
        let progress = crate::collectable_text::found_message(&self.career.strings, pickup.collected, pickup.total);
        self.show_message(&progress.to_uppercase());
        // Keep the controller busy through the entire post-close sequence. Do not
        // put this state change inside debug_assert: it must also run in release.
        let acknowledged = flow.finish_effects(completed);
        debug_assert!(acknowledged, "the validated completion must still belong to this flow");
        written
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{career::SaveState, collectable_tutorial::Query, game::SaveData};
    use mcla_config::Language;
    use mcla_formats::{hash::joaat, strtbl::StringTable};
    use serde_json::{json, Value};

    // Short text fixtures transcribed from INFORME_COLECCIONABLES_XBOX360.md,
    // provided by the user. Original table SHA-256:
    // c01f84edf4563d21084140839b9ead10405f97ae01949ad4abc90b5fb8a3965d.
    // No original binary is copied, and production continues to load the player's table.
    const EN_PROGRESS: &str = "COLLECTABLE <D>NUM_COLLECTABLE_COLLECTED</D> OF 60 FOUND";
    const ES_PROGRESS: &str = "OBJETO COLECC.  <D>NUM_COLLECTABLE_COLLECTED</D> DE 60 ENCONTRADO";

    fn game(language: Language, collected: usize) -> Game {
        let mut game = Game::new(Vec::new(), 0);
        game.collected = (0..collected as u16).collect();
        game.career.strings = StringTable {
            keys: vec!["COLLECTABLE_FOUND_MSG".to_string()],
            langs: [EN_PROGRESS, ES_PROGRESS].into_iter().map(|text| [(joaat("COLLECTABLE_FOUND_MSG"), text.to_string())].into_iter().collect()).collect(),
            lang: language.table_index(),
        };
        game.show_message("PREVIOUS MESSAGE");
        game
    }

    fn game_with_career(language: Language, collected: usize) -> Game {
        // Career::restore deliberately starts a new career (clearing statistics)
        // if it has no non-locked mission. Load a tiny synthetic mission graph so
        // these tests exercise the real restore path, then keep all saves in memory.
        // No player resource/save directory or legacy migration is accessed.
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!("mcla-collectable-stats-{}-{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        struct Fixture(std::path::PathBuf);
        impl Drop for Fixture {
            fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
        }
        let fixture = Fixture(root);
        std::fs::create_dir_all(fixture.0.join("tune/career/missions")).unwrap();
        std::fs::write(fixture.0.join("tune/career/missions/RACE_GH_06.xml"), r#"
            <Mission>
              <szId content="ascii">RACE_GH_06</szId>
              <aObjectives><Item type="mcRequirement">
                <szDescriptionId content="ascii">SYNTHETIC_UNFINISHED_OBJECTIVE</szDescriptionId>
                <aConditions><Item type="mcConditionEvent">
                  <szEventId content="ascii">SYNTHETIC_UNFINISHED_EVENT</szEventId>
                </Item></aConditions>
              </Item></aObjectives>
            </Mission>"#).unwrap();
        let mut game = game(language, collected);
        let strings = std::mem::take(&mut game.career.strings);
        game.career = crate::career::Career::load(fixture.0.to_str().unwrap());
        assert!(game.career.loaded());
        game.career.new_game();
        assert!(!game.career.save().missions.is_empty());
        game.career.strings = strings;
        game
    }

    fn completed(game: &Game) -> (Flow, Completed) {
        let mut flow = Flow::default();
        assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::ShowForm(94)));
        assert!(flow.shown(94));
        assert!(matches!(flow.tick(0.0), Some(Query::Visible(94))));
        assert!(matches!(flow.visibility(94, Some(false)), Some(Query::Result(94))));
        let completed = flow.result(94, 0).expect("verified form closure and result");
        assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::Busy));
        (flow, completed)
    }

    #[test]
    fn eligibility_uses_existing_xtco_sources_without_changing_saved_state_or_gcol() {
        for (stats, ostats, expected) in [
            (None, None, 0.0),
            (Some(0), None, 0.0),
            (Some(1), None, 1.0),
            (None, Some(1.0), 1.0),
            (Some(7), Some(1.0), 7.0),
            (Some(-1), None, -1.0),
            // Retain the port's existing precedence; never reconcile a save on load.
            (Some(0), Some(1.0), 0.0),
        ] {
            let mut game = game(Language::Spanish, 17);
            game.mc4_stat_set("gCOL", 0, 60.0);
            if let Some(value) = stats { game.career.stats.insert("xTCO".to_string(), value); }
            if let Some(value) = ostats { game.career.ostats.insert("xTCO/0".to_string(), value); }
            let before = serde_json::to_value(game.career.save()).unwrap();
            let mut flow = Flow::default();
            let start = game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60);
            if expected == 0.0 {
                assert!(matches!(start, Start::ShowForm(94)));
            } else {
                assert!(matches!(start, Start::Progress(Pickup { collected: 17, total: 60 })));
            }
            assert_eq!(game.mc4_stat("xTCO", 0), expected);
            assert_eq!(game.mc4_stat("gCOL", 0), 60.0);
            assert_eq!(game.collected, (0..17).collect::<Vec<_>>());
            assert_eq!(serde_json::to_value(game.career.save()).unwrap(), before);
            assert_eq!(game.message, "PREVIOUS MESSAGE");
            assert!(game.mc4.is_none() && game.mc4_raw.is_none());
        }
    }

    #[test]
    fn suppressed_tutorial_keeps_xtco_zero_and_can_become_eligible_on_a_later_pickup() {
        let gates = [
            Eligibility { no_tutorial: true, ..Default::default() },
            Eligibility { networked: true, ..Default::default() },
            Eligibility { connecting: true, ..Default::default() },
            Eligibility { editor: true, ..Default::default() },
        ];
        for gate in gates {
            let mut game = game(Language::English, 17);
            game.mc4_stat_set("gCOL", 0, 17.0);
            let mut flow = Flow::default();
            assert!(matches!(game.collectable_tutorial_begin(&mut flow, gate, 60), Start::Progress(Pickup { collected: 17, total: 60 })));
            assert_eq!(game.mc4_stat("xTCO", 0), 0.0);
            assert!(!game.mc4_stat_present("xTCO", 0));
            assert!(flow.result(94, 0).is_none());
            game.collected.push(17);
            assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::ShowForm(94)));
            assert_eq!(game.mc4_stat("gCOL", 0), 17.0);
        }
    }

    #[test]
    fn xtco_is_written_only_after_close_then_autosave_is_requested_before_authentic_progress() {
        for language in [Language::English, Language::Spanish] {
            for count in [1, 2, 3, 10, 20, 60] {
                let mut game = game(language, count);
                game.mc4_stat_set("gCOL", 0, count as f64);
                game.mc4_stat_set("xOTHER", 0, 73.0);
                let mut flow = Flow::default();
                assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::ShowForm(94)));
                assert_eq!(game.mc4_stat("xTCO", 0), 0.0);
                assert!(!game.mc4_stat_present("xTCO", 0));
                assert_eq!(game.message, "PREVIOUS MESSAGE");
                assert!(flow.shown(94));
                assert!(matches!(flow.tick(0.1), Some(Query::Visible(94))));
                assert!(flow.visibility(94, Some(true)).is_none());
                assert!(flow.result(94, 0).is_none());
                assert_eq!(game.mc4_stat("xTCO", 0), 0.0);
                assert!(matches!(flow.tick(0.1), Some(Query::Visible(94))));
                assert!(matches!(flow.visibility(94, Some(false)), Some(Query::Result(94))));
                let done = flow.result(94, 0).expect("actual close result");
                let mut autosave_state = None;
                assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::Busy));
                assert!(game.collectable_tutorial_finish(&mut flow, done, |at_request, event| {
                    assert_eq!(event, AUTOSAVE_EVENT);
                    assert_eq!(at_request.mc4_stat("xTCO", 0), 1.0);
                    assert_eq!(at_request.career.stats.get("xTCO"), Some(&1));
                    assert_eq!(at_request.career.ostats.get("xTCO/0"), Some(&1.0));
                    assert_eq!(at_request.message, "PREVIOUS MESSAGE");
                    autosave_state = Some(serde_json::to_value(at_request.career.save()).unwrap());
                }));
                assert!(autosave_state.is_some());
                assert_eq!(game.message, match language {
                    Language::English => format!("COLLECTABLE {count} OF 60 FOUND"),
                    Language::Spanish => format!("OBJETO COLECC.  {count} DE 60 ENCONTRADO"),
                });
                assert_eq!(game.message_timer, 3.0);
                assert_eq!(game.mc4_stat("gCOL", 0), count as f64);
                assert_eq!(game.mc4_stat("xOTHER", 0), 73.0);
                assert!(game.mc4.is_none()); // no auxiliary tutorial-list/block creation
                assert!(!flow.is_active());
                assert!(flow.result(94, 0).is_none()); // the completion token is one-shot
            }
        }
    }

    #[test]
    fn career_and_save_data_json_round_trip_preserve_xtco_and_do_not_replay_seen_tutorials() {
        for original_value in [0, 1, 7] {
            let mut before = game_with_career(Language::Spanish, 23);
            // Same integer/sub-id-zero representation produced by the original-save importer.
            before.career.stats.insert("xTCO".to_string(), original_value);
            before.career.ostats.insert("xTCO/0".to_string(), original_value as f64);
            before.mc4_stat_set("gCOL", 0, 19.0); // preserve an independently stored counter
            before.mc4_stat_set("xOTHER", 0, 42.0);
            let saved = SaveData { career: Some(before.career.save()), collected: before.collected.clone(), ..Default::default() };
            let bytes = serde_json::to_vec(&saved).unwrap();
            let loaded: SaveData = serde_json::from_slice(&bytes).unwrap();
            let mut after = game_with_career(Language::Spanish, 0);
            after.collected = loaded.collected;
            after.career.restore(loaded.career.as_ref().unwrap());
            assert_eq!(serde_json::to_value(after.career.save()).unwrap(), serde_json::to_value(before.career.save()).unwrap());
            let mut flow = Flow::default();
            let start = after.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60);
            if original_value == 0 { assert!(matches!(start, Start::ShowForm(94))); }
            else { assert!(matches!(start, Start::Progress(Pickup { collected: 23, total: 60 }))); }
            assert_eq!(after.mc4_stat("xTCO", 0), original_value as f64);
            assert_eq!(after.mc4_stat("gCOL", 0), 19.0);
            assert_eq!(after.mc4_stat("xOTHER", 0), 42.0);
        }
    }

    #[test]
    fn post_close_snapshot_survives_reload_without_replacing_an_unsupported_mc4_block() {
        let raw = json!("UNSUPPORTED_MC4_SENTINEL");
        let mut game = game_with_career(Language::Spanish, 1);
        game.mc4_load(Some(raw.clone())); // fails type parsing before any resource loading
        assert_eq!(game.mc4_raw, Some(raw.clone()));
        let (mut flow, done) = completed(&game);
        let mut snapshot = None;
        assert!(game.collectable_tutorial_finish(&mut flow, done, |at_request, _| {
            assert_eq!(at_request.mc4_save(), Some(raw.clone()));
            snapshot = Some(SaveData { career: Some(at_request.career.save()), collected: at_request.collected.clone(), mc4: at_request.mc4_save(), ..Default::default() });
        }));
        assert_eq!(game.mc4_raw, Some(raw.clone()));
        let bytes = serde_json::to_vec(&snapshot.unwrap()).unwrap();
        let loaded: SaveData = serde_json::from_slice(&bytes).unwrap();
        let mut reloaded = game_with_career(Language::Spanish, 0);
        reloaded.collected = loaded.collected;
        reloaded.career.restore(loaded.career.as_ref().unwrap());
        reloaded.mc4_load(loaded.mc4);
        assert_eq!(reloaded.mc4_save(), Some(raw));
        let mut flow = Flow::default();
        assert!(matches!(reloaded.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::Progress(Pickup { collected: 1, total: 60 })));
        assert_eq!(reloaded.mc4_stat("xTCO", 0), 1.0);
    }

    #[test]
    fn unfinished_form_stays_unacknowledged_in_a_memory_save() {
        let game = game_with_career(Language::English, 1);
        let mut flow = Flow::default();
        assert!(matches!(game.collectable_tutorial_begin(&mut flow, Eligibility::default(), 60), Start::ShowForm(94)));
        assert!(flow.shown(94));
        let snapshot: SaveState = serde_json::from_value(serde_json::to_value(game.career.save()).unwrap()).unwrap();
        assert!(!snapshot.stats.contains_key("xTCO") && !snapshot.ostats.contains_key("xTCO/0"));
        let mut reloaded = game_with_career(Language::English, 2);
        reloaded.career.restore(&snapshot);
        let mut next = Flow::default();
        assert!(matches!(reloaded.collectable_tutorial_begin(&mut next, Eligibility::default(), 60), Start::ShowForm(94)));
    }

    #[test]
    fn original_stats_capacity_restrictions_are_not_bypassed_by_completion() {
        for existing in [false, true] {
            let mut game = game(Language::English, 1);
            if existing { game.mc4_stat_set("xTCO", 0, 0.0); }
            let room = crate::mc4::STAT_RECORDS - game.career.ostats.len();
            for i in 0..room { game.career.ostats.insert(format!("FILL{i}/0"), 1.0); }
            assert_eq!(game.career.ostats.len(), crate::mc4::STAT_RECORDS);
            let (mut flow, done) = completed(&game);
            let mut requested = false;
            let written = game.collectable_tutorial_finish(&mut flow, done, |at_request, event| {
                requested = true;
                assert_eq!(event, AUTOSAVE_EVENT);
                assert_eq!(at_request.mc4_stat("xTCO", 0), if existing { 1.0 } else { 0.0 });
                assert_eq!(at_request.message, "PREVIOUS MESSAGE");
            });
            assert_eq!(written, existing);
            assert!(requested);
            assert_eq!(game.career.ostats.len(), crate::mc4::STAT_RECORDS);
            assert_eq!(game.message, "COLLECTABLE 1 OF 60 FOUND");
        }
    }

    #[test]
    fn typed_mc4_extra_fields_and_tutorial_list_are_not_changed_by_the_statistics_bridge() {
        let mut game = game(Language::English, 1);
        let mut mc4 = crate::mc4::Mc4::default();
        mc4.tutorials_shown = vec!["tTBM".to_string()];
        mc4.extra.insert("UNKNOWN_MC4_SENTINEL".to_string(), json!({"keep": [1, 2, 3]}));
        mc4.stats_summary.insert("SUMMARY_SENTINEL".to_string(), json!(73));
        let before: Value = serde_json::to_value(&mc4).unwrap();
        game.mc4 = Some(mc4);
        let (mut flow, done) = completed(&game);
        assert!(game.collectable_tutorial_finish(&mut flow, done, |_, event| assert_eq!(event, AUTOSAVE_EVENT)));
        assert_eq!(serde_json::to_value(game.mc4.as_ref().unwrap()).unwrap(), before);
        assert!(!game.mc4.as_ref().unwrap().tutorials_shown.iter().any(|flag| flag == "xTCO"));
    }

    #[test]
    fn completion_from_another_flow_is_rejected_before_any_side_effect_even_for_the_same_pickup() {
        let mut game = game(Language::English, 1);
        let (mut first, wrong_token) = completed(&game);
        let (mut second, correct_token) = completed(&game);
        let before = serde_json::to_value(game.career.save()).unwrap();
        let mut event_called = false;
        assert!(!game.collectable_tutorial_finish(&mut second, wrong_token, |_, _| event_called = true));
        assert!(!event_called);
        assert_eq!(serde_json::to_value(game.career.save()).unwrap(), before);
        assert_eq!(game.message, "PREVIOUS MESSAGE");
        assert!(matches!(game.collectable_tutorial_begin(&mut first, Eligibility::default(), 60), Start::Busy));
        assert!(matches!(game.collectable_tutorial_begin(&mut second, Eligibility::default(), 60), Start::Busy));
        assert!(game.collectable_tutorial_finish(&mut second, correct_token, |at_request, event| {
            event_called = true;
            assert_eq!(event, AUTOSAVE_EVENT);
            assert_eq!(at_request.mc4_stat("xTCO", 0), 1.0);
            assert_eq!(at_request.message, "PREVIOUS MESSAGE");
        }));
        assert!(event_called);
        assert!(!second.is_active());
        assert_eq!(game.message, "COLLECTABLE 1 OF 60 FOUND");
    }
}
