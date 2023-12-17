use crate::traits::f32e;
use std::f32;
use std::ops::*;

#[allow(non_camel_case_types)]
pub trait Vec2 {
    fn new(x: f32, y: f32) -> V2;
    fn set(&mut self, x: f32, y: f32);
    fn to_str(&self) -> String;
    fn magnitude(&self) -> f32;
    fn normalize(&self) -> V2;
    fn distance(a: &mut V2, b: &mut V2) -> f32;
    fn dot(a: &mut V2, b: &mut V2) -> f32;
    fn cross(a: &mut V2, b: &mut V2) -> f32;
    fn angle(a: &mut V2, b: &mut V2) -> f32;
}

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

impl Vec2 for V2 {
    fn new(x: f32, y: f32) -> V2 {
        V2 { x, y }
    }

    fn to_str(&self) -> String {
        return format!("({},{})", self.x, self.y);
    }

    fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> V2 {
        let m = self.magnitude();
        V2::new(self.x / m, self.y / m)
    }

    fn distance(a: &mut V2, b: &mut V2) -> f32 {
        (*a - *b).magnitude()
    }

    fn dot(a: &mut V2, b: &mut V2) -> f32 {
        a.x * b.x + a.y * b.y
    }

    fn cross(a: &mut V2, b: &mut V2) -> f32 {
        a.x * b.y - b.y * a.x
    }

    fn angle(a: &mut V2, b: &mut V2) -> f32 {
        let (lal, lbl) = (a.magnitude(), b.magnitude());
        if f32::absf(lal + lbl) < f32::EPSILON {
            return 0.;
        }
        f32::acosf(V2::dot(a, b)) * f32::rad_to_deg()
    }
}

// pub mod trrne {
//     pub fn print() -> String {
//         return String::from("test");
//     }
// }
