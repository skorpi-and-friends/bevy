macro_rules! impl_transform {
    ($type_name:ident, $real:ident, $vec3:ident, $quat:ident, $mat3:ident, $mat4:ident) => {
        impl $type_name {
            /// Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component
            /// is used for z-ordering elements: higher `z`-value will be in front of lower
            /// `z`-value.
            #[inline]
            pub fn from_xyz(x: $real, y: $real, z: $real) -> Self {
                Self::from_translation($vec3::new(x, y, z))
            }

            /// Creates a new identity [`Transform`], with no translation, rotation, and a scale of 1 on
            /// all axes.
            #[inline]
            pub const fn identity() -> Self {
                Transform {
                    translation: $vec3::ZERO,
                    rotation: $quat::IDENTITY,
                    scale: $vec3::ONE,
                }
            }

            /// Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
            /// transformation matrix.
            #[inline]
            pub fn from_matrix(matrix: $mat4) -> Self {
                let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

                Self {
                    translation,
                    rotation,
                    scale,
                }
            }

            /// Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on
            /// all axes.
            #[inline]
            pub fn from_translation(translation: $vec3) -> Self {
                Self {
                    translation,
                    ..Default::default()
                }
            }

            /// Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on
            /// all axes.
            #[inline]
            pub fn from_rotation(rotation: $quat) -> Self {
                Self {
                    rotation,
                    ..Default::default()
                }
            }

            /// Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on
            /// all axes.
            #[inline]
            pub fn from_scale(scale: $vec3) -> Self {
                Self {
                    scale,
                    ..Default::default()
                }
            }

            /// Updates and returns this [`Transform`] by rotating it so that its unit vector in the
            /// local z direction is toward `target` and its unit vector in the local y direction
            /// is toward `up`.
            #[inline]
            pub fn looking_at(mut self, target: $vec3, up: $vec3) -> Self {
                self.look_at(target, up);
                self
            }

            /// Returns this [`Transform`] with a new translation.
            #[inline]
            pub fn with_translation(mut self, translation: $vec3) -> Self {
                self.translation = translation;
                self
            }

            /// Returns this [`Transform`] with a new rotation.
            #[inline]
            pub fn with_rotation(mut self, rotation: $quat) -> Self {
                self.rotation = rotation;
                self
            }

            /// Returns this [`Transform`] with a new scale.
            #[inline]
            pub fn with_scale(mut self, scale: $vec3) -> Self {
                self.scale = scale;
                self
            }

            /// Returns the 3d affine transformation matrix from this transforms translation,
            /// rotation, and scale.
            #[inline]
            pub fn compute_matrix(&self) -> $mat4 {
                $mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
            }

            /// Get the unit vector in the local x direction.
            #[inline]
            pub fn local_x(&self) -> $vec3 {
                self.rotation * $vec3::X
            }

            /// Equivalent to -local_x()
            #[inline]
            pub fn left(&self) -> $vec3 {
                -self.local_x()
            }

            /// Equivalent to local_x()
            #[inline]
            pub fn right(&self) -> $vec3 {
                self.local_x()
            }

            /// Get the unit vector in the local y direction.
            #[inline]
            pub fn local_y(&self) -> $vec3 {
                self.rotation * $vec3::Y
            }

            /// Equivalent to local_y()
            #[inline]
            pub fn up(&self) -> $vec3 {
                self.local_y()
            }

            /// Equivalent to -local_y()
            #[inline]
            pub fn down(&self) -> $vec3 {
                -self.local_y()
            }

            /// Get the unit vector in the local z direction.
            #[inline]
            pub fn local_z(&self) -> $vec3 {
                self.rotation * $vec3::Z
            }

            /// Equivalent to -local_z()
            #[inline]
            pub fn forward(&self) -> $vec3 {
                -self.local_z()
            }

            /// Equivalent to local_z()
            #[inline]
            pub fn back(&self) -> $vec3 {
                self.local_z()
            }

            /// Rotates the transform by the given rotation.
            #[inline]
            pub fn rotate(&mut self, rotation: $quat) {
                self.rotation = rotation * self.rotation;
            }

            /// Returns a [`$vec3`] of this [`Transform`] applied to `value`.
            #[inline]
            pub fn mul_$vec3(&self, mut value: $vec3) -> $vec3 {
                value = self.rotation * value;
                value = self.scale * value;
                value += self.translation;
                value
            }

            /// Changes the `scale` of this [`Transform`], multiplying the current `scale` by
            /// `scale_factor`.
            #[inline]
            pub fn apply_non_uniform_scale(&mut self, scale_factor: $vec3) {
                self.scale *= scale_factor;
            }

            /// Rotates this [`Transform`] so that its unit vector in the local z direction is toward
            /// `target` and its unit vector in the local y direction is toward `up`.
            #[inline]
            pub fn look_at(&mut self, target: $vec3, up: $vec3) {
                let forward = $vec3::normalize(self.translation - target);
                let right = up.cross(forward).normalize();
                let up = forward.cross(right);
                self.rotation = $quat::from_mat3(&$mat3::from_cols(right, up, forward));
            }
        }

        impl Default for $type_name {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl Mul<$type_name> for $type_name {
            type Output = $type_name;

            #[inline]
            fn mul(self, transform: $type_name) -> Self::Output {
                let translation = self.mul_vec3(transform.translation);
                let rotation = self.rotation * transform.rotation;
                let scale = self.scale * transform.scale;
                Self {
                    translation,
                    rotation,
                    scale,
                }
            }
        }

        impl Mul<$vec3> for $type_name {
            type Output = $vec3;

            fn mul(self, value: $vec3) -> Self::Output {
                self.mul_vec3(value)
            }
        }
    };
}

// export both versions despite feature flag
pub use xform_32::*;
pub use xform_64::*;

// default to one version according to feature flag
pub use defualt_tform_types::*;

#[cfg(not(feature = "xform_64"))]
mod defualt_tform_types {
    use super::xform_32::*;

    // use bevy_math::*;

    // pub type TVec3 = Vec3;
    // pub type TQuat = Quat;
    // pub type TVec2 = Vec2;
    // pub type TMat3 = Mat3;
    // pub type TMat4 = Mat4;

    pub type Transform = Transform32;
    pub type GlobalTransform = GlobalTransform32;
}

#[cfg(feature = "xform_64")]
mod defualt_tform_types {
    use super::xform_64::*;

    // pub type TVec3 = DVec3;
    // pub type TQuat = DQuat;
    // pub type TVec2 = DVec2;
    // pub type TMat3 = DMat3;
    // pub type TMat4 = DMat4;

    pub type Transform = Transform64;
    pub type GlobalTransform = GlobalTransform64;
}

mod xform_32 {
    use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
    use bevy_math::{F32Convert, F64Convert, Mat3, Mat4, Quat, Vec3};
    use bevy_reflect::Reflect;
    use std::ops::Mul;

    /// Describe the position of an entity. If the entity has a parent, the position is relative
    /// to its parent position.
    ///
    /// * To place or move an entity, you should set its [`Transform`].
    /// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
    /// * To get the global position of an entity, you should get its [`GlobalTransform`].
    ///
    /// ## [`Transform`] and [`GlobalTransform`]
    ///
    /// [`Transform`] is the position of an entity relative to its parent position, or the reference
    /// frame if it doesn't have a [`Parent`](super::Parent).
    ///
    /// [`GlobalTransform`] is the position of an entity relative to the reference frame.
    ///
    /// [`GlobalTransform`] is updated from [`Transform`] in the system
    /// [`transform_propagate_system`](crate::transform_propagate_system::transform_propagate_system).
    ///
    /// In pseudo code:
    /// ```ignore
    /// for entity in entities_without_parent:
    ///     set entity.global_transform to entity.transform
    ///     recursively:
    ///         set parent to current entity
    ///         for child in parent.children:
    ///             set child.global_transform to parent.global_transform * child.transform
    /// ```
    ///
    /// This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
    /// update the[`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
    /// before the [`GlobalTransform`] is updated.
    #[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct Transform32 {
        /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
        pub translation: Vec3,
        /// Rotation of the entity.
        pub rotation: Quat,
        /// Scale of the entity.
        pub scale: Vec3,
    }

    impl_transform!(Transform32, f32, Vec3, Quat, Mat3, Mat4);

    impl From<GlobalTransform32> for Transform32 {
        fn from(transform: GlobalTransform32) -> Self {
            Self {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }

    impl Transform32 {
        /// Multiplies `self` with `transform` component by component, returning the
        /// resulting [`Transform`]
        #[inline]
        pub fn mul_transform(&self, transform: Transform32) -> Self {
            let translation = self.mul_vec3(transform.translation);
            let rotation = self.rotation * transform.rotation;
            let scale = self.scale * transform.scale;
            Self {
                translation,
                rotation,
                scale,
            }
        }
    }

    impl F32Convert for Transform32 {
        type F32Ver = Self;

        fn f32(&self) -> Self::F32Ver {
            *self
        }
    }
    impl F64Convert for Transform32 {
        type F64Ver = super::Transform64;

        fn f64(&self) -> Self::F64Ver {
            super::Transform64 {
                translation: self.translation.f64(),
                rotation: self.rotation.f64(),
                scale: self.scale.f64(),
            }
        }
    }

    /// Describe the position of an entity relative to the reference frame.
    ///
    /// * To place or move an entity, you should set its [`Transform`].
    /// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
    /// * To get the global position of an entity, you should get its [`GlobalTransform`].
    ///
    /// ## [`Transform`] and [`GlobalTransform`]
    ///
    /// [`Transform`] is the position of an entity relative to its parent position, or the reference
    /// frame if it doesn't have a [`Parent`](super::Parent).
    ///
    /// [`GlobalTransform`] is the position of an entity relative to the reference frame.
    ///
    /// [`GlobalTransform`] is updated from [`Transform`] in the system
    /// [`transform_propagate_system`](crate::transform_propagate_system::transform_propagate_system).
    ///
    /// In pseudo code:
    /// ```ignore
    /// for entity in entities_without_parent:
    ///     set entity.global_transform to entity.transform
    ///     recursively:
    ///         set parent to current entity
    ///         for child in parent.children:
    ///             set child.global_transform to parent.global_transform * child.transform
    /// ```
    ///
    /// This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
    /// update the[`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
    /// before the [`GlobalTransform`] is updated.
    #[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct GlobalTransform32 {
        pub translation: Vec3,
        pub rotation: Quat,
        pub scale: Vec3,
    }

    impl_transform!(GlobalTransform32, f32, Vec3, Quat, Mat3, Mat4);

    impl GlobalTransform32 {
        /// Multiplies `self` with `transform` component by component, returning the
        /// resulting [`GlobalTransform`]
        #[inline]
        pub fn mul_transform(&self, transform: Transform32) -> GlobalTransform32 {
            let translation = self.mul_vec3(transform.translation);
            let rotation = self.rotation * transform.rotation;
            let scale = self.scale * transform.scale;
            GlobalTransform32 {
                translation,
                rotation,
                scale,
            }
        }
    }

    impl From<Transform32> for GlobalTransform32 {
        fn from(transform: Transform32) -> Self {
            Self {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }

    impl Mul<Transform32> for GlobalTransform32 {
        type Output = GlobalTransform32;

        #[inline]
        fn mul(self, transform: Transform32) -> Self::Output {
            self.mul_transform(transform)
        }
    }

    impl F32Convert for GlobalTransform32 {
        type F32Ver = Self;

        fn f32(&self) -> Self::F32Ver {
            *self
        }
    }
    impl F64Convert for GlobalTransform32 {
        type F64Ver = super::GlobalTransform64;

        fn f64(&self) -> Self::F64Ver {
            super::GlobalTransform64 {
                translation: self.translation.f64(),
                rotation: self.rotation.f64(),
                scale: self.scale.f64(),
            }
        }
    }
}

mod xform_64 {
    use bevy_ecs::{prelude::Component, reflect::ReflectComponent};
    use bevy_math::{DMat3, DMat4, DQuat, DVec3, F32Convert, F64Convert};
    use bevy_reflect::Reflect;
    use std::ops::Mul;

    /// Describe the position of an entity. If the entity has a parent, the position is relative
    /// to its parent position.
    ///
    /// * To place or move an entity, you should set its [`Transform`].
    /// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
    /// * To get the global position of an entity, you should get its [`GlobalTransform`].
    ///
    /// ## [`Transform`] and [`GlobalTransform`]
    ///
    /// [`Transform`] is the position of an entity relative to its parent position, or the reference
    /// frame if it doesn't have a [`Parent`](super::Parent).
    ///
    /// [`GlobalTransform`] is the position of an entity relative to the reference frame.
    ///
    /// [`GlobalTransform`] is updated from [`Transform`] in the system
    /// [`transform_propagate_system`](crate::transform_propagate_system::transform_propagate_system).
    ///
    /// In pseudo code:
    /// ```ignore
    /// for entity in entities_without_parent:
    ///     set entity.global_transform to entity.transform
    ///     recursively:
    ///         set parent to current entity
    ///         for child in parent.children:
    ///             set child.global_transform to parent.global_transform * child.transform
    /// ```
    ///
    /// This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
    /// update the[`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
    /// before the [`GlobalTransform`] is updated.
    #[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct Transform64 {
        /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
        pub translation: DVec3,
        /// Rotation of the entity.
        pub rotation: DQuat,
        /// Scale of the entity.
        pub scale: DVec3,
    }

    impl_transform!(Transform64, f64, DVec3, DQuat, DMat3, DMat4);

    impl From<GlobalTransform64> for Transform64 {
        fn from(transform: GlobalTransform64) -> Self {
            Self {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }

    impl Transform64 {
        /// Multiplies `self` with `transform` component by component, returning the
        /// resulting [`Transform`]
        #[inline]
        pub fn mul_transform(&self, transform: Transform64) -> Self {
            let translation = self.mul_vec3(transform.translation);
            let rotation = self.rotation * transform.rotation;
            let scale = self.scale * transform.scale;
            Self {
                translation,
                rotation,
                scale,
            }
        }
    }

    /// Describe the position of an entity relative to the reference frame.
    ///
    /// * To place or move an entity, you should set its [`Transform`].
    /// * To be displayed, an entity must have both a [`Transform`] and a [`GlobalTransform`].
    /// * To get the global position of an entity, you should get its [`GlobalTransform`].
    ///
    /// ## [`Transform`] and [`GlobalTransform`]
    ///
    /// [`Transform`] is the position of an entity relative to its parent position, or the reference
    /// frame if it doesn't have a [`Parent`](super::Parent).
    ///
    /// [`GlobalTransform`] is the position of an entity relative to the reference frame.
    ///
    /// [`GlobalTransform`] is updated from [`Transform`] in the system
    /// [`transform_propagate_system`](crate::transform_propagate_system::transform_propagate_system).
    ///
    /// In pseudo code:
    /// ```ignore
    /// for entity in entities_without_parent:
    ///     set entity.global_transform to entity.transform
    ///     recursively:
    ///         set parent to current entity
    ///         for child in parent.children:
    ///             set child.global_transform to parent.global_transform * child.transform
    /// ```
    ///
    /// This system runs in stage [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate). If you
    /// update the[`Transform`] of an entity in this stage or after, you will notice a 1 frame lag
    /// before the [`GlobalTransform`] is updated.
    #[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct GlobalTransform64 {
        pub translation: DVec3,
        pub rotation: DQuat,
        pub scale: DVec3,
    }

    impl_transform!(GlobalTransform64, f64, DVec3, DQuat, DMat3, DMat4);

    impl GlobalTransform64 {
        /// Multiplies `self` with `transform` component by component, returning the
        /// resulting [`GlobalTransform`]
        #[inline]
        pub fn mul_transform(&self, transform: Transform64) -> GlobalTransform64 {
            let translation = self.mul_vec3(transform.translation);
            let rotation = self.rotation * transform.rotation;
            let scale = self.scale * transform.scale;
            GlobalTransform64 {
                translation,
                rotation,
                scale,
            }
        }
    }

    impl From<Transform64> for GlobalTransform64 {
        fn from(transform: Transform64) -> Self {
            Self {
                translation: transform.translation,
                rotation: transform.rotation,
                scale: transform.scale,
            }
        }
    }

    impl Mul<Transform64> for GlobalTransform64 {
        type Output = GlobalTransform64;

        #[inline]
        fn mul(self, transform: Transform64) -> Self::Output {
            self.mul_transform(transform)
        }
    }

    impl F64Convert for GlobalTransform64 {
        type F64Ver = Self;

        #[inline]
        fn f64(&self) -> Self::F64Ver {
            *self
        }
    }
    impl F32Convert for GlobalTransform64 {
        type F32Ver = super::GlobalTransform32;

        fn f32(&self) -> Self::F32Ver {
            super::GlobalTransform32 {
                translation: self.translation.f32(),
                rotation: self.rotation.f32(),
                scale: self.scale.f32(),
            }
        }
    }
}
