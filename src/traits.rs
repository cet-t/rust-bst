#[allow(non_camel_case_types)]
pub trait f32e {
    fn pi() -> f32;
    fn rad_to_deg() -> f32;
    fn deg_to_rad() -> f32;
    fn sinf(deg: f32) -> f32;
    fn asinf(deg: f32) -> f32;
    fn cosf(deg: f32) -> f32;
    fn acosf(deg: f32) -> f32;
    fn tanf(deg: f32) -> f32;
    fn atanf(deg: f32) -> f32;
    fn absf(a: f32) -> f32;
    fn twinf(a: f32, b: f32) -> bool;
}

impl f32e for f32 {
    fn pi() -> f32 {
        3.14159265359
    }

    fn rad_to_deg() -> f32 {
        180. / f32::pi()
    }

    fn deg_to_rad() -> f32 {
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

    fn absf(a: f32) -> f32 {
        a.abs()
    }

    fn twinf(a: f32, b: f32) -> bool {
        let ofs: f32 = f32::absf(a - b);
        ofs < 1e-2
    }
}
