use iron_ingot::UBound;
use std::ptr::copy_nonoverlapping;

#[derive(Copy, Clone, Debug)]
pub(crate) struct AtlasInstanceFlow {
  dst: *mut u8,
  src: *const u8,
}

unsafe impl Send for AtlasInstanceFlow {}

impl AtlasInstanceFlow {
  pub(crate) const fn new(src: &[u8], dst: *mut u8) -> Self {
    Self {
      dst,
      src: src.as_ptr(),
    }
  }

  pub(crate) async unsafe fn flush(self, bound: UBound) {
    copy_nonoverlapping(
      self.src.add(bound.get_min() as _),
      self.dst.add(bound.get_min() as _),
      (bound.get_max() - bound.get_min()) as _,
    );
  }
}
