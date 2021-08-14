use crate as bevy_reflect;
use crate::ReflectDeserialize;
use bevy_reflect_derive::{impl_from_reflect_value, impl_reflect_value};
use glam::{
    DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat3, Mat4, Quat, UVec2, UVec3,
    UVec4, Vec2, Vec3, Vec4,
};

impl_reflect_value!(IVec2(PartialEq, Serialize, Deserialize));
impl_reflect_value!(IVec3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(IVec4(PartialEq, Serialize, Deserialize));
impl_reflect_value!(UVec2(PartialEq, Serialize, Deserialize));
impl_reflect_value!(UVec3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(UVec4(PartialEq, Serialize, Deserialize));

impl_reflect_value!(Vec2(PartialEq, Serialize, Deserialize));
impl_reflect_value!(Vec3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(Vec4(PartialEq, Serialize, Deserialize));
impl_reflect_value!(Mat3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(Mat4(PartialEq, Serialize, Deserialize));
impl_reflect_value!(Quat(PartialEq, Serialize, Deserialize));

impl_from_reflect_value!(IVec2);
impl_from_reflect_value!(IVec3);
impl_from_reflect_value!(IVec4);
impl_from_reflect_value!(UVec2);
impl_from_reflect_value!(UVec3);
impl_from_reflect_value!(UVec4);
impl_from_reflect_value!(Vec2);
impl_from_reflect_value!(Vec3);
impl_from_reflect_value!(Vec4);
impl_from_reflect_value!(Mat3);
impl_from_reflect_value!(Mat4);
impl_from_reflect_value!(Quat);
impl_reflect_value!(DVec2(PartialEq, Serialize, Deserialize));
impl_reflect_value!(DVec3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(DVec4(PartialEq, Serialize, Deserialize));
impl_reflect_value!(DMat3(PartialEq, Serialize, Deserialize));
impl_reflect_value!(DMat4(PartialEq, Serialize, Deserialize));
impl_reflect_value!(DQuat(PartialEq, Serialize, Deserialize));
