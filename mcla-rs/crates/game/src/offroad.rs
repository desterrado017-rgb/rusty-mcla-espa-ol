//! The stretches of a race route that leave the lane network (alleys, car parks, plazas, the
//! shortcuts the race scripts list as a few widely spaced nodes), and the navigation mesh.
//!
//! The computer racers find their way over the mesh with the original's quickest-path query
//! (brain.rs, `NavMesh::quickest_path`). The route line itself is only read by the port's own
//! helpers (racers slid along it outside the streamed city, the test autopilot, the old driver
//! behind `MCLA_AI_OLD`, the plain fallback map): an off-road stretch is a straight hop between
//! its two nodes. (The port's own mesh and grid planners - invented costs, snap and margin -
//! were retired on 2026-10-22, MAP.md "Navigation mesh queries".)

use crate::physics::collision::WorldCollider;
use glam::Vec3;
use mcla_formats::rage::navmesh::NavMesh;

/// The city's navigation mesh (`city.xsm`), loaded once.
pub fn mesh() -> Option<&'static NavMesh> {
    // Use exactly the root already selected for the world, races and vehicles.
    // Do not cache failure before startup has resolved that root.
    let data = &crate::hooks::get()?.data;
    static MESH: std::sync::OnceLock<Option<NavMesh>> = std::sync::OnceLock::new();
    MESH.get_or_init(|| {
        let path = data.path.join("resources/city/sc/0x69d17d57.r26");
        let resource = match crate::assets::load_resource_file(&path) {
            Ok(resource) => resource,
            Err(e) => {
                log::warn!("off-road: no navigation mesh at {} (data source {:?}): {:#}", path.display(), data.source, e);
                return None;
            }
        };
        let m = NavMesh::parse(&resource);
        match m.as_ref() {
            Some(m) => log::info!("off-road: navigation mesh, {} polygons ({} for cars), {} (data source {:?})", m.polys.len(), m.polys.iter().filter(|p| p.drivable()).count(), path.display(), data.source),
            None => log::warn!("off-road: no navigation mesh: could not parse {} (data source {:?})", path.display(), data.source),
        }
        m
    })
    .as_ref()
}

