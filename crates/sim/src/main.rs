use crate::host::Host;
use crate::map::{declare, draw_frame};
use macroquad::prelude::next_frame;
use robot::Robot;
use shared::io::VelocityCommand;

mod host;
mod map;

#[macroquad::main("Roomba Sim")]
async fn main() {
    let mut scene = declare();
    let mut host = Host::new(
        scene.docking_station.clone(),
        scene.dirt.clone(),
        scene.obstacles.clone(),
    );

    loop {
        host.drive(VelocityCommand {
            linear: 1.0,
            angular: 0.6,
        });

        // TODO: once `SimHost` exposes a `true_pose()` getter, pull ground truth
        //  back into the scene so the visualizer animates the LIME dot
        scene.true_pose.pose = host.true_pose();

        draw_frame(&scene);
        next_frame().await;
    }
}
