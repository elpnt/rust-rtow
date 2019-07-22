use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Get length
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    // Get squaret length
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Make unit vector
    pub fn make_unit_vector() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    // Get inner product
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Get cross product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, coef: f32) -> Self {
        Vec3 {
            x: self.x * coef,
            y: self.y * coef,
            z: self.z * coef,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, coef: f32) -> Self {
        Vec3 {
            x: self.x / coef,
            y: self.y / coef,
            z: self.z / coef,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v1, v2);
    }

    #[test]
    fn get_vec_length() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn get_veg_squared_length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.squared_length(), 14.0);
    }

    #[test]
    fn get_unit_vector() {
        let v1 = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let v2 = Vec3::make_unit_vector();
        assert_eq!(v1, v2);
    }

    #[test]
    fn get_inner_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn get_cross_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_expected = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(v1.cross(&v2), v_expected);
    }

    #[test]
    fn v_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_expected = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(v1 + v2, v_expected);
    }

    #[test]
    fn v_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v_expected = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(v1 - v2, v_expected);
    }

    #[test]
    fn v_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let a: f32 = 2.0;
        let v_expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v * a, v_expected);
    }

    #[test]
    fn v_div() {
        let v = Vec3::new(3.0, 6.0, 9.0);
        let a: f32 = 3.0;
        let v_expected = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v / a, v_expected);
    }

    #[test]
    fn v_neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v_expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-v1, v_expected);
    }
}