/// The line through the given points of an off-road stretch: the points themselves, straight
/// from one to the next (always "through").
pub fn plan(_col: &WorldCollider, _solid: &[Vec3], pts: &[Vec3]) -> Option<(Vec<Vec3>, bool)> {
    (pts.len() >= 2).then(|| (pts.to_vec(), true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU32, Ordering};

    const TEST_NAME: &str = "offroad::tests::navigation_mesh_uses_runtime_root_without_raw_data_argument";
    const CASE: &str = "MCLA_NAV_ROOT_TEST_CASE";
    const ROOT: &str = "MCLA_NAV_ROOT_TEST_ROOT";
    const SETTINGS: &str = "MCLA_NAV_ROOT_TEST_SETTINGS";
    const MESH_FILE: &str = "resources/city/sc/0x69d17d57.r26";

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU32 = AtomicU32::new(0);
            let p = std::env::temp_dir().join(format!("mcla nav root \u{f1} {}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir_all(&p).unwrap();
            Self(p)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A complete MCLR resource containing one synthetic road triangle. The real
    /// resource loader and NavMesh parser consume it; no game data is copied.
    fn write_mesh(root: &Path) -> Vec<u8> {
        fn u16at(d: &mut [u8], o: usize, n: u16) { d[o..o + 2].copy_from_slice(&n.to_be_bytes()); }
        fn u32at(d: &mut [u8], o: usize, n: u32) { d[o..o + 4].copy_from_slice(&n.to_be_bytes()); }
        fn ptr(d: &mut [u8], o: usize, to: u32) { u32at(d, o, 0x5000_0000 | to); }
        fn float(d: &mut [u8], o: usize, n: f32) { u32at(d, o, n.to_bits()); }

        let mut data = vec![0u8; 0x200];
        ptr(&mut data, 0x08, 0x20); // material manager
        ptr(&mut data, 0x0c, 0x60); // level
        ptr(&mut data, 0x24, 0x30); // material pointer array
        u16at(&mut data, 0x28, 1);
        ptr(&mut data, 0x30, 0x40);
        u32at(&mut data, 0x48, mcla_formats::rage::navmesh::AI_ROAD);
        ptr(&mut data, 0x88, 0xc0); // chunk-data table
        ptr(&mut data, 0xc4, 0xd0);
        ptr(&mut data, 0xd0, 0x160);
        ptr(&mut data, 0x90, 0xe0); // chunk pointers
        u16at(&mut data, 0x94, 1);
        ptr(&mut data, 0xe0, 0x100);
        float(&mut data, 0x110, 1.0); // identity chunk basis
        float(&mut data, 0x124, 1.0);
        float(&mut data, 0x138, 1.0);
        ptr(&mut data, 0x170, 0x1a0); // polygon
        ptr(&mut data, 0x174, 0x1c0); // vertices
        u16at(&mut data, 0x184, 1);
        u16at(&mut data, 0x186, 3);
        float(&mut data, 0x1a8, 50.0); // triangle area
        ptr(&mut data, 0x1b4, 0x1e0); // polygon's vertex indices
        data[0x1ba] = 3;
        u16at(&mut data, 0x1c6, 200); // (10, 0, 0), quantized at 20 units/m
        u16at(&mut data, 0x1d0, 200); // (0, 0, 10)
        u16at(&mut data, 0x1e2, 1);
        u16at(&mut data, 0x1e4, 2);
        let mut file = b"MCLR".to_vec();
        for word in [0u32, 0, data.len() as u32] {
            file.extend_from_slice(&word.to_le_bytes());
        }
        file.extend_from_slice(&data);
        let path = root.join(MESH_FILE);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, &file).unwrap();
        file
    }

    fn child(case: &str) {
        let root = PathBuf::from(std::env::var_os(ROOT).expect("fixture root"));
        let settings = PathBuf::from(std::env::var_os(SETTINGS).expect("fixture settings"));
        assert!(!std::env::args().any(|a| a == "--data"), "the legacy raw-argv resolver must not see a data argument");
        assert!(std::env::var_os("MCLA_DATA_DIR").is_none(), "the legacy test variable must not mask the regression");
        assert!(crate::hooks::get().is_none());
        if case == "early" {
            assert!(mesh().is_none(), "an early caller has no runtime root yet");
        }
        let flag = (case == "flag").then(|| root.to_str().unwrap());
        let rt = crate::hooks::init(flag, Some(settings.to_str().unwrap()), false);
        let expected = match case {
            "flag" => mcla_config::paths::DataSource::Flag,
            "env" => mcla_config::paths::DataSource::Env,
            "settings" | "early" => mcla_config::paths::DataSource::Settings,
            _ => panic!("unknown fixture mode {case}"),
        };
        assert_eq!(rt.data.source, expected);
        assert_eq!(rt.data.path, root);
        let loaded = mesh().expect("navigation mesh must use the same resolved data root as the game");
        assert_eq!(loaded.polys.len(), 1);
        assert_eq!(loaded.materials, [mcla_formats::rage::navmesh::AI_ROAD]);
        assert!(loaded.polys[0].drivable());
        assert_eq!(loaded.polys[0].verts[..3], [Vec3::ZERO, Vec3::X * 10.0, Vec3::Z * 10.0]);
        assert!(std::ptr::eq(loaded, mesh().unwrap()), "later reads reuse the initialized mesh");
    }

    #[test]
    fn navigation_mesh_uses_runtime_root_without_raw_data_argument() {
        if let Ok(case) = std::env::var(CASE) {
            child(&case);
            return;
        }
        // Each child has independent hooks and mesh OnceLocks. No parent-process
        // environment or real settings, saves, or data directories are changed.
        let fixture = Fixture::new();
        let original = write_mesh(&fixture.0);
        let settings_path = fixture.0.join("settings.toml");
        let mut failed = Vec::new();
        for case in ["settings", "env", "flag", "early"] {
            let mut settings = mcla_config::Settings::default();
            if matches!(case, "settings" | "early") {
                settings.paths.data_dir = fixture.0.to_str().unwrap().to_string();
            }
            settings.save(&settings_path).unwrap();
            let mut cmd = Command::new(std::env::current_exe().unwrap());
            cmd.args(["--exact", TEST_NAME, "--nocapture"])
                .env(CASE, case).env(ROOT, &fixture.0).env(SETTINGS, &settings_path)
                .env_remove("MCLA_DATA_DIR").env_remove("MCLA_DATA").env_remove("MCLA_SETTINGS")
                .env_remove("MCLA_SAVE_FILE").env_remove("MCLA_SAVE_DIR")
                .env("RUSTY_MCLA_USER_DIR", fixture.0.join("isolated user"));
            if case == "env" { cmd.env("MCLA_DATA", &fixture.0); }
            let out = cmd.output().unwrap();
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !out.status.success() || !stdout.contains(TEST_NAME) || !stdout.contains("1 passed") {
                failed.push(format!("{case}: status {}\nstdout:\n{}\nstderr:\n{}", out.status, String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)));
            }
        }
        assert_eq!(std::fs::read(fixture.0.join(MESH_FILE)).unwrap(), original, "the input resource remains byte-identical");
        assert!(failed.is_empty(), "{}", failed.join("\n"));
    }
}
