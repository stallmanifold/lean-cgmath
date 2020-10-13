use crate::rotation::{
    Rotation2,
    Rotation3,
};
use crate::translation::{
    Translation2,
    Translation3,
};
use crate::scalar::{
    ScalarFloat,
};
use crate::matrix::{
    Matrix3x3,
    Matrix4x4,
};
use crate::point::{
    Point2,
    Point3,
};
use crate::vector::{
    Vector2,
    Vector3,
};
use crate::angle::{
    Radians,
};
use crate::transform::{
    Transform2,
    Transform3,
};

use core::fmt;
use core::ops;


#[repr(C)]
pub struct Isometry2<S> {
    rotation: Rotation2<S>,
    translation: Translation2<S>,
}

impl<S> Isometry2<S> where S: ScalarFloat {
    #[inline]
    pub fn from_parts(translation: Translation2<S>, rotation: Rotation2<S>) -> Isometry2<S> {
        Isometry2 {
            rotation: rotation,
            translation: translation,
        }
    }

    #[inline]
    pub fn from_translation(translation: Translation2<S>) -> Isometry2<S> {
        Self::from_parts(translation, Rotation2::identity())
    }

    #[inline]
    pub fn from_rotation(rotation: Rotation2<S>) -> Isometry2<S> {
        Self::from_parts(Translation2::identity(), rotation)
    }

    #[inline]
    pub fn from_angle<A: Into<Radians<S>>>(angle: A) -> Isometry2<S> {
        let translation = Translation2::identity();
        let rotation = Rotation2::from_angle(angle);
        
        Self::from_parts(translation, rotation)
    }

    #[inline]
    pub fn to_transform2d(&self) -> Transform2<S> {
        let matrix = self.to_affine_matrix();
        Transform2::from_specialized(matrix)
    }

    #[inline]
    pub fn to_affine_matrix(&self) -> Matrix3x3<S> {
        let zero = S::zero();
        let one = S::one();
        let rotation_matrix = self.rotation.matrix();
        let translation = self.translation.as_ref();

        Matrix3x3::new(
            rotation_matrix.c0r0, rotation_matrix.c0r1, zero,
            rotation_matrix.c1r0, rotation_matrix.c1r1, zero,
            translation[0], translation[1], one
        )
    }
    
    #[inline]
    pub fn rotation(&self) -> &Rotation2<S> {
        &self.rotation
    }

    #[inline]
    pub fn translation(&self) -> &Translation2<S> {
        &self.translation
    }

    #[inline]
    pub fn inverse(&self) -> Isometry2<S> {
        Isometry2 {
            rotation: self.rotation.inverse(),
            translation: self.translation.inverse(),
        }
    }

    /// Apply a rotation followed by a translation.
    #[inline]
    pub fn transform_point(&self, point: &Point2<S>) -> Point2<S> {
        let rotated_point = self.rotation.rotate_point(&point);

        self.translation.translate_point(&rotated_point)
    }

    /// Apply a rotation followed by a translation.
    #[inline]
    pub fn transform_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        let rotated_vector = self.rotation.rotate_vector(vector);
        
        self.translation.translate_vector(&rotated_vector)
    }

    #[inline]
    pub fn inverse_transform_point(&self, point: &Point2<S>) -> Point2<S> {
        let rotated_point = self.rotation.inverse_rotate_point(point);

        self.translation.inverse_translate_point(&rotated_point)
    }
    
    #[inline]
    pub fn inverse_transform_vector(&self, vector: &Vector2<S>) -> Vector2<S> {
        let rotated_vector = self.rotation.inverse_rotate_vector(vector);

        self.translation.inverse_translate_vector(&rotated_vector)
    }

    #[inline]
    pub fn identity() -> Isometry2<S> {
        Isometry2 {
            rotation: Rotation2::identity(),
            translation: Translation2::identity()
        }
    }
}
