/// Converts an angle from degrees to radians.
///
/// # Arguments
///
/// * `value` - The angle in degrees.
///
/// # Returns
///
/// The equivalent angle in radians.
///
/// # Example
///
/// ```
/// # use vmm::*;
///
/// let degrees = 90.0;
/// let radians = to_radians(degrees);
///
/// assert_eq!(radians, 1.5707963267948966);
/// ```
pub fn to_radians(value: f64) -> f64
{
    value * 0.01745329251994329576923690768489
}
/// Converts an angle from radians to degrees.
///
/// # Arguments
///
/// * `value` - The angle in radians.
///
/// # Returns
///
/// The equivalent angle in degrees.
///
/// # Example
///
/// ```
/// # use vmm::*;
/// 
/// let radians = 1.5707963267948966;
/// let degrees = to_degrees(radians);
///
/// assert_eq!(degrees, 90.0);
/// ```
pub fn to_degrees(value: f64) -> f64
{
    value * 57.295779513082320876798154814105
}