use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use std::collections::HashMap;

use nom::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
}

fn get_input_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    BufReader::new(file).lines().collect()
}

named!(parse_point<&str, Point>,
    do_parse!(
        x:  map_res!(take_until_and_consume!(", "), str::parse::<usize>) >>
        y:  map_res!(nom::rest, str::parse::<usize>) >>
        (Point::new(x, y))
    )
);

fn get_input_points(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .map(|s| parse_point(s).unwrap())
        .map(|(_, p)| p)
        .collect()
}

fn abs_diff(from: usize, to: usize) -> usize {
    if from > to {
        from - to
    } else {
        to - from
    }
}

fn distance(from: &Point, to: &Point) -> usize {
    abs_diff(from.x, to.x) + abs_diff(from.y, to.y)
}

fn get_closest_point(from: &Point, points: &[Point]) -> Option<usize> {
    let mut smallest_distance = std::usize::MAX;
    let mut smallest_distance_count = 0;
    let mut closest_point = std::usize::MAX;

    for i in 0..points.len() {
        let other = &points[i];
        let dist = distance(from, other);

        if dist < smallest_distance {
            smallest_distance = dist;
            smallest_distance_count = 1;
            closest_point = i;
        } else if dist == smallest_distance {
            smallest_distance_count += 1;
        }
    }

    if smallest_distance_count == 1 {
        Some(closest_point)
    } else {
        None
    }
}

fn has_finite_area(owner: &Point, others: &[Point]) -> bool {
    return others.iter().any(|p| is_in_west_quadrant(owner, p))
        && others.iter().any(|p| is_in_north_quadrant(owner, p))
        && others.iter().any(|p| is_in_east_quadrant(owner, p))
        && others.iter().any(|p| is_in_south_quadrant(owner, p));
}

fn get_largest_finite_area(points: &[Point]) -> usize {
    let min_x = points.iter().map(|p| p.x).min().unwrap().clone();
    let min_y = points.iter().map(|p| p.y).min().unwrap().clone();
    let max_x = points.iter().map(|p| p.x).max().unwrap().clone();
    let max_y = points.iter().map(|p| p.y).max().unwrap().clone();
    let mut area_by_owner = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = Point::new(x, y);
            if let Some(owner) = get_closest_point(&p, points) {
                if has_finite_area(&points[owner], points) {
                    area_by_owner
                        .entry(owner)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    }

    let mut finite_areas: Vec<_> = area_by_owner.values().collect();
    let last = finite_areas.len() - 1;

    finite_areas.sort();

    finite_areas[last].clone()
}

fn part1(points: &[Point]) {
    let area = get_largest_finite_area(points);

    println!("Part1: {:?}", area);
}

fn get_farness(location: &Point, coordinates: &[Point]) -> usize {
    coordinates.iter().map(|c| distance(location, c)).sum()
}

fn get_safe_region_size(coordinates: &[Point], max_farness: usize) -> usize {
    let min_x = coordinates.iter().map(|p| p.x).min().unwrap().clone();
    let min_y = coordinates.iter().map(|p| p.y).min().unwrap().clone();
    let max_x = coordinates.iter().map(|p| p.x).max().unwrap().clone();
    let max_y = coordinates.iter().map(|p| p.y).max().unwrap().clone();
    let mut region_size = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let location = Point::new(x, y);
            let farness = get_farness(&location, coordinates);

            if farness < max_farness {
                region_size += 1;
            }
        }
    }

    region_size
}

fn part2(coordinates: &[Point]) {
    let region_size = get_safe_region_size(coordinates, 10000);

    println!("Part2: {:?}", region_size);
}

fn main() -> io::Result<()> {
    let os_args: Vec<_> = std::env::args().collect();
    let lines = get_input_lines(&os_args[1])?;
    let points = get_input_points(&lines);

    part1(&points);
    part2(&points);

    Ok(())
}

fn is_in_west_quadrant(origin: &Point, tested: &Point) -> bool {
    tested.x < origin.x && abs_diff(tested.y, origin.y) <= abs_diff(tested.x, origin.x)
}

fn is_in_east_quadrant(origin: &Point, tested: &Point) -> bool {
    tested.x > origin.x && abs_diff(tested.y, origin.y) <= abs_diff(tested.x, origin.x)
}

fn is_in_north_quadrant(origin: &Point, tested: &Point) -> bool {
    tested.y < origin.y && abs_diff(tested.x, origin.x) <= abs_diff(tested.y, origin.y)
}

