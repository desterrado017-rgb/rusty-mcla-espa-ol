//! Opt-in observations of the opening film's hand-off to vehicle selection.
//! Never changes cameras, streaming, visibility, timings, resources or saves.
//! Enable with MCLA_DIAG_OPENING=1; output goes to the game's existing log.

use glam::Vec3;
use std::{fmt, sync::{Mutex, OnceLock}};

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("MCLA_DIAG_OPENING").is_ok_and(|v| v == "1"))
}

pub fn event(stage: &str, detail: fmt::Arguments<'_>) {
    if enabled() {
        log::info!(target: "mcla::opening_diag", "opening-diag event={} {}", stage, detail);
    }
}

#[derive(Default)]
struct Observations {
    state: Option<(u8, u16)>,
    render: Option<(u8, u64)>,
    focus: Option<(u8, u64)>,
}

impl Observations {
    fn state_changed(&mut self, stage: u8, flags: u16) -> bool {
        let next = Some((stage, flags));
        if self.state == next { return false; }
        self.state = next;
        true
    }

    fn sample(last: &mut Option<(u8, u64)>, stage: u8, frame: u64) -> bool {
        if stage == 0 {
            *last = None;
            return false;
        }
        if last.is_none_or(|(old_stage, old_frame)| old_stage != stage || frame.saturating_sub(old_frame) >= 60) {
            *last = Some((stage, frame));
            true
        } else {
            false
        }
    }
}

fn observations() -> &'static Mutex<Observations> {
    static STATES: OnceLock<Mutex<Observations>> = OnceLock::new();
    STATES.get_or_init(Mutex::default)
}

pub fn state(stage: u8, flags: u16, detail: fmt::Arguments<'_>) {
    if !enabled() { return; }
    let changed = observations().lock().map(|mut s| s.state_changed(stage, flags)).unwrap_or(false);
    if changed { event("state", detail); }
}

pub fn stream_focus(frame: u64, stage: u8, focus: Vec3, world: &crate::world::World) {
    if !enabled() { return; }
    let sample = observations().lock().map(|mut s| Observations::sample(&mut s.focus, stage, frame)).unwrap_or(false);
    if !sample { return; }
    event("stream-focus", format_args!("frame={} stage={} focus={:?} pending={} loaded={} ready_at_focus={}",
        frame, stage, focus, world.streaming(), world.loaded.len(), world.ready_at(focus)));
}

/// These values describe the actual view after all camera overrides, not just sel_cam.
pub struct View {
    pub eye: Vec3,
    pub direction: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub lod_near: f32,
    pub viewport: [u32; 2],
    pub vp_finite: bool,
    pub selection_candidate: Option<(Vec3, Vec3)>,
    pub cinematic: bool,
}

pub fn render(frame: u64, stage: u8, view: View, vehicles: &crate::vehicles::Vehicles,
    world: &crate::world::World, frustum: &crate::render::Frustum, occluders: &crate::occluder::Selected) {
    if !enabled() { return; }
    let sample = observations().lock().map(|mut s| Observations::sample(&mut s.render, stage, frame)).unwrap_or(false);
    if !sample { return; }
    let starters = vehicles.cars.iter().filter(|c| c.driver == "starter").count();
    event("render-view", format_args!(
        "frame={} stage={} eye={:?} direction={:?} fov_y_deg={} projection_near={} lod_near={} viewport={:?} vp_finite={} selection_candidate={:?} cinematic={} pending={} loaded={} ready_at_eye={} starters={} hide_player={}",
        frame, stage, view.eye, view.direction, view.fov_y.to_degrees(), view.near, view.lod_near,
        view.viewport, view.vp_finite, view.selection_candidate, view.cinematic, world.streaming(),
        world.loaded.len(), world.ready_at(view.eye), starters, vehicles.hide_player));
    for (i, car) in vehicles.cars.iter().enumerate().filter(|(_, c)| c.driver == "starter") {
        let pose = car.render_pose(vehicles.render_alpha);
        let centre = pose.pos + pose.rot * car.model.lod_centre;
        let radius = car.model.lod_radius;
        event("starter", format_args!(
            "frame={} stage={} uid={} model={} state_pos={:?} render_pos={:?} render_rot={:?} finite_pose={} pinned={} deactivated={} is_player={} ready_at_car={} lod={:?} model_batches={} drawable_batches={:?} has_model_lods={} sphere_centre={:?} sphere_radius={} frustum_sphere={} cpu_occluder_sphere={}",
            frame, stage, car.uid, car.model.name, car.state.pos, pose.pos, pose.rot,
            pose.pos.is_finite() && pose.rot.is_finite(), car.pinned, car.deactivated, car.is_player,
            world.ready_at(car.state.pos), vehicles.lod_pick.get(i), car.model.batches.len(),
            vehicles.lod_batches(i).map(|(b, _)| b.len()), !car.model.lods[0].0.is_empty(), centre,
            radius, frustum.intersects_sphere(centre, radius), occluders.hides_sphere(centre, radius)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_observations_only_repeat_on_state_changes_or_sixty_frames() {
        let mut observations = Observations::default();
        assert!(observations.state_changed(2, 1));
        assert!(!observations.state_changed(2, 1));
        assert!(observations.state_changed(2, 0));
        assert!(observations.state_changed(3, 0));
        assert!(Observations::sample(&mut observations.render, 2, 100));
        for frame in [100, 101, 159] {
            assert!(!Observations::sample(&mut observations.render, 2, frame));
        }
        assert!(Observations::sample(&mut observations.render, 2, 160));
        assert!(Observations::sample(&mut observations.render, 3, 161));
        assert!(!Observations::sample(&mut observations.render, 0, 162));
        assert!(Observations::sample(&mut observations.render, 1, 163));
        // Streaming observations have their own cadence; neither consumes a render sample.
        assert!(Observations::sample(&mut observations.focus, 1, 163));
        assert!(!Observations::sample(&mut observations.render, 1, 163));
    }
}
