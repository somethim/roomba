use robot::get_current_position_on_map;
use shared::map::Map;
use shared::robot::{Mode, Pose, State};
use shared::state::{Orientation, Position};

fn main() {
    let world = Map::default();
    let state = State::new(
        Pose::new(
            Position::new(12.3, 123.5, 123.54311),
            Orientation::new(0.0, 0.0, 13.6),
        ),
        Mode::Sweeping,
        11.14,
    );

    println!(
        "You are in: {:?}",
        get_current_position_on_map(state, world)
    );
}
