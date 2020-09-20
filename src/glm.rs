#![cfg(feature = "glm")]
use crate::vector::{
    Vector1,
    Vector2,
    Vector3,
    Vector4,
};
use crate::matrix::{
    Matrix2x2,
    Matrix3x3,
    Matrix4x4,
};
use crate::scalar::{
    Scalar,
    ScalarFloat,
};
use crate::angle::{
    Radians,
};
use crate::traits::{
    DotProduct,
    CrossProduct,
};
use crate::quaternion::{
    Quaternion,
};
use crate::projection::{
    OrthographicSpec,
    PerspectiveSpec,
    PerspectiveFovSpec,
};


/// Construct a new one-dimensional vector. This follows the style of
/// other GLSL vector constructors even though GLSL itself lacks a
/// `vec1()` function.
pub fn vec1<S: Scalar>(x: S) -> Vector1<S> {
    Vector1::new(x)
}

/// Construct a new two-dimensional vector in the style of
/// a GLSL `vec2` function.
pub fn vec2<S: Scalar>(x: S, y: S) -> Vector2<S> {
    Vector2::new(x, y)
}

/// Construct a new three-dimensional vector in the style of
/// a GLSL `vec3` function.
pub fn vec3<S: Scalar>(x: S, y: S, z: S) -> Vector3<S> {
    Vector3::new(x, y, z)
}

/// Construct a new four-dimensional vector in the style of
/// a GLSL `vec4` function.
pub fn vec4<S: Scalar>(x: S, y: S, z: S, w: S) -> Vector4<S> {
    Vector4::new(x, y, z, w)
}

/// Create a new quaternion in the style of the glm `quat` function.
pub fn quat<S: Scalar>(s: S, x: S, y: S, z: S) -> Quaternion<S> {
    Quaternion::new(s, x, y, z)
}

/// Create a new 2x2 matrix in the style of a GLSL `mat2` type constructor.
pub fn mat2<S: Scalar>(m00: S, m01: S, m10: S, m11: S) -> Matrix2x2<S> {
    Matrix2x2::new(m00, m01, m10, m11)
}

/// Create a new 3x3 matrix in the style of a GLSL `mat3` type constructor.
pub fn mat3<S: Scalar>(
    m00: S, m01: S, m02: S, m10: S, m11: S, m12: S, m20: S, m21: S, m22: S) -> Matrix3x3<S> {
    
    Matrix3x3::new(
        m00, m01, m02, 
        m10, m11, m12, 
        m20, m21, m22
    )
}

/// Create a new 4x4 matrix in the style of a GLSL `mat4` type constructor.
pub fn mat4<S: Scalar>(
    m00: S, m01: S, m02: S, m03: S, 
    m10: S, m11: S, m12: S, m13: S,
    m20: S, m21: S, m22: S, m23: S,
    m30: S, m31: S, m32: S, m33: S) -> Matrix4x4<S> {
    
    Matrix4x4::new(
        m00, m01, m02, m03,
        m10, m11, m12, m13,
        m20, m21, m22, m23,
        m30, m31, m32, m33
    )
}

/// Compute the orthographic projection matrix for converting from camera space to
/// normalized device coordinates.
pub fn ortho<S, Spec>(spec: Spec) -> Matrix4x4<S> 
    where 
        S: ScalarFloat,
        Spec: Into<OrthographicSpec<S>>,
{
    Matrix4x4::from(spec.into())
}

/// Compute a perspective matrix from a view frustum.
pub fn frustum<S, Spec>(spec: Spec) -> Matrix4x4<S> 
    where 
        S: ScalarFloat,
        Spec: Into<PerspectiveSpec<S>>        
{
    Matrix4x4::from(spec.into())
}

/// Compute the perspective matrix for converting from camera space to 
/// normalized device coordinates. 
pub fn perspective<S, A: Into<Radians<S>>>(
    fovy: A, aspect: S, near: S, far: S) -> Matrix4x4<S> 
    where S: ScalarFloat
{    
    let spec = PerspectiveFovSpec::new(fovy, aspect, near, far);
    
    Matrix4x4::from(spec)
}

/// Compute the dot product between two vectors.
pub fn dot<W, V>(v1: V, v2: W) -> <V as DotProduct<W>>::Output 
    where
        W: Copy + Clone,
        V: DotProduct<W>,
{
    V::dot(v1, v2)
}

/// Compute the cross product of two three-dimensional vectors.
pub fn cross<'a, 'b, S: Scalar>(v1: &'a Vector3<S>, v2: &'b Vector3<S>) -> Vector3<S> {
    v1.cross(v2)
}

/// Compute the cross product of a pair of two-dimensional vectors
/// extended to three-dimensional vectors with the _z-axis_ components set to zero.
pub fn cross2d<'a, 'b, S: Scalar>(v1: &'a Vector2<S>, v2: &'b Vector2<S>) -> Vector3<S> {
    let v1_3d = v1.expand(S::zero());
    let v2_3d = v2.expand(S::zero());
    
    v1_3d.cross(v2_3d)
}

