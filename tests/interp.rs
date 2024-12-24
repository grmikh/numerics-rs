#[cfg(test)]
mod tests {
    use numerics_rs::interp::{InterpolationType, Interpolator, ExtrapolationStrategy};
    const EPSILON: f64 = 1e-4;
    #[test]
    fn test_linear_interpolation_within_range() {
        // Set up some test points
        let x_values = vec![0.0, 1.0, 2.0, 3.0];
        let y_values = vec![0.0, 2.0, 4.0, 6.0];

        // Create an interpolator
        let interpolator = Interpolator::new(x_values, y_values, InterpolationType::Linear, ExtrapolationStrategy::None);

        // Test linear interpolation at known points
        assert_eq!(interpolator.interpolate(0.0), 0.0);
        assert_eq!(interpolator.interpolate(1.0), 2.0);
        assert_eq!(interpolator.interpolate(2.0), 4.0);
        assert_eq!(interpolator.interpolate(3.0), 6.0);

        // Test linear interpolation between the points
        assert_eq!(interpolator.interpolate(0.5), 1.0);
        assert_eq!(interpolator.interpolate(1.5), 3.0);
        assert_eq!(interpolator.interpolate(2.5), 5.0);
    }

    #[test]
    #[should_panic(expected = "Value x = -1 is out of bounds and no extrapolation is enabled.")]
    fn test_linear_interpolation_out_of_bounds_no_extrapolation() {
        let x_values = vec![0.0, 1.0, 2.0];
        let y_values = vec![0.0, 1.0, 4.0];

        // Create an interpolator with no extrapolation
        let interpolator = Interpolator::new(x_values, y_values, InterpolationType::Linear, ExtrapolationStrategy::None);

        // Interpolation for out-of-bounds value (this should panic)
        interpolator.interpolate(-1.0);
    }

    #[test]
    fn test_linear_extrapolation_constant() {
        let x_values = vec![0.0, 2.0, 4.0];
        let y_values = vec![0.0, 4.0, 8.0];

        // Create an interpolator with constant extrapolation strategy
        let interpolator = Interpolator::new(x_values, y_values, InterpolationType::Linear, ExtrapolationStrategy::Constant);

        // Test extrapolation on the left side
        assert_eq!(interpolator.interpolate(-1.0), 0.0);

        // Test extrapolation on the right side
        assert_eq!(interpolator.interpolate(5.0), 8.0);
    }

    #[test]
    fn test_linear_extrapolation_extend_spline() {
        let x_values = vec![0.0, 1.0, 2.0, 3.0];
        let y_values = vec![0.0, 2.0, 4.0, 6.0];

        // Create an interpolator with ExtendSpline extrapolation strategy
        let interpolator = Interpolator::new(x_values, y_values, InterpolationType::Linear, ExtrapolationStrategy::ExtendSpline);
        // Test extrapolation on the left side (linear slope: 2.0 from x[0])
        assert_eq!(interpolator.interpolate(-1.0), -2.0);

        // Test extrapolation on the right side (linear slope: 2.0 from x[n-1])
        assert_eq!(interpolator.interpolate(4.0), 8.0);
    }

    #[test]
    fn test_quadratic_interpolation_basic() {
        let x_values = vec![0.0, 1.0, 2.0];
        let y_values = vec![0.0, 1.0, 4.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Quadratic,
            ExtrapolationStrategy::None,
        );

        // Test interpolation at known points
        assert_eq!(interpolator.interpolate(0.0), 0.0);
        assert_eq!(interpolator.interpolate(1.0), 1.0);
        assert_eq!(interpolator.interpolate(2.0), 4.0);

        // Test interpolation between known points
        let interpolated_value = interpolator.interpolate(1.5);
        assert!((interpolated_value - 2.5).abs() < EPSILON);
    }

    #[test]
    fn test_quadratic_interpolation_non_uniform() {
        let x_values = vec![0.0, 1.0, 3.0];
        let y_values = vec![1.0, 3.0, 19.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Quadratic,
            ExtrapolationStrategy::None,
        );

        // Test interpolation at known points
        assert_eq!(interpolator.interpolate(0.0), 1.0);
        assert_eq!(interpolator.interpolate(1.0), 3.0);
        assert_eq!(interpolator.interpolate(3.0), 19.0);

        // Test interpolation between known points
        let interpolated_value = interpolator.interpolate(2.0);
        assert!((interpolated_value - 11.0).abs() < EPSILON);
    }

