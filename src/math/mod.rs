pub mod vec2;
pub mod vec3;
pub type vec2f = vec2::vec2<f32>;
pub type vec2i = vec2::vec2<i32>;
pub type vec3f = vec3::vec3<f32>;
pub type vec3i = vec3::vec3<i32>;

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}