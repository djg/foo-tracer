#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl ::std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl ::std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ::std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ::std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ::std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl ::std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ::std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl ::std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ::std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(mut self, rhs: Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl ::std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ::std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ::std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ::std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, mut rhs: Vec3) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl ::std::ops::Div for Vec3 {
    type Output = Vec3;
    fn div(mut self, rhs: Vec3) -> Self::Output {
        self /= rhs;
        self
    }
}

impl ::std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Vec3 {
    #[inline]
    pub fn squared_len(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn len(&self) -> f32 {
        f32::sqrt(self.squared_len())
    }

    pub fn normalize(&mut self) {
        let k = 1. / self.len();
        *self *= k;
    }
}

#[inline]
pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3(
        lhs.1 * rhs.2 - lhs.2 * rhs.1,
        lhs.2 * rhs.0 - lhs.0 * rhs.2,
        lhs.0 * rhs.1 - lhs.1 * rhs.0,
    )
}

pub fn normalized(mut v: Vec3) -> Vec3 {
    v.normalize();
    v
}
