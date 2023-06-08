use std::ops;

pub struct Vector2D {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Clone, Copy)]
pub struct Vector3D {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl ops::Add<f32> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: f32) -> Self::Output {
        Vector2D{ x: self.x + rhs, y: self.y + rhs }
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Sub<f32> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector2D{ x: self.x - rhs, y: self.y - rhs }
    }
}

impl ops::Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D{ x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D{ x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Div<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: Vector2D) -> Self::Output {
        Vector2D{ x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl ops::Add<f32> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: f32) -> Self::Output {
        Vector3D{ x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        Vector3D{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl ops::BitAnd<Vector3D> for Vector3D {
    type Output = f32;

    fn bitand(self, rhs: Vector3D) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Vector2D{
    pub fn length(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl Vector3D{
    pub fn length(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
    pub fn norm(self) -> Vector3D {
        let length = self.length();
        self / Vector3D{x: length, y: length, z: length}
    }
}