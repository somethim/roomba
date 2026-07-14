use std::time::Duration;

use robot::{Hardware, Robot};
use shared::{
    command::Command,
    geometry::{Orientation, Point},
    map::{DockingStation, Map},
    measurement::Measurement,
    pose::Pose,
};

use crate::draw::Draw;

pub struct SimHost {
    pub(crate) map: Map,
    pub(crate) true_pose: Pose,
    pub(crate) ekf_pose: Pose,
    pub(crate) commands: (Command, Option<Command>),
    pub(crate) trail: Vec<Pose>,
}

impl SimHost {
    pub(crate) fn new() -> Self {
        let docking_station = DockingStation {
            point: Point { x: 0.5, y: 0.5 },
            orientation: Orientation { yaw: 0.0 },
        };

        Self {
            true_pose: Pose {
                position: docking_station.point,
                heading: docking_station.orientation,
            },
            ekf_pose: Pose {
                position: docking_station.point,
                heading: docking_station.orientation,
            },
            map: Map::default(),
            commands: (
                Command {
                    linear_velocity: 0.0,
                    angular_velocity: 0.0,
                },
                None,
            ),
            trail: Vec::new(),
        }
    }
}

impl Hardware for SimHost {
    const CONTROL_DT_MS: u32 = 100;
    const WHEEL_RADIUS_M: f64 = 0.036;
    const TRACK_WIDTH_M: f64 = 0.235;
    const ENCODER_TICKS_PER_REV: u32 = 508;

    fn sense(&self) -> Measurement {
        Measurement::new::<Self>(
            &self.commands.0,
            &self.commands.1,
            &self.true_pose,
            &self.map,
        )
    }

    fn act(&mut self, command: Command) {
        let dt = f64::from(Self::CONTROL_DT_MS) / 1000.0;
        let d = command.linear_velocity * dt;
        let theta = command.angular_velocity * dt;

        self.true_pose = self.true_pose.translate(d, theta);
        self.commands.1 = Some(self.commands.0);
        self.commands.0 = command;
    }
}

impl SimHost {
    pub fn run(mut self) {
        let mut robot = Robot::new(&self.map);

        let mut tick_count = 0;
        loop {
            let (ex, ey, eyaw, measurement, command) = self.step(&mut robot);

            if tick_count >= 100 {
                break;
            }

            println!(
                "[{tick_count:3}] true=({:.2},{:.2},{:.2}) ekf=({:.2},{:.2},{:.2}) range={:.2} command={:?}",
                self.true_pose.position.x,
                self.true_pose.position.y,
                self.true_pose.heading.yaw,
                ex,
                ey,
                eyaw,
                measurement.beacon.range,
                command,
            );

            tick_count += 1;

            self.draw();

            std::thread::sleep(Duration::from_millis(u64::from(Self::CONTROL_DT_MS)));
        }
    }

    fn step(&mut self, robot: &mut Robot) -> (f64, f64, f64, Measurement, Command) {
        let measurement = self.sense();
        let command = robot.plan::<Self>(&self.map, &measurement);
        self.act(command);

        let (ex, ey, eyaw) = robot.ekf().state();

        (ex, ey, eyaw, measurement, command)
    }

    fn draw(&self) {
        let draw = Draw::new(self);

        draw.draw_outer_shell();
        draw.draw_rooms();
        draw.draw_robot();
        draw.draw_trail();
        draw.draw_beacon();
    }
}
