use iron_ingot::{FVec2, FVec4, UVec2};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
pub(super) struct Rect {
  pub position: FVec2,
  pub size: UVec2,
  pub color: FVec4,
}

impl Default for Rect {
  fn default() -> Self {
    Self {
      position: FVec2::new((0.0, 0.0)),
      size: UVec2::new((1, 1)),
      color: FVec4::new((1.0, 1.0, 1.0, 1.0)),
    }
  }
}

impl PartialEq for Rect {
  fn eq(&self, other: &Self) -> bool {
    self.size == other.size
  }
}

impl Eq for Rect {}

impl Hash for Rect {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.size.hash(state);
  }
}
