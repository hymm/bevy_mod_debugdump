use std::path::PathBuf;

use bevy::prelude::*;
use bevy_mod_debugdump_stageless::EdgeStyle;

fn main() -> Result<(), std::io::Error> {
    let compare_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("compare");
    std::fs::create_dir_all(&compare_path)?;

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.world
        .resource_scope::<Schedules, _>(|world, mut schedules| {
            let schedule = schedules.get_mut(&CoreSchedule::Main).unwrap();

            // for access info
            schedule.graph_mut().initialize(world);
            // for `conflicting_systems`
            schedule
                .graph_mut()
                .build_schedule(world.components())
                .unwrap();

            for edge_style in [
                EdgeStyle::None,
                EdgeStyle::Line,
                EdgeStyle::Polyline,
                EdgeStyle::Curved,
                EdgeStyle::Ortho,
                EdgeStyle::Spline,
            ] {
                let settings = bevy_mod_debugdump_stageless::Settings {
                    edge_style,
                    ..Default::default()
                };
                let dot =
                    bevy_mod_debugdump_stageless::schedule_to_dot(schedule, &world, &settings);

                std::fs::write(
                    compare_path.join(format!("schedule_{}.dot", edge_style.as_dot())),
                    dot,
                )?;
            }

            Ok(())
        })
}