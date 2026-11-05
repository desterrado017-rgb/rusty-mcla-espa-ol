use super::*;
use mcla_config::Language;
use mcla_formats::hash::joaat;

// Reduced text fixtures supplied in INFORME_COLECCIONABLES_XBOX360.md, checked
// there against UI SHA-256 c01f84edf4563d21084140839b9ead10405f97ae01949ad4abc90b5fb8a3965d.
// No binary resource is included and these fixtures do not prove native controls.
const TITLE: &str = "tutorial_collectables_title";
const DESCRIPTION: &str = "tutorial_collectables_desc";
const EN_TITLE: &str = "COLLECTABLES";
const ES_TITLE: &str = "OBJETOS COLECCIONABLES";
const EN_DESCRIPTION: &str = "You just found a collectable.  Collectables can be used to unlock cheats.";
const ES_DESCRIPTION: &str = "Has encontrado un objeto coleccionable. Éstos sirven para desbloquear trucos.";

fn strings() -> StringTable {
    StringTable {
        keys: vec![TITLE.into(), DESCRIPTION.into(), "prompt_continue".into()],
        langs: [(EN_TITLE, EN_DESCRIPTION, "CONTINUE"), (ES_TITLE, ES_DESCRIPTION, "CONTINUAR")].into_iter().map(|(title, description, prompt)| {
            [(joaat(TITLE), title.to_string()), (joaat(DESCRIPTION), description.to_string()), (joaat("prompt_continue"), prompt.to_string())].into_iter().collect()
        }).collect(),
        lang: 0,
    }
}

fn finish(flow: &mut Flow, result: i32) -> Completed {
    assert!(flow.shown(FORM_ID));
    assert_eq!(flow.tick(0.1), Some(Query::Visible(FORM_ID)));
    assert_eq!(flow.visibility(FORM_ID, Some(false)), Some(Query::Result(FORM_ID)));
    flow.result(FORM_ID, result).expect("validated form result completes")
}

#[test]
fn original_tutorial_payload_uses_selected_authentic_text_without_control_assumptions() {
    let mut strings = strings();
    assert_eq!(crate::mc4::tutorial_keys(FORM_ID), Some((TITLE.into(), DESCRIPTION.into())));
    for (language, title, description) in [(Language::English, EN_TITLE, EN_DESCRIPTION), (Language::Spanish, ES_TITLE, ES_DESCRIPTION)] {
        strings.set_lang(language.table_index());
        let before = strings.clone();
        let page = tutorial_page(&strings, &[]).unwrap();
        assert_eq!(page.title, title);
        assert_eq!(page.layout, MenuLayout::Tutorial { description: description.into() });
        assert_eq!(page.select, usize::MAX);
        assert!(page.rows.is_empty() && page.prompts.is_empty());
        // A caller-supplied prompt is forwarded verbatim; icon 0 asserts no button mapping.
        let supplied = vec![(strings.get("prompt_continue").unwrap().to_string(), 0)];
        assert_eq!(tutorial_page(&strings, &supplied).unwrap().prompts, supplied);
        assert_eq!(strings.keys, before.keys);
        assert_eq!(strings.langs, before.langs);
        assert_eq!(strings.lang, before.lang);
    }
}

#[test]
fn tutorial_payload_retains_per_key_english_fallback_and_does_not_invent_missing_text() {
    let mut strings = strings();
    strings.set_lang(Language::Spanish.table_index());
    strings.langs[1].remove(&joaat(DESCRIPTION));
    let page = tutorial_page(&strings, &[]).unwrap();
    assert_eq!(page.title, ES_TITLE);
    assert_eq!(page.layout, MenuLayout::Tutorial { description: EN_DESCRIPTION.into() });
    strings.langs[0].remove(&joaat(DESCRIPTION));
    assert!(tutorial_page(&strings, &[]).is_none());
    assert!(tutorial_page(&StringTable::default(), &[]).is_none());
}

