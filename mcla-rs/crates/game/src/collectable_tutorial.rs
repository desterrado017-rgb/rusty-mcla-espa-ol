//! The verified Collectables.sco protocol around original form 94.
//!
//! This backend does not choose buttons, pause the game or show a runtime panel.
//! Those native-form details still need independent evidence. A future adapter
//! must acknowledge the actual form, answer its visibility query and return its
//! result before the caller may persist xTCO and perform the following effects.

use crate::flash_ui::{MenuLayout, MenuPage};
use mcla_formats::strtbl::StringTable;
use std::sync::atomic::{AtomicU64, Ordering};

pub const FORM_ID: u32 = 94;
const POLL_SECONDS: f64 = 0.1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pickup {
    pub collected: usize,
    pub total: usize,
}

/// The three original exclusions and GetNoTutorialCommandLine. Counts are not gates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Eligibility {
    pub no_tutorial: bool,
    pub networked: bool,
    pub connecting: bool,
    pub editor: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    ShowForm(u32),
    Progress(Pickup),
    Busy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Query {
    Visible(u32),
    Result(u32),
}

/// Only the real show/visibility/result protocol can create this one-shot token.
/// It deliberately has no public constructor, Clone, Copy or persisted representation.
#[derive(Debug, PartialEq, Eq)]
pub struct Completed {
    pickup: Pickup,
    token: u64,
}

impl Completed {
    pub fn pickup(&self) -> Pickup {
        self.pickup
    }
}

#[derive(Debug, Default)]
enum State {
    #[default]
    Idle,
    AwaitingShown(Pickup),
    Visible { pickup: Pickup, elapsed: f64 },
    AwaitingVisibility(Pickup),
    AwaitingResult(Pickup),
    AwaitingEffects { pickup: Pickup, token: u64 },
}

/// Transient script execution state; xTCO remains the existing persisted statistic.
#[derive(Debug, Default)]
pub struct Flow {
    state: State,
}

impl Flow {
    pub fn is_active(&self) -> bool {
        !matches!(self.state, State::Idle)
    }

    pub fn begin(&mut self, pickup: Pickup, xtco: f64, gates: Eligibility) -> Start {
        if self.is_active() {
            return Start::Busy;
        }
        if xtco != 0.0 || gates.no_tutorial || gates.networked || gates.connecting || gates.editor {
            return Start::Progress(pickup);
        }
        self.state = State::AwaitingShown(pickup);
        Start::ShowForm(FORM_ID)
    }

    /// A request alone is not evidence that the original form was actually shown.
    pub fn shown(&mut self, form: u32) -> bool {
        let State::AwaitingShown(pickup) = self.state else { return false };
        if form != FORM_ID {
            return false;
        }
        // Form_GetFormVisible is queried immediately; WAITUNWARPED(100) is
        // between rechecks, not before this first query.
        self.state = State::Visible { pickup, elapsed: POLL_SECONDS };
        true
    }

    /// The first poll is ready immediately. Subsequent polls wait 100 ms of unwarped time.
    /// A query must be answered before another can be issued; no catch-up burst.
    pub fn tick(&mut self, unwarped_dt: f32) -> Option<Query> {
        if !unwarped_dt.is_finite() || unwarped_dt < 0.0 {
            return None;
        }
        let State::Visible { pickup, elapsed } = &mut self.state else { return None };
        *elapsed += f64::from(unwarped_dt);
        if *elapsed < POLL_SECONDS {
            return None;
        }
        let pickup = *pickup;
        self.state = State::AwaitingVisibility(pickup);
        Some(Query::Visible(FORM_ID))
    }

    /// Unknown visibility is not a close. It rearms a later poll without completing.
    pub fn visibility(&mut self, form: u32, visible: Option<bool>) -> Option<Query> {
        let State::AwaitingVisibility(pickup) = self.state else { return None };
        if form != FORM_ID {
            return None;
        }
        if visible == Some(false) {
            self.state = State::AwaitingResult(pickup);
            Some(Query::Result(FORM_ID))
        } else {
            self.state = State::Visible { pickup, elapsed: 0.0 };
            None
        }
    }

    /// The script gets but does not branch on the form's result. No button is inferred.
    pub fn result(&mut self, form: u32, _result: i32) -> Option<Completed> {
        let State::AwaitingResult(pickup) = self.state else { return None };
        if form != FORM_ID {
            return None;
        }
        // This is a transient identity, not a saved statistic or tutorial flag.
        // A token from another Flow must not authorize effects even for the same pickup.
        static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);
        let token = NEXT_TOKEN.fetch_add(1, Ordering::Relaxed);
        self.state = State::AwaitingEffects { pickup, token };
        Some(Completed { pickup, token })
    }

    pub fn completion_pending(&self, completed: &Completed) -> bool {
        matches!(self.state, State::AwaitingEffects { pickup, token }
            if pickup == completed.pickup && token == completed.token)
    }

    /// The caller acknowledges only after xTCO, autosave and progress were handled.
    /// Keep the script busy through those effects, including a rejected stat write.
    pub fn finish_effects(&mut self, completed: Completed) -> bool {
        if !self.completion_pending(&completed) {
            return false;
        }
        self.state = State::Idle;
        true
    }
}

/// The existing Flash tutorial payload, using only selected original table values.
/// Prompts must come from a separately verified native-form policy. Empty means
/// no policy has been supplied, not that the original has no controls.
pub fn tutorial_page(strings: &StringTable, prompts: &[(String, i32)]) -> Option<MenuPage> {
    let (title, description) = crate::mc4::tutorial_keys(FORM_ID)?;
    Some(MenuPage {
        layout: MenuLayout::Tutorial { description: strings.get(&description)?.to_string() },
        title: strings.get(&title)?.to_string(),
        select: usize::MAX,
        prompts: prompts.to_vec(),
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "collectable_tutorial_tests.rs"]
mod tests;
