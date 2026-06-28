use shared::map::Map;
use shared::robot::State;
use shared::state::Position;

#[must_use]
pub fn get_current_position_on_map(state: State, map: Map) -> Position {
    state.get_relative_position(&map.origin)
}
