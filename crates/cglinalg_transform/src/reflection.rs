use cglinalg_numeric::{
    SimdScalarFloat,
};
use cglinalg_core::{
    Matrix3x3,
    Matrix4x4,
    Vector2,
    Vector3,
    Point2,
    Point3,
    Unit,
};
use crate::transform::{
    Transform2,
    Transform3,
};

use core::fmt;
use core::ops;


/// A reflection transformation about a plane (line) in two dimensions.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Reflection2<S> {
    /// A known point on the line of reflection.
    bias: Vector2<S>,
    /// The normal vector to the plane.
    normal: Vector2<S>,
    /// The matrix representing the affine transformation.
    matrix: Matrix3x3<S>,
}

impl<S> Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    /// Construct a new reflection transformation from the vector normal to the 
    /// plane of reflection.
    /// 
    /// # Example
    /// 
    /// A reflection about the **y-axis** using the origin as the bias.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// #
    /// // Normal to the plane of reflection.
    /// let normal = Unit::from_value(Vector2::unit_x());
    /// let bias = Vector2::zero();
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// let vector = Vector2::new(1_f64, 2_f64);
    /// let expected = Vector2::new(-1_f64, 2_f64);
    /// let result = reflection.reflect_vector(&vector);
    /// 
    /// assert_eq!(result, expected);
    /// 
    /// // In two dimensions, we can just as well use the opposite normal.
    /// let opposite_normal = Unit::from_value(-Vector2::unit_x());
    /// let opposite_reflection = Reflection2::from_normal_bias(&opposite_normal, &bias);
    /// let opposite_result = opposite_reflection.reflect_vector(&vector);
    /// 
    /// assert_eq!(opposite_result, expected);
    /// ```
    #[inline]
    pub fn from_normal_bias(normal: &Unit<Vector2<S>>, bias: &Vector2<S>) -> Self {
        Self {
            bias: *bias,
            normal: normal.into_inner(),
            matrix: Matrix3x3::from_affine_reflection(normal, bias),
        }
    }

    /// Return the bias for calculating the reflections.
    ///
    /// The `bias` is the coordinates of a known point in the plane of 
    /// reflection.
    /// 
    /// # Example
    /// 
    /// A reflection about the **y-axis** using the origin as the bias.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// #
    /// // Normal to the plane of reflection.
    /// let normal: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_x());
    /// let bias = Vector2::zero();
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// 
    /// assert_eq!(reflection.bias(), bias);
    /// ```
    #[inline]
    pub const fn bias(&self) -> Vector2<S> {
        self.bias
    }

    /// Return the normal vector to the reflection plane.
    ///
    /// There is an ambiguity in the choice of normal to a line in
    /// two dimensions. One can choose either a normal vector or its negation
    /// to construct the reflection and get the same reflection transformation.
    /// 
    /// # Example
    /// 
    /// A reflection about the **y-axis** using the origin as the bias.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// #
    /// // Normal to the plane of reflection.
    /// let normal = Vector2::unit_x();
    /// let unit_normal: Unit<Vector2<f64>> = Unit::from_value(normal);
    /// let bias = Vector2::zero();
    /// let reflection = Reflection2::from_normal_bias(&unit_normal, &bias);
    /// 
    /// assert_eq!(reflection.normal(), normal);
    /// ```
    #[inline]
    pub const fn normal(&self) -> Vector2<S> {
        self.normal
    }

    /// Return the underlying matrix of the reflection transformation.
    /// 
    /// # Example
    /// 
    /// A reflection about the **y-axis** using the origin as the bias.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use cglinalg_core::{
    /// #     Matrix3x3,
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// #
    /// // Normal to the plane of reflection.
    /// let normal: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_x());
    /// let bias = Vector2::zero();
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// let expected = Matrix3x3::new(
    ///     -1_f64, 0_f64, 0_f64,
    ///      0_f64, 1_f64, 0_f64,
    ///      0_f64, 0_f64, 1_f64
    /// );
    /// let result = reflection.matrix();
    /// 
    /// assert_eq!(result, &expected);
    /// 
    /// // In two dimensions, we can just as well use the opposite normal.
    /// let opposite_normal = Unit::from_value(-Vector2::unit_x());
    /// let opposite_reflection = Reflection2::from_normal_bias(&opposite_normal, &bias);
    /// 
    /// assert_eq!(opposite_reflection.matrix(), reflection.matrix());
    /// ```
    #[inline]
    pub const fn matrix(&self) -> &Matrix3x3<S> {
        &self.matrix
    }

    /// Reflect a vector across a line described by the reflection 
    /// transformation.
    ///
    /// # Example
    ///
    /// ```
    /// # use cglinalg_core::{
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq, 
    /// # };
    /// #
    /// let normal: Unit<Vector2<f64>> = Unit::from_value(Vector2::new(
    ///     -1_f64 / 2_f64, 
    ///      1_f64
    /// ));
    /// let bias = Vector2::new(0_f64, 1_f64);
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// let vector = Vector2::new(1_f64, 1_f64);
    /// let expected = Vector2::new(7_f64 / 5_f64, 1_f64 / 5_f64);
    /// let result = reflection.reflect_vector(&vector);
    ///
    /// assert_relative_eq!(result, expected, epsilon = 1e-8);
    /// ```
    #[inline]
    pub fn reflect_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    /// Reflect a point across a line described by the reflection 
    /// transformation.
    ///
    /// # Example
    ///
    /// ```
    /// # use cglinalg_core::{
    /// #     Point2,
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq, 
    /// # };
    /// #
    /// let normal: Unit<Vector2<f64>> = Unit::from_value(Vector2::new(
    ///     -1_f64 / 2_f64, 
    ///      1_f64
    /// ));
    /// let bias = Vector2::new(0_f64, 1_f64);
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// let point = Point2::new(1_f64, 1_f64);
    /// let expected = Point2::new(3_f64 / 5_f64, 9_f64 / 5_f64);
    /// let result = reflection.reflect_point(&point);
    ///
    /// assert_relative_eq!(result, expected, epsilon = 1e-8);
    /// ```
    #[inline]
    pub fn reflect_point(&self, point: &Point2<S>) -> Point2<S> {
        Point2::from_homogeneous(&(self.matrix * point.to_homogeneous())).unwrap()
    }

    /// Compute the identity reflection. 
    ///
    /// The identity reflection is a reflection that does not move a point 
    /// or vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use cglinalg_core::{
    /// #     Vector2, 
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// # };
    /// #
    /// let reflection = Reflection2::identity();
    /// let vector = Vector2::new(1_f64, 2_f64);
    /// let expected = vector;
    /// let result = reflection.reflect_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self { 
            bias: Vector2::zero(),
            normal: Vector2::zero(), 
            matrix: Matrix3x3::identity(),
        }
    }

    /// Convert a reflection into a generic transformation.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `y == 2 * x`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection2,
    /// #     Transform2,
    /// # };
    /// # use cglinalg_core::{
    /// #     Matrix3x3,
    /// #     Vector2,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq,
    /// # };
    /// #
    /// let normal = Unit::from_value(Vector2::new(-2_f64, 1_f64));
    /// let bias = Vector2::zero();
    /// let reflection = Reflection2::from_normal_bias(&normal, &bias);
    /// let expected = Transform2::from_matrix_unchecked(Matrix3x3::new(
    ///     -3_f64 / 5_f64, 4_f64 / 5_f64, 0_f64,
    ///      4_f64 / 5_f64, 3_f64 / 5_f64, 0_f64,
    ///      0_f64,         0_f64,         1_f64
    /// ));
    /// let result = reflection.to_transform();
    /// 
    /// assert_relative_eq!(result, expected, epsilon = 1e-15);
    /// ```
    #[inline]
    pub fn to_transform(&self) -> Transform2<S> {
        Transform2::from_specialized(self)
    }
}


