
#[repr(C)]
#[derive(Debug, Clone)]
pub struct vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> vec3<T> where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + 
    std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Self{x, y, z}
    }
}
impl<T> std::ops::Add for vec3<T> where T: std::ops::Add<Output = T> {
    type Output = vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        return Self{x: self.x + rhs.x, y: self.y + rhs.y, z:self.z + rhs.z};
    }
}
impl<T> std::ops::Sub for vec3<T> where T: std::ops::Sub<Output = T> {
    type Output = vec3<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self{x: self.x - rhs.x, y: self.y - rhs.y, z:self.z - rhs.z};
    }
}
// impl<T> std::ops::Mul for vec3<T> where T: std::ops::Mul<Output = T> + Copy {
//     type Output = vec3<T>;
//     fn mul(self, rhs: Self) -> Self::Output {
//         return Self{x: self.x * rhs.x, y: self.y * rhs.y, z:self.z * rhs.z};
//     }
// }
// impl<T> std::ops::Div for vec3<T> where T: std::ops::Div<Output = T> + Copy {
//     type Output = vec3<T>;
//     fn div(self, rhs: Self) -> Self::Output {
//         return Self{x: self.x / rhs.x, y: self.y / rhs.y, z:self.z / rhs.z};
//     }
// }

#[cfg(test)]
mod tests {
    use crate::vec3;
    #[test]
    fn t1() {
        let c = vec3::new(2, 2, 2);
        let q = vec3::new(1, 1, 1);
        let e = c + q;
    }
}