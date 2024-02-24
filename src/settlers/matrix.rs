use std::fmt::{Display, Formatter, Result};
use std::ops::Sub;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn magnitude(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3::new(self.0 / mag, self.1 / mag, self.2 / mag)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Vec3::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Vec3::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(PartialEq, Debug)]
pub struct Mat4 {
    inner: [[f32; 4]; 4],
}

impl Display for Mat4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.inner {
            for &value in row {
                write!(f, "{:>10.2} ", value)?; // Adjust the formatting as needed
            }
            writeln!(f)?; // Move to the next line after each row
        }
        Ok(())
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(matrix: [[f32; 4]; 4]) -> Self {
        Self { inner: matrix }
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.inner[index.0][index.1]
    }
}

impl Mat4 {
    /// Returns an empty 4x4 Matrix
    pub fn new() -> Self {
        Mat4 {
            inner: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    /// Return an identity matrix
    pub fn identity() -> Self {
        Mat4 {
            inner: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Returns a projection matrix
    pub fn projection(aspect_ratio: f32, fov: f32, far: f32, near: f32) -> Self {
        Mat4 {
            inner: [
                [1.0 / (aspect_ratio * f32::tan(fov / 2.0)), 0.0, 0.0, 0.0],
                [0.0, 1.0 / f32::tan(fov / 2.0), 0.0, 0.0],
                [
                    0.0,
                    0.0,
                    -(far + near) / (far - near),
                    -2.0 * far * near / (far - near),
                ],
                [0.0, 0.0, -1.0, 1.],
            ],
        }
    }

    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
        // https://stackoverflow.com/questions/19740463/lookat-function-im-going-crazy
        let f = (center - eye).normalize();
        let u = up.normalize();
        let s = f.cross(&u).normalize();
        let u = s.cross(&f);

        let mut view = Self::new();
        view[0][0] = s.0;
        view[1][0] = s.1;
        view[2][0] = s.2;
        view[0][1] = u.0;
        view[1][1] = u.1;
        view[2][1] = u.2;
        view[0][2] = -f.0;
        view[1][2] = -f.1;
        view[2][2] = -f.2;
        view[3][0] = -s.dot(&eye);
        view[3][1] = -u.dot(&eye);
        view[3][2] = f.dot(&eye);
        view[3][3] = 1.;
        view
    }

    pub fn to_array(&self) -> [[f32; 4]; 4] {
        self.inner
    }

    /// Rotation around the z-axis by specified angle in radians.
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.inner[0][0] *= f32::cos(angle);
        self.inner[0][1] *= -f32::sin(angle);
        self.inner[1][0] *= f32::sin(angle);
        self.inner[1][1] *= f32::cos(angle);
        self
    }

    /// Moves the position to specified coordinates.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.inner[3][0] = x;
        self.inner[3][1] = y;
        self.inner[3][2] = z;
        self
    }

    /// Scale individal axis.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.inner[0][0] *= x;
        self.inner[1][1] *= y;
        self.inner[2][2] *= z;
        self
    }

    /// Scale all axis by `s`.
    pub fn scale_uniformly(&mut self, s: f32) -> &mut Self {
        self.scale(s, s, s)
    }

    pub fn multiply_by(&mut self, other: &Self) -> &mut Self {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.inner[i][k] * other.inner[k][j];
                }
            }
        }
        self.inner = result;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mat_mult_1() {
        let mut mat1 = Mat4::identity();
        let mut k: f32 = 0.;
        for i in 0..4 {
            for j in 0..4 {
                mat1[i][j] = k;
                k += 1.;
            }
        }
        println!("{}", mat1);

        let mut mat2 = Mat4::identity();
        let mut k: f32 = 4. * 4.;
        for i in 0..4 {
            for j in 0..4 {
                mat2[i][j] = k;
                k -= 1.;
            }
        }
        println!("{}", mat2);
        mat1.multiply_by(&mat2);

        assert_eq!(
            mat1,
            [
                [40., 34., 28., 22.],
                [200., 178., 156., 134.],
                [360., 322., 284., 246.],
                [520., 466., 412., 358.],
            ]
            .into()
        );

        println!("{}", mat1);
    }
}
