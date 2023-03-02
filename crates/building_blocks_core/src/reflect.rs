use bevy_reflect::{
    impl_from_reflect_value, impl_reflect_value, std_traits::ReflectDefault, ReflectDeserialize,
    ReflectSerialize,
};

use crate::{Extent3i, Point3i};

impl_reflect_value!(Point3i(Debug, PartialEq, Serialize, Deserialize, Default));
impl_from_reflect_value!(Point3i);

impl_reflect_value!(Extent3i(Debug, PartialEq, Serialize, Deserialize));
impl_from_reflect_value!(Extent3i);
