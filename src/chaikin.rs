use crate::point::Point;

pub fn chaikin(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::with_capacity((points.len() - 1) * 2);
    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];

        let q = Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        };
        let r = Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        };

        new_points.push(q);
        new_points.push(r);
    }
    new_points
}
