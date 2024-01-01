#[macro_export]
macro_rules! vec2 
{
    ($($e:expr),*) => (Vec2::from_array(&[$($e),*]));
}

#[macro_export]
macro_rules! vec3 
{
    ($($e:expr),*) => (Vec3::from_array(&[$($e),*]));
}

#[macro_export]
macro_rules! vec4 
{
    ($($e:expr),*) => (Vec4::from_array(&[$($e),*]));
}