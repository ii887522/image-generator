use std::mem::size_of;

pub(super) const INSTANCE_CAP: usize = 32768;
pub(super) const ATLAS_INSTANCE_SIZE: usize = 8 * size_of::<f32>();
pub(super) const INSTANCE_BUFFER_COUNT: usize = 3;
pub(super) const ATLAS_GAP: u32 = 2;
