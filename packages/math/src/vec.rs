use std::ops::{Add, AddAssign};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    data: [f32; 4],
}

// Initializers
impl Vec3 {
    fn zero() -> Self {
        Self::default()
    }

    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [x, y, z, 1.0],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
                1.0,
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}
