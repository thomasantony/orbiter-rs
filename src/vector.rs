use crate::V;
#[derive(Debug, Default)]
#[repr(C)]
pub struct Vector3(pub [f64; 3]);

impl Vector3 {
    /// Build a new `Vector3` struct from given `x`, `y` and `z` coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }
    /// Returns the `x` coordinate of the vector
    #[inline(always)]
    pub fn x(&self) -> f64
    {
        self.0[0]
    }
    /// Returns the `y` coordinate of the vector
    #[inline(always)]
    pub fn y(&self) -> f64
    {
        self.0[1]
    }
    /// Returns the `z` coordinate of the vector
    #[inline(always)]
    pub fn z(&self) -> f64
    {
        self.0[2]
    }
    /// Computes the dot product with `other`
    pub fn dot(&self, other: &Vector3) -> f64
    {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    /// Computes cross product with `other`
    pub fn cross(&self, other: &Vector3) -> Vector3
    {
        let [a1, a2, a3] = self.0;
        let [b1, b2, b3] = other.0;
        Self::new(
            a2*b3 - a3*b2,
            a3*b1 - a1*b3,
            a1*b2 - a2*b1
        )
    }
    /// Computes the length of the vector
    pub fn length(&self) -> f64
    {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }
    /// Computes the unit vector in the direction of this vector
    pub fn unit(&self) -> Self
    {
        self / self.length()
    }
}

/// Implement negation operator for `Vector3`
impl std::ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output
    {
        V!(-self.x(), -self.y(), -self.z())
    }
}
/// Implement addition operator for `Vector3`
impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        V!(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}
/// Implement subtraction operator for `Vector3`
impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        V!(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}
/// Implement addition operator for `Vector3`
impl std::ops::Add<&Vector3> for Vector3 {
    type Output = Self;
    fn add(self, other: &Self) -> Self::Output {
        V!(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}
/// Implement subtraction operator for `Vector3`
impl std::ops::Sub<&Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, other: &Self) -> Self::Output {
        V!(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}
/// Implement subtraction operator for `Vector3`
impl std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Self::Output {
        V!(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}
/// Scalar multiplication for `Vector3`
impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output
    {
        V!(self.x()*rhs, self.y()*rhs, self.z()*rhs)
    }
}
/// Scalar multiplication for `&Vector3`
impl std::ops::Mul<f64> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output
    {
        V!(self.x()*rhs, self.y()*rhs, self.z()*rhs)
    }
}
/// Scalar division for `Vector3`
impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output
    {
        V!(self.x()/rhs, self.y()/rhs, self.z()/rhs)
    }
}
/// Scalar division for `&Vector3`
impl std::ops::Div<f64> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output
    {
        V!(self.x()/rhs, self.y()/rhs, self.z()/rhs)
    }
}
