use cgmath::Vector3;

pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl From<Vector3<f32>> for Rgb {
    fn from(v: Vector3<f32>) -> Self {
        Rgb::new(v.x, v.y, v.z)
    }
}
