mod draw;
mod sensors;
mod sim;

use std::time::Duration;

use rerun::RecordingStreamBuilder;
use robot::{Hardware, Robot};

use crate::{draw::draw_map, sim::SimHost};

fn main() {
    let rec = RecordingStreamBuilder::new("roomba")
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("Error: failed to spawn rerun recording stream: {e}");
            std::process::exit(1);
        });

    let mut sim = SimHost::new();
    let mut robot = Robot::new(&sim.map);

    draw_map(&sim, &rec);

    loop {
        sim.step(&mut robot);

        println!("{:?} {:?} {:?}", sim.true_pose, sim.ekf_pose, sim.trail);

        std::thread::sleep(Duration::from_millis(u64::from(SimHost::CONTROL_DT_MS)));
    }
}