impl<S> AsRef<Matrix3x3<S>> for Reflection2<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix3x3<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for Reflection2<S> 
where 
    S: fmt::Display 
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Reflection2 [{}]",
            self.matrix
        )
    }
}

impl<S> From<Reflection2<S>> for Matrix3x3<S> 
where 
    S: Copy 
{
    #[inline]
    fn from(transformation: Reflection2<S>) -> Matrix3x3<S> {
        transformation.matrix
    }
}

impl<S> From<&Reflection2<S>> for Matrix3x3<S> 
where 
    S: Copy
{
    #[inline]
    fn from(transformation: &Reflection2<S>) -> Matrix3x3<S> {
        transformation.matrix
    }
}

impl<S> approx::AbsDiffEq for Reflection2<S> 
where 
    S: SimdScalarFloat
{
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Vector2::abs_diff_eq(&self.bias, &other.bias, epsilon)
            && Vector2::abs_diff_eq(&self.normal, &other.normal, epsilon)
            && Matrix3x3::abs_diff_eq(&self.matrix, &other.matrix, epsilon)
    }
}

impl<S> approx::RelativeEq for Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
       Vector2::relative_eq(&self.bias, &other.bias, epsilon, max_relative)
           && Vector2::relative_eq(&self.normal, &other.normal, epsilon, max_relative)
           && Matrix3x3::relative_eq(&self.matrix, &other.matrix, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector2::ulps_eq(&self.bias, &other.bias, epsilon, max_ulps)
            && Vector2::ulps_eq(&self.normal, &other.normal, epsilon, max_ulps)
            && Matrix3x3::ulps_eq(&self.matrix, &other.matrix, epsilon, max_ulps)
    }
}

