use robot::{Hardware, Robot};
use shared::{command::Command, map::Map, measurement::Measurement, pose::Pose};

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
        let map = Map::default();

        Self {
            true_pose: Pose {
                position: map.docking_station.point,
                heading: map.docking_station.orientation,
            },
            ekf_pose: Pose {
                position: map.docking_station.point,
                heading: map.docking_station.orientation,
            },
            map,
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
    pub async fn run(mut self) {
        let mut robot = Robot::new(&self.map);
        self.step(&mut robot);
        self.draw().await;
    }

    fn step(&mut self, robot: &mut Robot) -> (f64, f64, f64, Measurement, Command) {
        let measurement = self.sense();
        let command = robot.plan::<Self>(&self.map, &measurement);
        self.act(command);

        let (ex, ey, eyaw) = robot.ekf().state();

        (ex, ey, eyaw, measurement, command)
    }

    async fn draw(&self) {
        let draw = Draw::new(self);

        draw.draw_outer_shell().await;
        draw.draw_rooms().await;
        draw.draw_robot().await;
        draw.draw_trail().await;
        draw.draw_beacon().await;
    }
}
