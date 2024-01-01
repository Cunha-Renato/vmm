pub trait ScalarMath<T>
{
    fn sum_scalar(&self, value: T) -> Self;
    fn sub_scalar(&self, value: T) -> Self;
    fn mul_scalar(&self, value: T) -> Self;
    fn div_scalar(&self, value: T) -> Self;
}