//! Opt-in observations of the first opponent's lifecycle. Never changes game decisions,
//! forces a spawn, or writes a save/data file. Enable with MCLA_DIAG_FIRST_RIVAL=1.

use std::{collections::HashMap, fmt, sync::{Mutex, OnceLock}, time::Instant};

pub const MISSION: &str = "RACE_GH_06";

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("MCLA_DIAG_FIRST_RIVAL").is_ok_and(|v| v == "1"))
}

pub fn record(stage: &str, detail: fmt::Arguments<'_>) {
    if !enabled() { return; }
    static START: OnceLock<Instant> = OnceLock::new();
    log::info!(target: "mcla::rival_diag", "first-rival t={}ms stage={} {}", START.get_or_init(Instant::now).elapsed().as_millis(), stage, detail);
}

#[derive(Default)]
struct States(HashMap<&'static str, String>);

impl States {
    fn changed(&mut self, stage: &'static str, state: String) -> bool {
        if self.0.get(stage) == Some(&state) { return false; }
        self.0.insert(stage, state);
        true
    }
}

/// Log changes of a gate, not every frame spent waiting at it. The state key must
/// exclude moving positions/timers; include those in detail for the transition only.
pub fn transition(stage: &'static str, state: String, detail: fmt::Arguments<'_>) {
    if !enabled() { return; }
    static STATES: OnceLock<Mutex<States>> = OnceLock::new();
    let changed = STATES.get_or_init(Mutex::default).lock().map(|mut s| s.changed(stage, state)).unwrap_or(false);
    if changed { record(stage, detail); }
}

pub fn snapshot(game: &crate::game::Game, stage: &str) {
    if !enabled() { return; }
    record(stage, format_args!(
        "mission={} state={:?} active={} in_log={} fresh_game={} mode={:?} table_lang={} saved_lang={:?} hookman={:?} rival={:?} car_dirs={} save={}",
        MISSION, game.career.mission_no(MISSION).map(|i| game.career.state_id_of(i)),
        game.career.mission_active(MISSION), game.career.mission_in_log(MISSION),
        game.fresh_game, game.mode, game.career.strings.lang,
        game.mc4.as_ref().map(|m| m.options.language), game.career.hookman_mission(),
        game.career.rival_id(), game.car_dirs.len(), crate::game::Game::save_path(&game.data_dir).display()
    ));
}

pub fn session(data_dir: &str) {
    if !enabled() { return; }
    record("session", format_args!(
        "version={} sha={} run={} target={}/{} debug_assertions={} exe={:?} cwd={:?} data={} canonical_data={:?} settings_lang={}",
        env!("CARGO_PKG_VERSION"), option_env!("GITHUB_SHA").unwrap_or("unrecorded"),
        option_env!("GITHUB_RUN_ID").unwrap_or("unrecorded"), std::env::consts::OS,
        std::env::consts::ARCH, cfg!(debug_assertions), std::env::current_exe().ok(),
        std::env::current_dir().ok(), data_dir, std::fs::canonicalize(data_dir).ok(),
        crate::hooks::language().word()
    ));
    // A fixed allowlist of gameplay switches, never the process environment.
    for key in ["MCLA_NEW_GAME", "MCLA_NO_CUTS", "MCLA_BOOT_LOAD", "MCLA_NO_OPP", "MCLA_HOOKMAN_TEST", "MCLA_ROAM_TEST", "MCLA_ROAM_OLD_AI"] {
        if let Some(value) = std::env::var_os(key) { record("switch", format_args!("{}={:?}", key, value)); }
    }
}

fn grid_uids() -> &'static Mutex<Vec<(usize, u32)>> {
    static GRID: OnceLock<Mutex<Vec<(usize, u32)>>> = OnceLock::new();
    GRID.get_or_init(Mutex::default)
}

/// Remember identities, not just mutable indices into the car vector. A later
/// removal/compaction can otherwise make a log of "racer 1" refer to another car.
pub fn grid_created(game: &crate::game::Game, vehicles: &crate::vehicles::Vehicles) {
    if !enabled() { return; }
    let Some(race) = game.race.as_ref() else { return; };
    if let Ok(mut grid) = grid_uids().lock() {
        *grid = race.participants.iter().filter_map(|&i| vehicles.cars.get(i).map(|c| (i, c.uid))).collect();
    }
    record("grid-created", format_args!("race={} participants={:?} cars={} models={}", race.name, race.participants, vehicles.cars.len(), vehicles.models.len()));
    sample_grid(game, vehicles, "grid-created", "", None);
}

/// Called every 60 rendered frames, after LOD selection, and at grid creation.
/// Observes both the original UID and the current participant index without
/// repairing, moving or reviving anything.
pub fn sample_grid(game: &crate::game::Game, vehicles: &crate::vehicles::Vehicles, phase: &str, scene: &str, world: Option<&crate::world::World>) {
    if !enabled() { return; }
    let Some(race) = game.race.as_ref() else { return; };
    let Ok(grid) = grid_uids().lock() else { return; };
    for &(slot, uid) in grid.iter() {
        let index_uid = vehicles.cars.get(slot).map(|c| c.uid);
        let actual = vehicles.cars.iter().enumerate().find(|(_, c)| c.uid == uid);
        match actual {
            None => record("grid-missing", format_args!("phase={} slot={} expected_uid={} index_uid={:?} participants={:?} scene={}", phase, slot, uid, index_uid, race.participants, scene)),
            Some((i, c)) => {
                let pose = c.render_pose(vehicles.render_alpha);
                record("grid-car", format_args!(
                    "phase={} slot={} expected_uid={} index_uid={:?} actual_index={} model={} driver={} config={} pos={:?} render_pos={:?} finite_pos={} speed={} placed={} pinned={} racer={} deactivated={} ai={} ai_reset={:?} engine_off={} stealth={} world_ready={:?} batches={} drawable_batches={:?} lod={:?} race_time={} countdown={} scene={}",
                    phase, slot, uid, index_uid, i, c.model.name, c.driver, c.vinyl_name,
                    c.state.pos, pose.pos, c.state.pos.is_finite() && pose.pos.is_finite(),
                    c.state.speed(), c.placed, c.pinned, c.racer, c.deactivated, c.ai.is_some(), c.ai.as_ref().and_then(|ai| ai.reset_to), c.engine_off, c.pu_stealth,
                    world.filter(|_| c.state.pos.is_finite()).map(|w| w.ready_at(c.state.pos)),
                    c.model.batches.len(), vehicles.lod_batches(i).map(|(b, _)| b.len()), vehicles.lod_pick.get(i),
                    race.time, race.countdown, scene
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_rival_trace_reports_reentry_after_a_gate_changes() {
        let mut states = States::default();
        assert!(states.changed("model", "reading".into()));
        assert!(!states.changed("model", "reading".into()));
        assert!(states.changed("model", "ready".into()));
        assert!(states.changed("placement", "no_ground".into()));
        assert!(states.changed("model", "reading".into()));
        assert!(!states.changed("placement", "no_ground".into()));
        assert!(states.changed("placement", "spawned".into()));
    }
}
