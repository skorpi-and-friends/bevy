use crate::*;

pub use defualt_tform_types::*;

#[cfg(not(feature = "xform_64"))]
mod defualt_tform_types {
    use glam::*;

    pub mod real {
        pub use std::f32::*;
    }

    pub type TReal = f32;
    pub type TVec3 = Vec3;
    pub type TQuat = Quat;
    pub type TVec2 = Vec2;
    pub type TMat3 = Mat3;
    pub type TMat4 = Mat4;

    // TODO: mull more over this
    impl<T, F32, F64> super::DefaultPrecisionConvert for T
    where
        T: super::F32Convert<F32Ver = F32> + super::F64Convert<F64Ver = F64>,
    {
        type DefaultVer = F32;

        fn default_precision(&self) -> Self::DefaultVer {
            self.f32()
        }
    }
}

#[cfg(feature = "xform_64")]
mod defualt_tform_types {
    use glam::*;

    pub mod real {
        pub use std::f64::*;
    }

    pub type TReal = f64;
    pub type TVec3 = DVec3;
    pub type TQuat = DQuat;
    pub type TVec2 = DVec2;
    pub type TMat3 = DMat3;
    pub type TMat4 = DMat4;

    // TODO: mull more over this
    impl<T, F32, F64> super::DefaultPrecisionConvert for T
    where
        T: super::F32Convert<F32Ver = F32> + super::F64Convert<F64Ver = F64>,
    {
        type DefaultVer = F64;

        fn default_precision(&self) -> Self::DefaultVer {
            self.f64()
        }
    }
}

pub trait F32Convert {
    type F32Ver;
    fn f32(&self) -> Self::F32Ver;
}

pub trait F64Convert {
    type F64Ver;
    fn f64(&self) -> Self::F64Ver;
}

pub trait DefaultPrecisionConvert {
    type DefaultVer;

    fn default_precision(&self) -> Self::DefaultVer;
}

// this only works if the type implements Copy
macro_rules! impl_f32_conv_self {
    ($t:ident) => {
        impl F32Convert for $t {
            type F32Ver = $t;

            #[inline]
            fn f32(&self) -> Self::F32Ver {
                *self
            }
        }
    };
}
impl_f32_conv_self!(f32);
impl_f32_conv_self!(Vec3);
impl_f32_conv_self!(Vec2);
impl_f32_conv_self!(Vec4);
impl_f32_conv_self!(Mat2);
impl_f32_conv_self!(Mat3);
impl_f32_conv_self!(Mat4);
impl_f32_conv_self!(Quat);
impl_f32_conv_self!(Vec3A);

// this only works for the glam types which have `as_f32` methods.
macro_rules! impl_f32_conv_glam {
    ($t:ident, $f32_t:ident) => {
        impl F32Convert for $t {
            type F32Ver = $f32_t;

            #[inline]
            fn f32(&self) -> Self::F32Ver {
                self.as_f32()
            }
        }
    };
}

impl_f32_conv_glam!(DVec3, Vec3);
impl_f32_conv_glam!(DVec2, Vec2);
impl_f32_conv_glam!(DVec4, Vec4);
impl_f32_conv_glam!(DMat2, Mat2);
impl_f32_conv_glam!(DMat3, Mat3);
impl_f32_conv_glam!(DMat4, Mat4);
impl_f32_conv_glam!(DQuat, Quat);

impl F32Convert for f64 {
    type F32Ver = f32;

    fn f32(&self) -> Self::F32Ver {
        *self as f32
    }
}

// this only works if the type implements Copy
macro_rules! impl_f64_conv_self {
    ($t:ident) => {
        impl F64Convert for $t {
            type F64Ver = $t;

            #[inline]
            fn f64(&self) -> Self::F64Ver {
                *self
            }
        }
    };
}
impl_f64_conv_self!(f64);
impl_f64_conv_self!(DVec3);
impl_f64_conv_self!(DVec2);
impl_f64_conv_self!(DVec4);
impl_f64_conv_self!(DMat2);
impl_f64_conv_self!(DMat3);
impl_f64_conv_self!(DMat4);
impl_f64_conv_self!(DQuat);

// this only works for the glam types which have `as_f64` methods.
macro_rules! impl_f64_conv_glam {
    ($t:ident, $f64_t:ident) => {
        impl F64Convert for $t {
            type F64Ver = $f64_t;

            #[inline]
            fn f64(&self) -> Self::F64Ver {
                self.as_f64()
            }
        }
    };
}

impl_f64_conv_glam!(Vec3, DVec3);
impl_f64_conv_glam!(Vec2, DVec2);
impl_f64_conv_glam!(Vec4, DVec4);
impl_f64_conv_glam!(Mat2, DMat2);
impl_f64_conv_glam!(Mat3, DMat3);
impl_f64_conv_glam!(Mat4, DMat4);
impl_f64_conv_glam!(Quat, DQuat);

impl F64Convert for f32 {
    type F64Ver = f64;

    #[inline]
    fn f64(&self) -> Self::F64Ver {
        *self as f64
    }
}
