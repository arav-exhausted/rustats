/// Returns the arithmetic mean of a sequence of `f64` values.
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::mean;
///
/// let values = [1.0, 2.0, 3.0];
/// let avg = mean(values);
/// assert_eq!(avg, Some(2.0));
/// ```
pub fn mean(values: &[f64]) -> Option<f64> {
    let mut count: u64 = 0;
    let mut sum: f64 = 0.0;

    for value in values {
        sum += value;
        count += 1;
    }

    if count == 0 {
        None
    } else {
        Some(sum / count as f64)
    }
}
