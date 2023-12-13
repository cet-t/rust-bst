use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::{f32, i32};

#[derive(Debug, Clone, Copy)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl Add<V2> for V2 {
    type Output = V2;
    fn add(self, rhs: V2) -> V2 {
        V2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<f32> for V2 {
    type Output = V2;
    fn add(self, rhs: f32) -> V2 {
        V2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<V2> for V2 {
    fn add_assign(&mut self, rhs: V2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f32> for V2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<V2> for V2 {
    type Output = V2;
    fn sub(self, rhs: V2) -> V2 {
        // return V2::new(self.x - rhs.x, self.y - rhs.y);
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        return V2::new(x, y);
    }
}

impl Sub<f32> for V2 {
    type Output = V2;
    fn sub(self, rhs: f32) -> V2 {
        V2::new(self.x - rhs, self.y - rhs)
    }
}

impl SubAssign<V2> for V2 {
    fn sub_assign(&mut self, rhs: V2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for V2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<V2> for V2 {
    type Output = V2;
    fn mul(self, rhs: V2) -> V2 {
        V2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for V2 {
    type Output = V2;
    fn mul(self, rhs: f32) -> V2 {
        V2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<V2> for V2 {
    fn mul_assign(&mut self, rhs: V2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<f32> for V2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<V2> for V2 {
    type Output = V2;
    fn div(self, rhs: V2) -> V2 {
        V2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Div<f32> for V2 {
    type Output = V2;
    fn div(self, rhs: f32) -> V2 {
        V2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<V2> for V2 {
    fn div_assign(&mut self, rhs: V2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for V2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl V2 {
    pub fn new(x: f32, y: f32) -> V2 {
        V2 { x, y }
    }

    pub fn to_str(&self) -> String {
        return format!("({},{})", self.x, self.y);
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(a: &mut V2, b: &mut V2) -> f32 {
        // (a-b).magnitude()
        a.magnitude() - b.magnitude()
    }

    pub fn dot(a: &mut V2, b: &mut V2) -> f32 {
        a.x * b.x + a.y * b.y
    }

    // pub fn cross(a: V2, b: V2) -> f32 {
    //     a.x * b.y - b.y * a.x
    // }

    pub fn angle(a: &mut V2, b: &mut V2) -> f32 {
        let da = a.magnitude();
        if da.abs() < f32::EPSILON {
            return 0.;
        }
        let db = b.magnitude();
        if db.abs() < f32::EPSILON {
            return 0.;
        }
        f32::to_degrees((V2::dot(a, b) / da / db).acos())
    }
}
