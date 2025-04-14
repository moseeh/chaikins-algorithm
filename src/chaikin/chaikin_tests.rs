#[cfg(test)]
mod tests {
    use super::*;
    use crate::chaikin::chaikin;
    
    #[test]
    fn test_empty_input() {
        let points = vec![];
        let result = chaikin(&points);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_point() {
        let point = Point { x: 5.0, y: 5.0 };
        let points = vec![point];
        let result = chaikin(&points);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].x, 5.0);
        assert_eq!(result[0].y, 5.0);
    }

    #[test]
    fn test_two_points_one_iteration() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 100.0, y: 0.0 },
        ];
        let result = chaikin(&points);
        
        assert_eq!(result.len(), 2);
        // First new point (Q)
        assert_eq!(result[0].x, 25.0);
        assert_eq!(result[0].y, 0.0);
        // Second new point (R)
        assert_eq!(result[1].x, 75.0);
        assert_eq!(result[1].y, 0.0);
    }

    #[test]
    fn test_two_points_two_iterations() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 100.0, y: 0.0 },
        ];
        let first_iter = chaikin(&points);
        let second_iter = chaikin(&first_iter);
        
        assert_eq!(second_iter.len(), 2);
        assert_eq!(second_iter[0].x, 37.5);
        assert_eq!(second_iter[0].y, 0.0);
        assert_eq!(second_iter[1].x, 62.5);
        assert_eq!(second_iter[1].y, 0.0);
    }

    #[test]
    fn test_three_points_one_iteration() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 50.0, y: 50.0 },
            Point { x: 100.0, y: 0.0 },
        ];
        let result = chaikin(&points);
        
        assert_eq!(result.len(), 4);
        // First segment results
        assert_eq!(result[0].x, 12.5);
        assert_eq!(result[0].y, 12.5);
        assert_eq!(result[1].x, 37.5);
        assert_eq!(result[1].y, 37.5);
        // Second segment results
        assert_eq!(result[2].x, 62.5);
        assert_eq!(result[2].y, 37.5);
        assert_eq!(result[3].x, 87.5);
        assert_eq!(result[3].y, 12.5);
    }

    #[test]
    fn test_floating_point_coordinates() {
        let points = vec![
            Point { x: 1.5, y: 2.5 },
            Point { x: 3.5, y: 4.5 },
        ];
        let result = chaikin(&points);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].x, 0.75 * 1.5 + 0.25 * 3.5);
        assert_eq!(result[0].y, 0.75 * 2.5 + 0.25 * 4.5);
        assert_eq!(result[1].x, 0.25 * 1.5 + 0.75 * 3.5);
        assert_eq!(result[1].y, 0.25 * 2.5 + 0.75 * 4.5);
    }
}