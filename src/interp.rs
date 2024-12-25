/// Enum to define the type of interpolation
#[derive(Debug)]
pub enum InterpolationType {
    Linear,           // Linear interpolation (order 1)
    Quadratic,        // Quadratic spline interpolation (order 2)
    Cubic,            // Cubic spline interpolation (order 3)
    ConstantBackward, // Constant interpolation taking the previous value
    ConstantForward,  // Constant interpolation taking the next value
}

/// Enum to define the extrapolation strategy
#[derive(Debug)]
pub enum ExtrapolationStrategy {
    None,         // Do not extrapolate, panic on out-of-bounds
    Constant,     // Use the closest y-value for out-of-bounds x
    ExtendSpline, // Use the same spline function as interpolation
}

#[derive(Debug)]
pub struct Interpolator {
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    b_coeffs: Vec<f64>,
    c_coeffs: Vec<f64>,
    d_coeffs: Vec<f64>,
    interpolation_type: InterpolationType,
    extrap_strategy: ExtrapolationStrategy,
}

impl Interpolator {
    /// Creates a new Interpolator with the given points
    pub fn new(
        x_values: Vec<f64>,
        y_values: Vec<f64>,
        interpolation_type: InterpolationType,
        extrap_strategy: ExtrapolationStrategy,
    ) -> Self {
        if x_values.len() != y_values.len() || x_values.len() < 2 {
            panic!(
                "x_values and y_values must have the same length and contain at least two points."
            );
        }

        // Precompute spline coefficients
        let (b_coeffs, c_coeffs, d_coeffs) =
            compute_spline_coefficients(&x_values, &y_values, &interpolation_type);
        Self {
            x_values,
            y_values,
            b_coeffs,
            c_coeffs,
            d_coeffs,
            interpolation_type,
            extrap_strategy,
        }
    }

    /// Performs interpolation for a given x value using the specified type
    pub fn interpolate(&self, x: f64) -> f64 {
        for j in 0..self.x_values.len() - 1 {
            if self.x_values[j] <= x && x <= self.x_values[j + 1] {
                // We found where the value is bracketed
                let dx = x - self.x_values[j];
                return match self.interpolation_type {
                    InterpolationType::Cubic
                    | InterpolationType::Quadratic
                    | InterpolationType::Linear => {
                        self.y_values[j]
                            + self.b_coeffs[j] * dx
                            + self.c_coeffs[j] * dx.powi(2)
                            + self.d_coeffs[j] * dx.powi(3)
                    }
                    InterpolationType::ConstantBackward => self.y_values[j],
                    InterpolationType::ConstantForward => self.y_values[j + 1],
                };
            }
        }
        if x < *self.x_values.first().unwrap() || x > *self.x_values.last().unwrap() {
            return self.extrapolate(x);
        }
        unreachable!("This could not be reached as the x is either bracketed or extrapolated");
    }

    /// Handles extrapolation for out-of-bounds x values
    fn extrapolate(&self, x: f64) -> f64 {
        match self.extrap_strategy {
            ExtrapolationStrategy::None => {
                panic!(
                    "Value x = {} is out of bounds and no extrapolation is enabled.",
                    x
                );
            }
            ExtrapolationStrategy::Constant => {
                if x < *self.x_values.first().unwrap() {
                    return *self.y_values.first().unwrap();
                }
                *self.y_values.last().unwrap()
            }
            ExtrapolationStrategy::ExtendSpline => {
                let j = if x < *self.x_values.first().unwrap() {
                    0
                } else {
                    self.x_values.len() - 2
                };
                let dx = x - self.x_values[j];
                self.y_values[j]
                    + self.b_coeffs[j] * dx
                    + self.c_coeffs[j] * dx.powi(2)
                    + self.d_coeffs[j] * dx.powi(3)
            }
        }
    }
}

/// Computes the coefficients for cubic spline interpolation
fn compute_spline_coefficients(
    x: &[f64],
    y: &[f64],
    interpolation_type: &InterpolationType,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    if matches!(
        interpolation_type,
        InterpolationType::ConstantForward | InterpolationType::ConstantBackward
    ) {
        return (vec![], vec![], vec![]);
    }

    let n = x.len() - 1; // Number of segments
    let dx: Vec<f64> = (0..n).map(|i| x[i + 1] - x[i]).collect(); // Spacing between x-values
    let dy: Vec<f64> = (0..n).map(|i| y[i + 1] - y[i]).collect(); // Spacing between y-values
    let slopes = (0..n).map(|i| dy[i] / dx[i]).collect();

    match interpolation_type {
        InterpolationType::Linear => (slopes, vec![0.0; n], vec![0.0; n]),
        InterpolationType::Quadratic => {
            let mut c = vec![0.0; n]; // Quadratic coefficients

            for i in 1..n - 1 {
                c[i] = (slopes[i] - slopes[i - 1]) / (dx[i - 1] * 2.0); // Derivative change over interval
            }
            c[n - 1] = 0.0; // Natural boundary at the last interval
            (slopes, c, vec![0.0; n])
        }
        InterpolationType::Cubic => {
            let mut alpha = vec![0.0; n - 1];
            for i in 1..n {
                alpha[i - 1] =
                    3.0 / dx[i] * (y[i + 1] - y[i]) - 3.0 / dx[i - 1] * (y[i] - y[i - 1]);
            }
            let mut b = vec![0.0; n];
            let mut c = vec![0.0; n + 1];
            let mut d = vec![0.0; n];
            let mut l = vec![1.0; n + 1];
            let mut mu = vec![0.0; n];
            let mut z = vec![0.0; n + 1];

            for i in 1..n {
                l[i] = 2.0 * (x[i + 1] - x[i - 1]) - dx[i - 1] * mu[i - 1];
                mu[i] = dx[i] / l[i];
                z[i] = (alpha[i - 1] - dx[i - 1] * z[i - 1]) / l[i];
            }
            for j in (0..n).rev() {
                c[j] = z[j] - mu[j] * c[j + 1];
                b[j] = dy[j] / dx[j] - dx[j] * (c[j + 1] + 2.0 * c[j]) / 3.0;
                d[j] = (c[j + 1] - c[j]) / (3.0 * dx[j]);
            }
            (b, c, d)
        }
        _ => panic!(
            "Interpolation type {:?} is not supported.",
            interpolation_type
        ),
    }
}