#[test]
fn xtco_and_original_gates_control_eligibility_not_the_collection_count() {
    for count in [1, 2, 3, 10, 59, 60] {
        let pickup = Pickup { collected: count, total: 60 };
        let mut flow = Flow::default();
        assert_eq!(flow.begin(pickup, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
        let completed = finish(&mut flow, 0);
        assert_eq!(completed.pickup(), pickup);
        assert!(flow.completion_pending(&completed));
        assert!(flow.finish_effects(completed));
        assert_eq!(flow.begin(pickup, 1.0, Eligibility::default()), Start::Progress(pickup));
        assert!(!flow.is_active());
    }
    let pickup = Pickup { collected: 1, total: 60 };
    for gates in [
        Eligibility { no_tutorial: true, ..Default::default() },
        Eligibility { networked: true, ..Default::default() },
        Eligibility { connecting: true, ..Default::default() },
        Eligibility { editor: true, ..Default::default() },
        Eligibility { no_tutorial: true, networked: true, connecting: true, editor: true },
    ] {
        let mut flow = Flow::default();
        assert_eq!(flow.begin(pickup, 0.0, gates), Start::Progress(pickup));
        assert!(!flow.is_active());
        // Suppression does not create a completion or consume future eligibility.
        assert!(flow.result(FORM_ID, 0).is_none());
        assert_eq!(flow.begin(Pickup { collected: 2, total: 60 }, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
    }
}

#[test]
fn completion_requires_actual_show_poll_visibility_and_result_in_order() {
    let pickup = Pickup { collected: 1, total: 60 };
    let mut flow = Flow::default();
    assert_eq!(flow.begin(pickup, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
    assert!(!flow.shown(95));
    assert!(flow.tick(10.0).is_none());
    assert!(flow.visibility(FORM_ID, Some(false)).is_none());
    assert!(flow.result(FORM_ID, 0).is_none());
    assert!(flow.shown(FORM_ID));
    assert!(flow.visibility(FORM_ID, Some(false)).is_none(), "no query is outstanding");
    assert!(flow.result(FORM_ID, 0).is_none());
    assert_eq!(flow.tick(0.1), Some(Query::Visible(FORM_ID)));
    assert!(flow.visibility(95, Some(false)).is_none());
    assert!(flow.tick(10.0).is_none(), "one unanswered query must not become a close");
    assert!(flow.visibility(FORM_ID, None).is_none());
    assert!(flow.result(FORM_ID, 0).is_none());
    assert_eq!(flow.tick(0.1), Some(Query::Visible(FORM_ID)));
    assert!(flow.visibility(FORM_ID, Some(true)).is_none());
    assert!(flow.result(FORM_ID, 0).is_none());
    assert_eq!(flow.tick(0.1), Some(Query::Visible(FORM_ID)));
    assert_eq!(flow.visibility(FORM_ID, Some(false)), Some(Query::Result(FORM_ID)));
    assert!(flow.result(95, 0).is_none());
    assert!(flow.tick(10.0).is_none());
    let completed = flow.result(FORM_ID, -1).unwrap();
    assert_eq!(completed.pickup(), pickup);
    assert!(flow.is_active() && flow.completion_pending(&completed));
    assert!(flow.result(FORM_ID, -1).is_none());
    assert_eq!(flow.begin(pickup, 0.0, Eligibility::default()), Start::Busy);
    assert!(flow.finish_effects(completed));
    assert!(!flow.is_active());
}

#[test]
fn polling_uses_unwarped_hundred_milliseconds_without_query_bursts() {
    let pickup = Pickup { collected: 3, total: 60 };
    let mut flow = Flow::default();
    assert_eq!(flow.begin(pickup, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
    assert!(flow.shown(FORM_ID));
    // The first native visibility query happens before its first WAITUNWARPED.
    assert_eq!(flow.tick(0.0), Some(Query::Visible(FORM_ID)));
    assert!(flow.visibility(FORM_ID, Some(true)).is_none());
    for _ in 0..3 {
        assert!(flow.tick(0.025).is_none());
    }
    for invalid in [-1.0, f32::NAN, f32::INFINITY] {
        assert!(flow.tick(invalid).is_none());
    }
    assert_eq!(flow.tick(0.025), Some(Query::Visible(FORM_ID)));
    assert!(flow.tick(2.0).is_none());
    assert!(flow.visibility(FORM_ID, Some(true)).is_none());
    assert!(flow.tick(0.05).is_none());
    assert_eq!(flow.tick(0.05), Some(Query::Visible(FORM_ID)));
}

#[test]
fn pending_pickups_do_not_reenter_and_form_results_are_opaque_one_shot_values() {
    for result in [i32::MIN, -1, 0, 1, i32::MAX] {
        let first = Pickup { collected: 1, total: 60 };
        let later = Pickup { collected: 2, total: 60 };
        let mut flow = Flow::default();
        assert_eq!(flow.begin(first, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
        assert_eq!(flow.begin(later, 0.0, Eligibility::default()), Start::Busy);
        let completed = finish(&mut flow, result);
        assert_eq!(completed.pickup(), first);
        assert!(flow.result(FORM_ID, result).is_none());
        assert_eq!(flow.begin(later, 0.0, Eligibility::default()), Start::Busy);
        assert_eq!(flow.begin(later, 1.0, Eligibility::default()), Start::Busy);
        assert!(flow.finish_effects(completed));
        assert_eq!(flow.begin(later, 1.0, Eligibility::default()), Start::Progress(later));
    }
}

#[test]
fn a_completion_from_another_flow_cannot_authorize_effects_for_the_same_pickup() {
    let pickup = Pickup { collected: 1, total: 60 };
    let mut first = Flow::default();
    let mut second = Flow::default();
    assert_eq!(first.begin(pickup, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
    assert_eq!(second.begin(pickup, 0.0, Eligibility::default()), Start::ShowForm(FORM_ID));
    let a = finish(&mut first, 0);
    let b = finish(&mut second, 0);
    assert!(first.completion_pending(&a) && second.completion_pending(&b));
    assert!(!first.completion_pending(&b) && !second.completion_pending(&a));
    assert!(!second.finish_effects(a));
    assert!(first.is_active() && second.is_active());
    assert!(second.completion_pending(&b));
    assert!(second.finish_effects(b));
    assert!(!second.is_active());
}
