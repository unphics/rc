
#[repr(C)]
#[derive(Debug, Clone)]
pub struct vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T> vec2<T> where T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + 
    std::ops::Mul<Output = T> + std::ops::Div<Output = T> + Copy {
    pub fn new(x: T, y: T) -> Self {
        return Self{x, y}
    }
}
impl<T> std::ops::Add for vec2<T> where T: std::ops::Add<Output = T> {
    type Output = vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        return Self{x: self.x + rhs.x, y: self.y + rhs.y};
    }
}
impl<T> std::ops::Sub for vec2<T> where T: std::ops::Sub<Output = T> {
    type Output = vec2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self{x: self.x - rhs.x, y: self.y - rhs.y};
    }
}
impl<T> std::ops::Mul for vec2<T> where T: std::ops::Mul<Output = T> + Copy {
    type Output = vec2<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self{x: self.x * rhs.x, y: self.y * rhs.y};
    }
}
impl<T> std::ops::Div for vec2<T> where T: std::ops::Div<Output = T> + Copy {
    type Output = vec2<T>;
    fn div(self, rhs: Self) -> Self::Output {
        return Self{x: self.x / rhs.x, y: self.y / rhs.y};
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2;
    #[test]
    fn t1() {
        let c = vec2::new(2, 2);
        let q = vec2::new(1, 1);
        let e = c + q;
    }
}