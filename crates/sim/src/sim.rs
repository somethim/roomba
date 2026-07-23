use robot::{Hardware, Robot};
use shared::{command::Command, map::Map, pose::Pose, sensors::SensorFrame};

use crate::sensors;

struct CommandState {
    current_command: Command,
    previous_command: Option<Command>,
}

pub struct SimHost {
    pub(crate) map: Map,
    pub(crate) true_pose: Pose,
    pub(crate) ekf_pose: Pose,
    pub(crate) commands: CommandState,
    pub(crate) trail: Vec<Pose>,
    pub(crate) time_ms: u64,
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
            commands: CommandState {
                current_command: Command {
                    linear_velocity: 0.0,
                    angular_velocity: 0.0,
                },
                previous_command: None,
            },
            trail: Vec::new(),
            time_ms: 0,
        }
    }
}

impl Hardware for SimHost {
    const CONTROL_DT_MS: u32 = 100;
    const WHEEL_RADIUS_M: f64 = 0.036;
    const TRACK_WIDTH_M: f64 = 0.235;
    const ENCODER_TICKS_PER_REV: u32 = 508;

    fn sense(&self) -> SensorFrame {
        sensors::sensor_frame::<Self>(
            &self.commands.0,
            self.commands.1.as_ref(),
            &self.true_pose,
            &self.map,
            self.time_ms,
        )
    }

    fn act(&mut self, command: Command) {
        let dt = f64::from(Self::CONTROL_DT_MS) / 1000.0;
        let d = command.linear_velocity * dt;
        let theta = command.angular_velocity * dt;

        self.true_pose = self.true_pose.translate(d, theta);
        self.commands.1 = Some(self.commands.0);
        self.commands.0 = command;
        self.time_ms = self.time_ms.saturating_add(u64::from(Self::CONTROL_DT_MS));
    }
}

impl SimHost {
    pub fn step(&mut self, robot: &mut Robot) {
        let measurement = self.sense();
        let command = robot.plan::<Self>(&self.map, &measurement);
        self.act(command);
    }
}
