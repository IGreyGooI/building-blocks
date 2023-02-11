use bevy_reflect::{impl_from_reflect_value, impl_reflect_value, ReflectSerialize, ReflectDeserialize, std_traits::ReflectDefault};

use crate::Point3i;

impl_reflect_value!(Point3i(Debug, PartialEq, Serialize, Deserialize, Default));
impl_from_reflect_value!(Point3i);
