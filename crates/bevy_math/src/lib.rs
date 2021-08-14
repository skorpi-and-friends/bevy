mod face_toward;
mod geometry;
mod precision;

pub use face_toward::*;
pub use geometry::*;
pub use glam::*;
pub use precision::*;

pub mod prelude {
    pub use super::precision::*;

    #[doc(hidden)]
    pub use crate::{
        BVec2, BVec3, BVec4, EulerRot, FaceToward, IVec2, IVec3, IVec4, Mat3, Mat4, Quat, Rect,
        Size, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
    };
}
