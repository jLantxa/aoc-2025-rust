//! This puzzle was a fucking nightmare to make it work efficiently.

use anyhow::Result;

pub(crate) fn part1(input: &str) -> Result<String> {
    // The first part is very straightforward...

    let tiles = parse_tiles(input)?;
    let areas = calculate_tiling_areas_filtered(&tiles, None as Option<fn(&Point, &Point) -> bool>);
    Ok(areas.iter().max().expect("Should have a max").to_string())
}

pub(crate) fn part2(input: &str) -> Result<String> {
    // ... but, as usual, there is a caveat. I don't even know how these
    // incompetent  elves are able to make anything useful for Christmas. There
    // is always a misundestanding, a problem, whatever...

    let tiles = parse_tiles(input)?;
    let areas = calculate_tiling_areas_filtered(
        &tiles,
        Some(|t0: &Point, t1: &Point| perimeter_contains(&tiles, t0, t1)),
    );
    Ok(areas.iter().max().expect("Should have a max").to_string())
}

type Value = i64;
type Point = (Value, Value);

/// Collect all the tiles from the input
fn parse_tiles(input: &str) -> Result<Vec<Point>> {
    Ok(input
        .trim()
        .lines()
        .map(|l| {
            let (xstr, ystr) = l.split_once(",").expect("Should have two coordinates");
            (
                xstr.parse::<Value>().expect("Should have x"),
                ystr.parse::<Value>().expect("Should have y"),
            )
        })
        .collect())
}

/// Rectangle area (inclusive coordinates).
#[inline(always)]
fn area(t0: &Point, t1: &Point) -> Value {
    ((t0.0 - t1.0).abs() + 1) * ((t0.1 - t1.1).abs() + 1)
}

/// Calculates areas for all pairs of tiles, optionally filtered by a predicate.
///
/// The predicate is a closure that takes references to the two points
/// and returns true if the area should be calculated.
fn calculate_tiling_areas_filtered<F>(points: &[Point], filter_fn: Option<F>) -> Vec<Value>
where
    F: Fn(&Point, &Point) -> bool + Clone,
{
    let n = points.len();

    (0..n)
        .flat_map(|i| {
            let filter_fn = filter_fn.clone();
            ((i + 1)..n).filter_map(move |j| {
                let t0 = &points[i];
                let t1 = &points[j];

                let should_calculate = match filter_fn.as_ref() {
                    Some(f) => f(t0, t1),
                    None => true,
                };

                if should_calculate {
                    Some(area(t0, t1))
                } else {
                    None
                }
            })
        })
        .collect()
}

/// Checks if the rectangle defined by corners t0 and t1 is strictly contained
/// within (or lies exactly on the boundary of) the rectilinear polygon.
fn perimeter_contains(perimeter: &[Point], t0: &Point, t1: &Point) -> bool {
    let min_x = std::cmp::min(t0.0, t1.0);
    let max_x = std::cmp::max(t0.0, t1.0);
    let min_y = std::cmp::min(t0.1, t1.1);
    let max_y = std::cmp::max(t0.1, t1.1);

    // Does any perimeter edge cut through the interior of the rectangle?
    // An edge on the boundary is allowed (red tiles). An edge strictly inside is forbidden.
    let n = perimeter.len();
    for i in 0..n {
        let p_start = perimeter[i];
        let p_end = perimeter[(i + 1) % n];

        if p_start.0 == p_end.0 {
            // Vertical Edge
            let edge_x = p_start.0;
            let edge_min_y = std::cmp::min(p_start.1, p_end.1);
            let edge_max_y = std::cmp::max(p_start.1, p_end.1);

            if edge_x > min_x && edge_x < max_x && edge_min_y < max_y && min_y < edge_max_y {
                return false;
            }
        } else if p_start.1 == p_end.1 {
            // Horizontal Edge
            let edge_y = p_start.1;
            let edge_min_x = std::cmp::min(p_start.0, p_end.0);
            let edge_max_x = std::cmp::max(p_start.0, p_end.0);

            if edge_y > min_y && edge_y < max_y && edge_min_x < max_x && min_x < edge_max_x {
                return false;
            }
        }
    }

    // Point-in-Polygon check (ray casting).
    // Check the exact geometric center point.
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;

    let mut inside = false;
    for i in 0..n {
        let p_curr = perimeter[i];
        let p_next = perimeter[(i + 1) % n];

        // We only intersect vertical edges (x coordinates match)
        if p_curr.0 == p_next.0 {
            let x = p_curr.0;
            let y1 = std::cmp::min(p_curr.1, p_next.1);
            let y2 = std::cmp::max(p_curr.1, p_next.1);

            if center_y >= y1 && center_y < y2 {
                // Is the edge to the right of the center?
                if x > center_x {
                    inside = !inside;
                }
            }
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 50.to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 24.to_string());
        Ok(())
    }
}
