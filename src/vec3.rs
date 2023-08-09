use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn lengthsquared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.lengthsquared().sqrt()
    }
    pub fn write(&self) {
        println!("{} {} {}", self.e[0], self.e[1], self.e[2])
    }
    pub fn nearzero(&self) -> bool {
        let s = 0.000000001;
        self.x() < s && self.y() < s && self.z() < s
    }
}
pub fn unit(u: &Vec3) -> Vec3 {
    u / u.length()
}
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * dot(v, n) * n)
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        };
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        };
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        return Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        };
    }
}
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, v: &Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        };
    }
}
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, v: &Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        };
    }
}
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Vec3 {
        return Vec3 {
            e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        };
    }
}
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, v: f64) -> Vec3 {
        return Vec3 {
            e: [self.e[0] * v, self.e[1] * v, self.e[2] * v],
        };
    }
}
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Vec3 {
        return v * self;
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, v: f64) -> Vec3 {
        return (1. / v) * self;
    }
}
pub type Point3 = Vec3;
pub type Color = Vec3;
