#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Default + Copy, const N: usize> Vec<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn dot(&self, other: &Vec<T, N>) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
    {
        let mut sum = T::default();
        for i in 0..N {
            sum = sum + self.data[i] * other.data[i];
        }
        sum
    }

    pub fn length_squared(&self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
    {
        self.dot(self)
    }

    pub fn length(&self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Into<f32>,
        f32: Into<T>,
    {
        (self.length_squared().into().sqrt()).into()
    }
}

impl Vec<f32, 3> {
    pub fn cross(&self, other: &Vec<f32, 3>) -> Vec<f32, 3> {
        let [x1, y1, z1] = self.data;
        let [x2, y2, z2] = other.data;
        Vec::new([y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut transposed = [[T::default(); ROWS]; COLS];
        for (i, row) in self.data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                transposed[j][i] = val;
            }
        }
        Matrix::new(transposed)
    }

    pub fn mul_vec(&self, vec: &Vec<T, COLS>) -> Vec<T, ROWS>
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Default + Copy,
    {
        let mut result = [T::default(); ROWS];
        for (i, row) in self.data.iter().enumerate() {
            result[i] = row
                .iter()
                .zip(&vec.data)
                .fold(T::default(), |acc, (&a, &b)| acc + a * b);
        }
        Vec::new(result)
    }

    pub fn mul_matrix<const OTHER_COLS: usize>(
        &self,
        other: &Matrix<T, COLS, OTHER_COLS>,
    ) -> Matrix<T, ROWS, OTHER_COLS>
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Default + Copy,
    {
        let mut result = [[T::default(); OTHER_COLS]; ROWS];
        for (i, row) in self.data.iter().enumerate() {
            for j in 0..OTHER_COLS {
                result[i][j] = row
                    .iter()
                    .zip(other.data.iter().map(|col| col[j]))
                    .fold(T::default(), |acc, (&a, b)| acc + a * b);
            }
        }
        Matrix::new(result)
    }
}

// Type Aliases
pub type Vec3 = Vec<f32, 3>;
pub type Vec4 = Vec<f32, 4>;
pub type Matrix4 = Matrix<f32, 4, 4>;

impl<
        T: Copy + Default + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Into<f32>,
        const N: usize,
    > Vec<T, N>
{
    pub fn normalize(&self) -> Self
    where
        f32: Into<T>,
    {
        let length: f32 = self.length().into();
        if length != 0.0 {
            let inv_length: T = (1.0 / length).into();
            Vec::new(self.data.map(|x| (x.into() * inv_length.into()).into()))
        } else {
            *self
        }
    }
}

// Operator overloading for Vec3 subtraction
impl std::ops::Sub for Vec<f32, 3> {
    type Output = Vec<f32, 3>;

    fn sub(self, other: Self) -> Self::Output {
        Vec::new([
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ])
    }
}

impl Matrix4 {}

impl<T: Copy, const N: usize> Vec<T, N> {
    pub fn x(&self) -> T
    where
        T: Default,
    {
        self.data.first().copied().unwrap_or_default()
    }

    pub fn y(&self) -> T
    where
        T: Default,
    {
        self.data.get(1).copied().unwrap_or_default()
    }

    pub fn z(&self) -> T
    where
        T: Default,
    {
        self.data.get(2).copied().unwrap_or_default()
    }

    pub fn w(&self) -> T
    where
        T: Default,
    {
        self.data.get(3).copied().unwrap_or_default()
    }

    pub fn xy(&self) -> Vec<T, 2>
    where
        T: Default,
    {
        Vec::new([self.x(), self.y()])
    }

    pub fn xyz(&self) -> Vec<T, 3>
    where
        T: Default,
    {
        Vec::new([self.x(), self.y(), self.z()])
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn perspective_lh_zo(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fovy = (fovy / 2.0).tan();
        let h = 1.0 / tan_half_fovy;
        let w = h / aspect;
        let a = far / (far - near);
        let b = -near * far / (far - near);
        Matrix4::new([
            [w, 0.0, 0.0, 0.0],
            [0.0, h, 0.0, 0.0],
            [0.0, 0.0, a, 0.0],
            [0.0, 0.0, b, 1.0],
        ])
    }

    pub fn rotate(angle: f32, axis: &Vec3) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        let (x, y, z) = (axis.x(), axis.y(), axis.z());
        let (tx, ty, tz) = (t * x, t * y, t * z);

        Matrix4::new([
            [tx * x + c, tx * y - s * z, tx * z + s * y, 0.0],
            [tx * y + s * z, ty * y + c, ty * z - s * x, 0.0],
            [tx * z - s * y, ty * z + s * x, tz * z + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

/// Converts a slice of values of type `T` into a byte slice.
///
/// # Safety
///
/// This function is marked `unsafe` because it involves raw pointer manipulation and
/// assumes that the type `T` meets certain requirements. However, it is safe to use under
/// the following conditions:
///
/// 1. The type `T` must implement the `Copy` trait, ensuring that it is a simple,
///    fixed-size type with well-defined memory layout.
/// 2. The `values` slice must be valid and properly aligned for the type `T`.
///
/// Given these conditions, the function safely creates a byte slice representation of
/// the `values` slice without violating memory safety. It relies on the fact that raw
/// pointer casting is only done for valid data types and within the bounds of the slice.
pub fn as_byte_slice<T: Copy>(values: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(values.as_ptr() as *const u8, std::mem::size_of_val(values))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_4;

    use super::*;

    #[test]
    fn vec3_dot_product() {
        let v1 = Vec3::new([1.0, 2.0, 3.0]);
        let v2 = Vec3::new([4.0, 5.0, 6.0]);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn vec3_cross_product() {
        let v1 = Vec3::new([1.0, 2.0, 3.0]);
        let v2 = Vec3::new([4.0, 5.0, 6.0]);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vec3::new([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn vec3_length() {
        let v = Vec3::new([3.0, 4.0, 0.0]);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn matrix4_transpose() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let transposed = m.transpose();
        assert_eq!(
            transposed,
            Matrix4::new([
                [1.0, 5.0, 9.0, 13.0],
                [2.0, 6.0, 10.0, 14.0],
                [3.0, 7.0, 11.0, 15.0],
                [4.0, 8.0, 12.0, 16.0],
            ])
        );
    }

    #[test]
    fn matrix4_mul_vec4() {
        let m = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vec4::new([1.0, 2.0, 3.0, 4.0]);
        let result = m.mul_vec(&v);
        assert_eq!(result, v);
    }

    #[test]
    fn matrix4_mul_matrix4() {
        let m1 = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let m2 = Matrix4::new([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 2.0],
        ]);
        let result = m1.mul_matrix(&m2);
        assert_eq!(
            result,
            Matrix4::new([
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ])
        );
    }

    #[test]
    fn identity_matrix4_test() {
        let identity = Matrix4::identity();
        assert_eq!(
            identity,
            Matrix4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_vec_swizzles() {
        let v = Vec4::new([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);
        assert_eq!(v.xy(), Vec::new([1.0, 2.0]));
        assert_eq!(v.xyz(), Vec::new([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_perspective_lh_zo() {
        let fovy = FRAC_PI_4;
        let aspect = 16.0 / 9.0;
        let near = 0.1;
        let far = 100.0;
        let perspective = Matrix4::perspective_lh_zo(fovy, aspect, near, far);

        // Calculate expected values
        let expected_w = 1.0 / ((fovy / 2.0).tan() * aspect);
        let expected_h = 1.0 / (fovy / 2.0).tan();
        let expected_a = far / (far - near);
        let expected_b = -near * far / (far - near);

        // Test with more lenient epsilon due to potential floating-point precision issues
        let epsilon = 1e-6;

        assert!(
            (perspective.data[0][0] - expected_w).abs() < epsilon,
            "Mismatch at [0][0]: expected {}, got {}",
            expected_w,
            perspective.data[0][0]
        );
        assert!(
            (perspective.data[1][1] - expected_h).abs() < epsilon,
            "Mismatch at [1][1]: expected {}, got {}",
            expected_h,
            perspective.data[1][1]
        );
        assert!(
            (perspective.data[2][2] - expected_a).abs() < epsilon,
            "Mismatch at [2][2]: expected {}, got {}",
            expected_a,
            perspective.data[2][2]
        );
        assert!(
            (perspective.data[3][2] - expected_b).abs() < epsilon,
            "Mismatch at [3][2]: expected {}, got {}",
            expected_b,
            perspective.data[3][2]
        );

        assert_eq!(perspective.data[2][3], 0.0);
        assert_eq!(perspective.data[3][3], 1.0);

        // Additional checks to ensure the matrix structure is correct
        assert_eq!(perspective.data[0][1], 0.0);
        assert_eq!(perspective.data[0][2], 0.0);
        assert_eq!(perspective.data[0][3], 0.0);
        assert_eq!(perspective.data[1][0], 0.0);
        assert_eq!(perspective.data[1][2], 0.0);
        assert_eq!(perspective.data[1][3], 0.0);
        assert_eq!(perspective.data[2][0], 0.0);
        assert_eq!(perspective.data[2][1], 0.0);
        assert_eq!(perspective.data[3][0], 0.0);
        assert_eq!(perspective.data[3][1], 0.0);

        // Print the actual values for debugging
        println!("Actual [2][2]: {}", perspective.data[2][2]);
        println!("Expected [2][2]: {}", expected_a);
    }

    #[test]
    fn test_to_byte_slice() {
        // Test Vec3
        let vec3 = Vec3::new([1.0, 2.0, 3.0]);
        let vec3_bytes = as_byte_slice(&vec3.data);
        assert_eq!(
            vec3_bytes,
            [
                0x00, 0x00, 0x80, 0x3F, // 1.0
                0x00, 0x00, 0x00, 0x40, // 2.0
                0x00, 0x00, 0x40, 0x40 // 3.0
            ]
        );

        // Test Vec4
        let vec4 = Vec4::new([1.0, 2.0, 3.0, 4.0]);
        let vec4_bytes = as_byte_slice(&vec4.data);
        assert_eq!(
            vec4_bytes,
            [
                0x00, 0x00, 0x80, 0x3F, // 1.0
                0x00, 0x00, 0x00, 0x40, // 2.0
                0x00, 0x00, 0x40, 0x40, // 3.0
                0x00, 0x00, 0x80, 0x40 // 4.0
            ]
        );

        // Test Matrix4
        let matrix4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let flat_matrix4 = matrix4.data.concat();
        let matrix4_bytes = as_byte_slice(&flat_matrix4);
        assert_eq!(
            matrix4_bytes,
            [
                0x00, 0x00, 0x80, 0x3F, // 1.0
                0x00, 0x00, 0x00, 0x40, // 2.0
                0x00, 0x00, 0x40, 0x40, // 3.0
                0x00, 0x00, 0x80, 0x40, // 4.0
                0x00, 0x00, 0xA0, 0x40, // 5.0
                0x00, 0x00, 0xC0, 0x40, // 6.0
                0x00, 0x00, 0xE0, 0x40, // 7.0
                0x00, 0x00, 0x00, 0x41, // 8.0
                0x00, 0x00, 0x10, 0x41, // 9.0
                0x00, 0x00, 0x20, 0x41, // 10.0
                0x00, 0x00, 0x30, 0x41, // 11.0
                0x00, 0x00, 0x40, 0x41, // 12.0
                0x00, 0x00, 0x50, 0x41, // 13.0
                0x00, 0x00, 0x60, 0x41, // 14.0
                0x00, 0x00, 0x70, 0x41, // 15.0
                0x00, 0x00, 0x80, 0x41 // 16.0
            ]
        );
    }
}