impl<S> ops::Mul<Point2<S>> for Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: Point2<S>) -> Self::Output {
        self.reflect_point(&other)
    }
}

impl<S> ops::Mul<&Point2<S>> for Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: &Point2<S>) -> Self::Output {
        self.reflect_point(other)
    }
}

impl<S> ops::Mul<Point2<S>> for &Reflection2<S> 
where 
    S: SimdScalarFloat 
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: Point2<S>) -> Self::Output {
        self.reflect_point(&other)
    }
}

impl<'a, 'b, S> ops::Mul<&'a Point2<S>> for &'b Reflection2<S> 
where 
    S: SimdScalarFloat
{
    type Output = Point2<S>;

    #[inline]
    fn mul(self, other: &'a Point2<S>) -> Self::Output {
        self.reflect_point(other)
    }
}


/// A reflection transformation about a plane in three dimensions.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Reflection3<S> {
    /// a known point on the plane of reflection.
    bias: Vector3<S>,
    /// The normal vector to the plane.
    normal: Vector3<S>,
    /// The matrix representing the affine transformation.
    matrix: Matrix4x4<S>,
}

impl<S> Reflection3<S> 
where 
    S: SimdScalarFloat 
{
    /// Construct a new reflection transformation from the vector normal to the 
    /// plane of reflection.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `x + y == -z`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq,
    /// # };
    /// #
    /// let normal = Unit::from_value(Vector3::new(1_f64, 1_f64, 1_f64));
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// let vector = Vector3::new(-5_f64, 7_f64, -3_f64);
    /// let expected = Vector3::new(-13_f64 / 3_f64, 23_f64 / 3_f64, -7_f64 / 3_f64);
    /// let result = reflection.reflect_vector(&vector);
    /// 
    /// assert_relative_eq!(result, expected, epsilon = 1e-15);
    /// ```
    #[inline]
    pub fn from_normal_bias(normal: &Unit<Vector3<S>>, bias: &Vector3<S>) -> Self {
        Self {
            bias: *bias,
            normal: normal.into_inner(),
            matrix: Matrix4x4::from_affine_reflection(normal, bias),
        }
    }

    /// Return the bias for calculating the reflections.
    ///
    /// The `bias` is the coordinates of a known point in the plane of 
    /// reflection.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `x + y == -z`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// #
    /// let normal = Unit::from_value(Vector3::new(1_f64, 1_f64, 1_f64));
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// 
    /// assert_eq!(reflection.bias(), bias);
    /// ```
    #[inline]
    pub const fn bias(&self) -> Vector3<S> {
        self.bias
    }

    /// Return the normal vector to the reflection plane.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `x + y == -z`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// # use cglinalg_core::{
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// #
    /// let normal = Vector3::new(1_f64, 1_f64, 1_f64);
    /// let unit_normal = Unit::from_value(normal);
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&unit_normal, &bias);
    /// let expected = normal / normal.norm();
    /// let result = reflection.normal();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn normal(&self) -> Vector3<S> {
        self.normal
    }

    /// Return the underlying matrix of the reflection transformation.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `x + y == -z`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// # use cglinalg_core::{
    /// #     Matrix4x4,
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq,
    /// # };
    /// #
    /// let normal = Unit::from_value(Vector3::new(1_f64, 1_f64, 1_f64));
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// let expected = Matrix4x4::new(
    ///      1_f64 / 3_f64, -2_f64 / 3_f64, -2_f64 / 3_f64, 0_f64,
    ///     -2_f64 / 3_f64,  1_f64 / 3_f64, -2_f64 / 3_f64, 0_f64,
    ///     -2_f64 / 3_f64, -2_f64 / 3_f64,  1_f64 / 3_f64, 0_f64,
    ///      0_f64,          0_f64,          0_f64,         1_f64
    /// );
    /// let result = reflection.matrix();
    /// 
    /// assert_relative_eq!(result, &expected, epsilon = 1e-15);
    /// ```
    #[inline]
    pub const fn matrix(&self) -> &Matrix4x4<S> {
        &self.matrix
    }

    /// Reflect a vector across the plane described by the reflection 
    /// transformation.
    /// 
    /// # Example
    ///
    /// ```
    /// # use cglinalg_core::{
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// #
    /// let normal: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// let vector = Vector3::new(1_f64, 1_f64, 1_f64);
    /// let expected = Vector3::new(1_f64, 1_f64, -1_f64);
    /// let result = reflection.reflect_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn reflect_vector(&self, vector: &Vector3<S>) -> Vector3<S> {
        (self.matrix * vector.extend(S::zero())).contract()
    }

    /// Reflect a point across the plane described by the reflection 
    /// transformation.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cglinalg_core::{
    /// #     Vector3,
    /// #     Point3,
    /// #     Unit,
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// #
    /// let normal: Unit<Vector3<f64>> = Unit::from_value(Vector3::unit_z());
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// let point = Point3::new(1_f64, 1_f64, 1_f64);
    /// let expected = Point3::new(1_f64, 1_f64, -1_f64);
    /// let result = reflection.reflect_point(&point);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn reflect_point(&self, point: &Point3<S>) -> Point3<S> {
        Point3::from_homogeneous(&(self.matrix * point.to_homogeneous())).unwrap()
    }

    /// Compute the identity reflection. 
    ///
    /// The identity reflection is a reflection that does not move a point 
    /// or vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use cglinalg_core::{
    /// #     Vector3, 
    /// # };
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// # };
    /// #
    /// let reflection = Reflection3::identity();
    /// let vector = Vector3::new(1_f64, 2_f64, 3_f64);
    /// let expected = vector;
    /// let result = reflection.reflect_vector(&vector);
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self { 
            bias: Vector3::zero(),
            normal: Vector3::zero(), 
            matrix: Matrix4x4::identity(),
        }
    }

    /// Convert a reflection to a generic transformation.
    /// 
    /// # Example
    /// 
    /// A reflection about the plane `x + y == -z`.
    /// ```
    /// # use cglinalg_transform::{
    /// #     Reflection3,
    /// #     Transform3,
    /// # };
    /// # use cglinalg_core::{
    /// #     Matrix4x4,
    /// #     Vector3,
    /// #     Unit,
    /// # };
    /// # use approx::{
    /// #     assert_relative_eq,
    /// # };
    /// #
    /// let normal = Unit::from_value(Vector3::new(1_f64, 1_f64, 1_f64));
    /// let bias = Vector3::zero();
    /// let reflection = Reflection3::from_normal_bias(&normal, &bias);
    /// let expected = Transform3::from_matrix_unchecked(Matrix4x4::new(
    ///      1_f64 / 3_f64, -2_f64 / 3_f64, -2_f64 / 3_f64, 0_f64,
    ///     -2_f64 / 3_f64,  1_f64 / 3_f64, -2_f64 / 3_f64, 0_f64,
    ///     -2_f64 / 3_f64, -2_f64 / 3_f64,  1_f64 / 3_f64, 0_f64,
    ///      0_f64,          0_f64,          0_f64,         1_f64
    /// ));
    /// let result = reflection.to_transform();
    /// 
    /// assert_relative_eq!(result, &expected, epsilon = 1e-15);
    /// ```
    #[inline]
    pub fn to_transform(&self) -> Transform3<S> {
        Transform3::from_specialized(self.matrix)
    }
}

