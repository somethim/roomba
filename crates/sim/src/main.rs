use std::time::Duration;

use robot::Robot;
use shared::hardware::Hardware;
use shared::{
    command::Command,
    geometry::{Orientation, Point},
    map::{DockingStation, Map},
    measurement::Measurement,
    pose::Pose,
};

struct SimHost {
    map: Map,
    pub true_pose: Pose,
    last_command: Command,
    prev_command: Option<Command>,
}

impl SimHost {
    fn new() -> Self {
        let docking_station = DockingStation {
            point: Point { x: 0.5, y: 0.5 },
            orientation: Orientation { yaw: 0.0 },
        };

        Self {
            true_pose: Pose {
                position: docking_station.point,
                heading: docking_station.orientation,
            },
            map: Map {
                outer_bounds: vec![
                    Point { x: 0.0, y: 0.0 },
                    Point { x: 5.0, y: 0.0 },
                    Point { x: 5.0, y: 3.0 },
                    Point { x: 0.0, y: 3.0 },
                ],
                inner_bounds: vec![vec![
                    Point { x: 3.0, y: 2.0 },
                    Point { x: 4.0, y: 2.0 },
                    Point { x: 4.0, y: 3.0 },
                    Point { x: 3.0, y: 3.0 },
                ]],
                docking_station,
            },
            last_command: Command {
                linear_velocity: 0.0,
                angular_velocity: 0.0,
            },
            prev_command: None,
        }
    }
}

impl Hardware for SimHost {
    const TICK_SPEED: u32 = 100;
    const WHEEL_RADIUS: f64 = 1.0;
    const WHEEL_DISTANCE: f64 = 2.0;
    const TICKS_PER_REV: i32 = 100;

    fn sense(&self) -> Measurement {
        Measurement::new::<Self>(
            &self.last_command,
            &self.prev_command,
            &self.true_pose,
            &self.map,
        )
    }

    fn act(&mut self, command: Command) {
        let dt = f64::from(Self::TICK_SPEED) / 1000.0;
        let d = command.linear_velocity * dt;
        let theta = command.angular_velocity * dt;

        self.true_pose = self.true_pose.translate(d, theta);
        self.prev_command = Some(self.last_command);
        self.last_command = command;
    }
}

fn main() {
    let mut sim = SimHost::new();
    let mut robot = Robot::new(&sim.map);

    let mut tick_count = 0;
    loop {
        let measurement = sim.sense();
        let command = robot.plan::<SimHost>(&sim.map, &measurement);
        sim.act(command);

        let (ex, ey, eyaw) = robot.ekf().state();
        println!(
            "[{tick_count:3}] true=({:.2},{:.2},{:.2}) ekf=({:.2},{:.2},{:.2}) range={:.2}",
            sim.true_pose.position.x,
            sim.true_pose.position.y,
            sim.true_pose.heading.yaw,
            ex,
            ey,
            eyaw,
            measurement.beacon.range,
        );

        tick_count += 1;

        if tick_count >= 100 {
            break;
        }

        std::thread::sleep(Duration::from_millis(u64::from(SimHost::TICK_SPEED)));
    }
}
