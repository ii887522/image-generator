use crate::gl_batch;
use sdl2::video::Window;
use std::fmt::Debug;

pub(super) trait GLScene: Debug {
  fn is_running(&self) -> bool;
  fn get_batch(&mut self) -> &mut dyn gl_batch::Specific;
  fn show(&mut self, window: &Window);
  fn exit(&mut self);
}