impl<S> AsRef<Matrix4x4<S>> for Reflection3<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix4x4<S> {
        &self.matrix
    }
}

impl<S> fmt::Display for Reflection3<S> 
where 
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Reflection3 [{}]",
            self.matrix
        )
    }
}

impl<S> From<Reflection3<S>> for Matrix4x4<S> 
where 
    S: Copy
{
    #[inline]
    fn from(transformation: Reflection3<S>) -> Matrix4x4<S> {
        transformation.matrix
    }
}

impl<S> From<&Reflection3<S>> for Matrix4x4<S> 
where 
    S: Copy
{
    #[inline]
    fn from(transformation: &Reflection3<S>) -> Matrix4x4<S> {
        transformation.matrix
    }
}

impl<S> approx::AbsDiffEq for Reflection3<S> 
where 
    S: SimdScalarFloat
{
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Vector3::abs_diff_eq(&self.bias, &other.bias, epsilon)
            && Vector3::abs_diff_eq(&self.normal, &other.normal, epsilon)
            && Matrix4x4::abs_diff_eq(&self.matrix, &other.matrix, epsilon)
    }
}

impl<S> approx::RelativeEq for Reflection3<S> 
where 
    S: SimdScalarFloat 
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
       Vector3::relative_eq(&self.bias, &other.bias, epsilon, max_relative)
           && Vector3::relative_eq(&self.normal, &other.normal, epsilon, max_relative)
           && Matrix4x4::relative_eq(&self.matrix, &other.matrix, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Reflection3<S> 
where 
    S: SimdScalarFloat 
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Vector3::ulps_eq(&self.bias, &other.bias, epsilon, max_ulps)
            && Vector3::ulps_eq(&self.normal, &other.normal, epsilon, max_ulps)
            && Matrix4x4::ulps_eq(&self.matrix, &other.matrix, epsilon, max_ulps)
    }
}

impl<S> ops::Mul<Point3<S>> for Reflection3<S> 
where 
    S: SimdScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: Point3<S>) -> Self::Output {
        self.reflect_point(&other)
    }
}

impl<S> ops::Mul<&Point3<S>> for Reflection3<S> 
where 
    S: SimdScalarFloat
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: &Point3<S>) -> Self::Output {
        self.reflect_point(other)
    }
}

impl<S> ops::Mul<Point3<S>> for &Reflection3<S> 
where 
    S: SimdScalarFloat 
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: Point3<S>) -> Self::Output {
        self.reflect_point(&other)
    }
}

impl<'a, 'b, S> ops::Mul<&'a Point3<S>> for &'b Reflection3<S> 
where 
    S: SimdScalarFloat
{
    type Output = Point3<S>;

    #[inline]
    fn mul(self, other: &'a Point3<S>) -> Self::Output {
        self.reflect_point(other)
    }
}

