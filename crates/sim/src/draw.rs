#![allow(clippy::cast_possible_truncation, clippy::similar_names)]

use rerun::RecordingStream;

use crate::sim::SimHost;

pub fn draw_map(sim: &SimHost, rec: &RecordingStream) {
    let openings = all_openings(sim);
    let rooms: Vec<Vec<[f32; 2]>> = sim
        .map
        .rooms
        .iter()
        .flat_map(|room| boundary_strips(&room.boundary, &openings))
        .collect();

    if let Err(err) = rec.log_static("world/map/rooms", &rerun::LineStrips2D::new(rooms)) {
        eprintln!("Error: failed to log room boundaries: {err}");
    }
}

fn boundary_strips(
    boundary: &[shared::geometry::Point],
    openings: &[shared::map::Opening],
) -> Vec<Vec<[f32; 2]>> {
    let mut strips = Vec::with_capacity(boundary.len() + openings.len());
    let edges = boundary
        .iter()
        .copied()
        .zip(boundary.iter().copied().cycle().skip(1))
        .take(boundary.len());

    for (start, end) in edges {
        let mut openings_on_edge = openings
            .iter()
            .filter_map(|opening| edge_interval(start, end, opening))
            .collect::<Vec<_>>();
        openings_on_edge.sort_by(|a, b| a.0.total_cmp(&b.0));

        let mut cursor = 0.0_f32;
        for (gap_start, gap_end) in openings_on_edge {
            if gap_start > cursor {
                strips.push(vec![
                    point_at(start, end, cursor),
                    point_at(start, end, gap_start),
                ]);
            }
            cursor = cursor.max(gap_end);
        }

        if cursor < 1.0 {
            strips.push(vec![
                point_at(start, end, cursor),
                point_at(start, end, 1.0),
            ]);
        }
    }

    strips
}

fn all_openings(sim: &SimHost) -> Vec<shared::map::Opening> {
    sim.map
        .rooms
        .iter()
        .flat_map(|room| room.openings.iter().cloned())
        .collect()
}

fn edge_interval(
    start: shared::geometry::Point,
    end: shared::geometry::Point,
    opening: &shared::map::Opening,
) -> Option<(f32, f32)> {
    let edge_dx = (end.x - start.x) as f32;
    let edge_dy = (end.y - start.y) as f32;
    let edge_len_sq = edge_dx.mul_add(edge_dx, edge_dy * edge_dy);
    if edge_len_sq == 0.0 {
        return None;
    }

    let opening_start = project_parameter(start, end, opening.start)?;
    let opening_end = project_parameter(start, end, opening.end)?;
    let min = opening_start.min(opening_end).clamp(0.0, 1.0);
    let max = opening_start.max(opening_end).clamp(0.0, 1.0);

    if max <= 0.0 || min >= 1.0 {
        return None;
    }

    Some((min, max))
}

fn project_parameter(
    start: shared::geometry::Point,
    end: shared::geometry::Point,
    point: shared::geometry::Point,
) -> Option<f32> {
    let edge_dx = (end.x - start.x) as f32;
    let edge_dy = (end.y - start.y) as f32;

    let point_dx = (point.x - start.x) as f32;
    let point_dy = (point.y - start.y) as f32;

    let edge_len_sq = edge_dx.mul_add(edge_dx, edge_dy * edge_dy);
    if edge_len_sq == 0.0 {
        return None;
    }

    let cross = edge_dx.mul_add(point_dy, -(edge_dy * point_dx)).abs();
    if cross > 0.001 {
        return None;
    }

    Some(point_dy.mul_add(edge_dy, point_dx * edge_dx) / edge_len_sq)
}

fn point_at(start: shared::geometry::Point, end: shared::geometry::Point, t: f32) -> [f32; 2] {
    let x = ((end.x - start.x) as f32).mul_add(t, start.x as f32);
    let y = ((end.y - start.y) as f32).mul_add(t, start.y as f32);
    [x, y]
}
