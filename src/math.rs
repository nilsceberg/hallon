pub trait VecN {
    fn length(&self) -> f32;
}

#[derive(Debug, Copy, Clone)]
pub struct Mat4x4(pub [[f32; 4]; 4]);

pub fn map((a0, b0): (f32, f32), (a1, b1): (f32, f32), x: f32) -> f32 {
    (x - a0) / (b0 - a0) * (b1 - a1) + a1
}

pub fn clamp((a, b): (f32, f32), x: f32) -> f32 {
    x.max(a).min(b)
}

impl Mat4x4 {
    pub fn mul(&self, vector: &Vec4) -> Vec4 {
        let &Mat4x4(m) = self;
        let v = vector;
        Vec4::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z + m[0][3] * v.w,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z + m[1][3] * v.w,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z + m[2][3] * v.w,
            m[3][0] * v.x + m[3][1] * v.y + m[3][2] * v.z + m[3][3] * v.w,
        )
    }

    pub fn mat_mul(&self, b: &Mat4x4) -> Mat4x4 {
        fn e(a: [f32; 4], b: [f32; 4]) -> f32 {
            a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
        }

        let &Mat4x4(a) = self;
        let Mat4x4(b) = b.transpose();

        Mat4x4([
            [e(a[0], b[0]), e(a[0], b[1]), e(a[0], b[2]), e(a[0], b[3])],
            [e(a[1], b[0]), e(a[1], b[1]), e(a[1], b[2]), e(a[1], b[3])],
            [e(a[2], b[0]), e(a[2], b[1]), e(a[2], b[2]), e(a[2], b[3])],
            [e(a[3], b[0]), e(a[3], b[1]), e(a[3], b[2]), e(a[3], b[3])],
        ])
    }

    pub fn transpose(&self) -> Mat4x4 {
        let &Mat4x4(a) = self;
        Mat4x4([
            [a[0][0], a[1][0], a[2][0], a[3][0]],
            [a[0][1], a[1][1], a[2][1], a[3][1]],
            [a[0][2], a[1][2], a[2][2], a[3][2]],
            [a[0][3], a[1][3], a[2][3], a[3][3]],
        ])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn mul(&self, f: f32) -> Vec2 {
        Vec2::new(self.x * f, self.y * f)
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn length2(&self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::from_vec3(*self, 1.0)
    }

    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn mul(&self, f: f32) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn translation(&self) -> Mat4x4 {
        Mat4x4([
            [1.0, 0.0, 0.0, self.x],
            [0.0, 1.0, 0.0, self.y],
            [0.0, 0.0, 1.0, self.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(&self) -> Mat4x4 {
        Mat4x4([
            [self.x, 0.0, 0.0, 0.0],
            [0.0, self.y, 0.0, 0.0],
            [0.0, 0.0, self.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation(&self) -> Mat4x4 {
        // X axis
        let x = Mat4x4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, self.x.cos(), -self.x.sin(), 0.0],
            [0.0, self.x.sin(), self.x.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        // Y axis
        let y = Mat4x4([
            [self.y.cos(), 0.0, -self.y.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [self.y.sin(), 0.0, self.y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        // Z axis
        let z = Mat4x4([
            [self.z.cos(), -self.z.sin(), 0.0, 0.0],
            [self.z.sin(), self.z.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        // Apply z first, then x, last y
        y.mat_mul(&x).mat_mul(&z)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn from_vec3(xyz: Vec3, w: f32) -> Vec4 {
        Vec4 {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
            w: w,
        }
    }

    pub fn mul(&self, f: f32) -> Vec4 {
        Vec4::new(self.x * f, self.y * f, self.z * f, self.w * f)
    }

    pub fn add(&self, other: &Vec4) -> Vec4 {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl VecN for Vec3 {
    fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl VecN for Vec4 {
    fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }
}