fn is_in_south_quadrant(origin: &Point, tested: &Point) -> bool {
    tested.y > origin.y && abs_diff(tested.x, origin.x) <= abs_diff(tested.y, origin.y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_in_west_quadrant_on_x_axis() {
        let origin = Point::new(5, 5);
        let other = Point::new(3, 5);
        assert_eq!(true, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn find_in_west_quadrant_on_upper_diagonal() {
        let origin = Point::new(5, 5);
        let other = Point::new(3, 3);
        assert_eq!(true, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn find_in_west_quadrant_on_lower_diagonal() {
        let origin = Point::new(5, 5);
        let other = Point::new(3, 7);
        assert_eq!(true, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn find_in_west_quadrant_unspecified() {
        let origin = Point::new(5, 5);
        let other = Point::new(1, 6);
        assert_eq!(true, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn dont_find_in_west_quadrant_because_too_high() {
        let origin = Point::new(5, 5);
        let other = Point::new(1, 0);
        assert_eq!(false, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn dont_find_in_west_quadrant_because_too_low() {
        let origin = Point::new(5, 5);
        let other = Point::new(1, 10);
        assert_eq!(false, is_in_west_quadrant(&origin, &other));
    }

    #[test]
    fn dont_find_in_west_quadrant_because_east() {
        let origin = Point::new(5, 5);
        let other = Point::new(6, 5);
        assert_eq!(false, is_in_west_quadrant(&origin, &other));
    }

    fn get_test_points() -> Vec<Point> {
        vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ]
    }
    /*
    aaaaa.cccc
    aAaaa.cccc
    aaaddecccc
    aadddeccCc
    ..dDdeeccc
    bb.deEeecc
    bBb.eeee..
    bbb.eeefff
    bbb.eeffff
    bbb.ffffFf
    */
    #[test]
    fn get_closest_point_matches() {
        let points = get_test_points();

        let tested = Point::new(0, 0);
        assert_eq!(Some(0), get_closest_point(&tested, &points));

        let tested = Point::new(4, 0);
        assert_eq!(Some(0), get_closest_point(&tested, &points));

        let tested = Point::new(5, 0);
        assert_eq!(None, get_closest_point(&tested, &points));

        let tested = Point::new(6, 0);
        assert_eq!(Some(2), get_closest_point(&tested, &points));

        let tested = Point::new(2, 2);
        assert_eq!(Some(0), get_closest_point(&tested, &points));

        let tested = Point::new(3, 2);
        assert_eq!(Some(3), get_closest_point(&tested, &points));

        let tested = Point::new(5, 2);
        assert_eq!(Some(4), get_closest_point(&tested, &points));

        let tested = Point::new(6, 2);
        assert_eq!(Some(2), get_closest_point(&tested, &points));

        let tested = Point::new(0, 4);
        assert_eq!(None, get_closest_point(&tested, &points));

        let tested = Point::new(1, 4);
        assert_eq!(None, get_closest_point(&tested, &points));

        let tested = Point::new(6, 8);
        assert_eq!(Some(5), get_closest_point(&tested, &points));

        let tested = Point::new(3, 10);
        assert_eq!(None, get_closest_point(&tested, &points));

        let tested = Point::new(8, 10);
        assert_eq!(Some(5), get_closest_point(&tested, &points));
    }

    #[test]
    fn has_finite_area_matches() {
        let points = get_test_points();

        let tested = &points[0];
        assert_eq!(false, has_finite_area(tested, &points));

        let tested = &points[1];
        assert_eq!(false, has_finite_area(tested, &points));

        let tested = &points[2];
        assert_eq!(false, has_finite_area(tested, &points));

        let tested = &points[3];
        assert_eq!(true, has_finite_area(tested, &points));

        let tested = &points[4];
        assert_eq!(true, has_finite_area(tested, &points));

        let tested = &points[5];
        assert_eq!(false, has_finite_area(tested, &points));
    }

    #[test]
    fn get_largest_finite_area_matches() {
        let points = get_test_points();

        assert_eq!(17, get_largest_finite_area(&points));
    }

    #[test]
    fn get_farness_matches() {
        let coordinates = get_test_points();
        let location = Point::new(4, 3);

        assert_eq!(30, get_farness(&location, &coordinates));
    }

    #[test]
    fn get_safe_region_size_matches() {
        let coordinates = get_test_points();
        let region_size = get_safe_region_size(&coordinates, 32);

        assert_eq!(16, region_size);
    }
}
