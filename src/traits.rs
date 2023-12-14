#[allow(non_camel_case_types)]
pub trait f32e {
    fn pi() -> f32;
    fn to_deg() -> f32;
    fn to_rad() -> f32;
    fn sinf(deg: f32) -> f32;
    fn asinf(deg: f32) -> f32;
    fn cosf(deg: f32) -> f32;
    fn acosf(deg: f32) -> f32;
    fn tanf(deg: f32) -> f32;
    fn atanf(deg: f32) -> f32;
}

impl f32e for f32 {
    fn pi() -> f32 {
        3.14159265359
    }

    fn to_deg() -> f32 {
        180. / f32::pi()
    }

    fn to_rad() -> f32 {
        f32::pi() / 180.
    }

    fn sinf(deg: f32) -> f32 {
        deg.sin()
    }

    fn asinf(deg: f32) -> f32 {
        deg.asin()
    }

    fn cosf(deg: f32) -> f32 {
        deg.cos()
    }

    fn acosf(deg: f32) -> f32 {
        deg.acos()
    }

    fn tanf(deg: f32) -> f32 {
        deg.tan()
    }

    fn atanf(deg: f32) -> f32 {
        deg.atan()
    }
}
