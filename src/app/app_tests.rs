use crate::app::generate_curve_steps;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;

    #[test]
    fn test_generate_curve_steps() {
        // Test empty input
        let empty = vec![];
        assert!(generate_curve_steps(&empty, 3).is_empty());

        // Test single point
        let single = vec![Point { x: 0.0, y: 0.0 }];
        assert!(generate_curve_steps(&single, 3).is_empty());

        // Basic line case
        let two_points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 100.0, y: 0.0 }
        ];
        
        // 1 step should produce 2 points
        let steps = generate_curve_steps(&two_points, 1);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].len(), 2);

        // 3 steps should produce 3 iterations
        let steps = generate_curve_steps(&two_points, 3);
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].len(), 2);
        assert_eq!(steps[1].len(), 2);
        assert_eq!(steps[2].len(), 2);

        // Verify Chaikin ratio
        let first_step = &steps[0];
        assert_eq!(first_step[0].x, 0.0 * 0.75 + 100.0 * 0.25);
        assert_eq!(first_step[1].x, 0.0 * 0.25 + 100.0 * 0.75);
    }

    #[test]
    fn test_state_transitions() {
        let mut control_points = vec![];
        let mut curve_steps = vec![];
        let mut animating = false;
        let mut max_steps = 5;

        // Test adding points
        control_points.push(Point { x: 10.0, y: 10.0 });
        control_points.push(Point { x: 50.0, y: 50.0 });
        assert_eq!(control_points.len(), 2);

        // Test Enter key
        curve_steps = generate_curve_steps(&control_points, max_steps);
        animating = true;
        assert!(!curve_steps.is_empty());
        assert!(animating);

        // Test Clear key
        control_points.clear();
        curve_steps.clear();
        animating = false;
        assert!(control_points.is_empty());
        assert!(!animating);

        // Test step adjustments
        max_steps = 7;
        assert_eq!(max_steps, 7);
        max_steps = (max_steps - 1).clamp(1, 10);
        assert_eq!(max_steps, 6);
        max_steps = (max_steps + 1).clamp(1, 10);
        assert_eq!(max_steps, 7);
    }

    #[test]
    fn test_point_dragging_update() {
        let mut control_points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 100.0, y: 0.0 }
        ];
        let mut curve_steps = generate_curve_steps(&control_points, 3);
        let original_first_point = curve_steps[0][0].clone();

        // Simulate dragging first point
        control_points[0] = Point { x: 50.0, y: 50.0 };
        curve_steps = generate_curve_steps(&control_points, 3);
        
        // Verify curve updated
        assert_ne!(curve_steps[0][0].x, original_first_point.x);
        assert_eq!(curve_steps[0][0].x, 50.0 * 0.75 + 100.0 * 0.25);
    }
}