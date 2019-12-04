use std::cmp::{max, min};

pub type Coord = (i32, i32);

fn on_segment(p: &Coord, q: &Coord, r: &Coord) -> bool {
    // Given three colinear points p, q, r, the function checks if
    // point q lies on line segment "pr"

    (q.0 <= max(p.0, r.0) && q.0 >= min(p.0, r.0) && q.1 <= max(p.1, r.1) && q.1 >= min(p.1, r.1))
}

fn orientation(p: &Coord, q: &Coord, r: &Coord) -> i32 {
    // Find orientation of ordered triplet (p, q, r).
    // The function returns following values
    // 0 --> p, q and r are colinear
    // 1 --> Clockwise
    // 2 --> Counterclockwise

    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
    if val == 0 {
        return 0; // colinear
    } else if val > 0 {
        return 1; // clockwise
    } else {
        return 2; // counter-clockwise
    }
}

pub fn do_intersect(p1: &Coord, p2: &Coord, q1: &Coord, q2: &Coord) -> bool {
    // Main function to check whether the closed line segments p1 - q1 and p2 - q2 intersect
    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    // p1, q1 and p2 are colinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, q1, p2) {
        return true;
    }

    // p1, q1 and p2 are colinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, p2) {
        return true;
    }

    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(q1, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, p2, q2) {
        return true;
    }
    return false;
}

fn get_cross_product(one: &Coord, two: &Coord) -> i32 {
    one.0 * two.1 - one.1 * two.0
}

pub fn get_intersect_point(p1: &Coord, p2: &Coord, q1: &Coord, q2: &Coord) -> Option<Coord> {
    let xdiff = (p1.0 - p2.0, q1.0 - q2.0);
    let ydiff = (p1.1 - p2.1, q1.1 - q2.1);

    let cross = get_cross_product(&xdiff, &ydiff);
    if cross == 0 {
        return None;
    }

    let d = (get_cross_product(p1, p2), get_cross_product(q1, q2));
    let x = get_cross_product(&d, &xdiff) / cross;
    let y = get_cross_product(&d, &ydiff) / cross;
    return Some((x, y));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_intersect() {
        let intersects = do_intersect(&(0, 0), &(0, 5), &(2, 2), &(-2, 2));
        assert_eq!(intersects, true);

        let point = get_intersect_point(&(0, 0), &(0, 5), &(2, 2), &(-2, 2));
        assert_eq!(point, Some((0, 2)));
    }
}