    #[test]
    fn test_quadratic_interpolation_extrapolation_constant() {
        let x_values = vec![0.0, 1.0, 2.0];
        let y_values = vec![0.0, 1.0, 4.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Quadratic,
            ExtrapolationStrategy::Constant,
        );

        // Test extrapolation to the left
        assert_eq!(interpolator.interpolate(-1.0), 0.0);

        // Test extrapolation to the right
        assert_eq!(interpolator.interpolate(3.0), 4.0);
    }

    #[test]
    fn test_quadratic_interpolation_extrapolation_extend_spline() {
        let x_values = vec![0.0, 1.0, 2.0];
        let y_values = vec![0.0, 1.0, 4.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Quadratic,
            ExtrapolationStrategy::ExtendSpline,
        );

        // Test extrapolation to the left
        let left_extrapolated_value = interpolator.interpolate(-1.0);
        assert!((left_extrapolated_value - (-1.0)).abs() < EPSILON);

        // Test extrapolation to the right
        let right_extrapolated_value = interpolator.interpolate(3.0);
        assert!((right_extrapolated_value - 7.0).abs() < EPSILON);
    }

    #[test]
    fn test_cubic_interpolation_with_cubic_data() {
        // Test with cubic data y = x^3 - x^2 + 2x - 1
        let x_values: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y_values: Vec<f64> = x_values.iter().map(|&x| x.powi(3) - x.powi(2) + 2.0 * x - 1.0).collect();
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::None,
        );

        // Test interpolation within the range
        let result = interpolator.interpolate(2.5);
        let exp = 13.09821;
        assert!((result - exp).abs() < EPSILON, "Expected {}, got {}", exp, result);

        let result = interpolator.interpolate(1.5);
        let exp = 3.2232;
        assert!((result - exp).abs() < EPSILON, "Expected {}, got {}", exp, result);

        // Test with cubic data y = x^3
        let x_values: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y_values: Vec<f64> = x_values.iter().map(|&x| x.powi(3)).collect();
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::None,
        );

        let result = interpolator.interpolate(1.0);
        let exp = 1.0;
        assert!((result - exp).abs() < EPSILON, "Expected {}, got {}", exp, result);

        let result = interpolator.interpolate(3.0);
        let exp = 27.0;
        assert!((result - exp).abs() < EPSILON, "Expected {}, got {}", exp, result);

    }

    #[test]
    fn test_cubic_interpolation_irregular_data() {
        // Test with irregularly spaced data
        let x_values = vec![0.0, 1.0, 2.5, 4.0, 5.5];
        let y_values = vec![1.0, 2.5, 3.5, 1.0, 0.5];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::None,
        );

        // Test interpolation within the range
        let result = interpolator.interpolate(3.0);
        assert!(result > 1.0 && result < 3.5, "Unexpected result: {}", result);

        let result = interpolator.interpolate(1.5);
        assert!(result > 2.5 && result < 3.5, "Unexpected result: {}", result);
    }

    #[test]
    fn test_cubic_interpolation_extrapolation() {
        // Test extrapolation with ExtendSpline strategy
        let x_values = vec![0.0, 1.0, 2.0, 3.0];
        let y_values = vec![0.0, 1.0, 8.0, 27.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::ExtendSpline,
        );

        // Test extrapolation below the range
        let result = interpolator.interpolate(-1.0);
        assert!(result < 0.0, "Expected a value < 0.0, got {}", result);

        // Test extrapolation above the range
        let result = interpolator.interpolate(4.0);
        assert!(result > 27.0, "Expected a value > 27.0, got {}", result);
    }

    #[test]
    #[should_panic]
    fn test_cubic_interpolation_no_extrapolation() {
        // Test extrapolation with None strategy
        let x_values = vec![0.0, 1.0, 2.0, 3.0];
        let y_values = vec![0.0, 1.0, 8.0, 27.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::None,
        );

        // This should panic since extrapolation is disabled
        interpolator.interpolate(-1.0);
    }

    #[test]
    fn test_cubic_interpolation_edge_case() {
        // Test edge case with only two points
        let x_values = vec![1.0, 2.0];
        let y_values = vec![1.0, 8.0];
        let interpolator = Interpolator::new(
            x_values,
            y_values,
            InterpolationType::Cubic,
            ExtrapolationStrategy::ExtendSpline,
        );

        // Test interpolation within range
        let result = interpolator.interpolate(1.5);
        assert!((result - 4.5).abs() < EPSILON, "Expected 4.5, got {}", result);

        // Test extrapolation
        let result = interpolator.interpolate(0.5);
        assert!(result < 1.0, "Expected a value < 1.0, got {}", result);
    }
}