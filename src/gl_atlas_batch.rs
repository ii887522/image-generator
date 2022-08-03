use crate::{
  atlas_instance_flow::AtlasInstanceFlow,
  consts::{ATLAS_GAP, ATLAS_INSTANCE_SIZE, INSTANCE_CAP},
  gl_batch::{self, GLBatch},
  rect::Rect,
};

use async_std::task;
use futures::future;
use gl::types::*;
use iron_ingot::{FVec2, UBound, UVec2};
use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
pub(super) struct Arg<F: Fn(), G: Fn()> {
  pub size: UVec2,
  pub on_dirty: F,
  pub on_cleaned: G,
}

#[derive(Debug)]
pub(super) struct GLAtlasBatch {
  proto: GLBatch,
  size: UVec2,
  src: Vec<u8>,
  instance_count: usize,
  bounding_rects: Vec<Rect>,
  current_bounding_rect_position: UVec2,
  max_bounding_rect_h: u32,
  has_reset: bool,
}

impl GLAtlasBatch {
  pub(super) fn new(arg: Arg<impl Fn() + 'static, impl Fn() + 'static>) -> Self {
    let Arg {
      size,
      on_dirty,
      on_cleaned,
    } = arg;
    Self {
      proto: GLBatch::new(gl_batch::Arg {
        instance_size: ATLAS_INSTANCE_SIZE,
        on_dirty,
        on_cleaned,
      }),
      size,
      src: Vec::with_capacity(INSTANCE_CAP * ATLAS_INSTANCE_SIZE),
      instance_count: 0,
      bounding_rects: Vec::with_capacity(INSTANCE_CAP),
      current_bounding_rect_position: UVec2::new((ATLAS_GAP >> 1, ATLAS_GAP >> 1)),
      max_bounding_rect_h: 0,
      has_reset: false,
    }
  }

  pub(super) fn init(&mut self) {
    self.proto.init();
  }

  fn reset(&mut self) {
    self.proto.reset();
    self.src.clear();
    self.instance_count = 0;
    self.bounding_rects.clear();
    self.current_bounding_rect_position = UVec2::new((ATLAS_GAP >> 1, ATLAS_GAP >> 1));
    self.max_bounding_rect_h = 0;
  }

  pub(super) fn show(&mut self, vao: GLuint) {
    if self.has_reset {
      self
        .bounding_rects
        .par_sort_unstable_by(|a, b| b.size.get_y().cmp(&a.size.get_y()));
      self.fill_bounding_rects();
      self.has_reset = false;
    }
    self.proto.show(vao, self.instance_count, self.flush());
  }

  fn fill_bounding_rects(&mut self) {
    for id in 0..self.bounding_rects.len() {
      self.fill_bounding_rect(id);
    }
    self.instance_count = self.bounding_rects.len();
  }

  fn fill_bounding_rect(&mut self, id: usize) {
    if self.current_bounding_rect_position.get_x()
      + self.bounding_rects[id].size.get_x()
      + (ATLAS_GAP >> 1)
      > self.size.get_x()
    {
      self.current_bounding_rect_position = UVec2::new((
        ATLAS_GAP >> 1,
        self.current_bounding_rect_position.get_y() + self.max_bounding_rect_h + ATLAS_GAP,
      ));
      self.max_bounding_rect_h = 0;
    }
    self
      .src
      .extend_from_slice(&(self.current_bounding_rect_position.get_x() as f32).to_ne_bytes());
    self
      .src
      .extend_from_slice(&(self.current_bounding_rect_position.get_y() as f32).to_ne_bytes());
    self
      .src
      .extend_from_slice(&(self.bounding_rects[id].size.get_x() as f32).to_ne_bytes());
    self
      .src
      .extend_from_slice(&(self.bounding_rects[id].size.get_y() as f32).to_ne_bytes());
    self
      .src
      .extend_from_slice(&self.bounding_rects[id].color.get_x().to_ne_bytes());
    self
      .src
      .extend_from_slice(&self.bounding_rects[id].color.get_y().to_ne_bytes());
    self
      .src
      .extend_from_slice(&self.bounding_rects[id].color.get_z().to_ne_bytes());
    self
      .src
      .extend_from_slice(&self.bounding_rects[id].color.get_w().to_ne_bytes());
    let position = FVec2::from(self.current_bounding_rect_position);
    self.current_bounding_rect_position +=
      UVec2::new((self.bounding_rects[id].size.get_x() + ATLAS_GAP, 0));
    self.max_bounding_rect_h = self
      .max_bounding_rect_h
      .max(self.bounding_rects[id].size.get_y());
    self.bounding_rects[id].position = position;
  }

  async fn flush(&self) {
    unsafe {
      let cpu_count = num_cpus::get();
      let mut tasks = Vec::with_capacity(cpu_count + 1);
      let sub_len = self.instance_count / cpu_count * ATLAS_INSTANCE_SIZE;
      let instance_flow = AtlasInstanceFlow::new(
        &self.src,
        self.proto.get_dst().add(self.proto.get_current_dst_id()),
      );
      if sub_len != 0 {
        for i in 0..cpu_count {
          tasks.push(task::spawn(
            instance_flow.flush(UBound::new(((i * sub_len) as _, ((i + 1) * sub_len) as _))),
          ));
        }
      }
      let flushed_instances_len = cpu_count * sub_len;
      let instances_left_len = self.instance_count % cpu_count * ATLAS_INSTANCE_SIZE;
      if instances_left_len != 0 {
        tasks.push(task::spawn(instance_flow.flush(UBound::new((
          flushed_instances_len as _,
          (flushed_instances_len + instances_left_len) as _,
        )))));
      }
      future::join_all(tasks).await;
    }
  }
}

impl gl_batch::Specific for GLAtlasBatch {
  fn fill_rect(&mut self, rect: Rect) {
    let Rect { size, color, .. } = rect;
    debug_assert!(
      size.get_x() >= 1,
      "Width must be greater than or equal to 1!"
    );
    debug_assert!(
      size.get_y() >= 1,
      "Height must be greater than or equal to 1!"
    );
    debug_assert!(
      (0.0..=1.0).contains(&color.get_x()),
      "Red component must be between 0.0 and 1.0 inclusive!"
    );
    debug_assert!(
      (0.0..=1.0).contains(&color.get_y()),
      "Green component must be between 0.0 and 1.0 inclusive!"
    );
    debug_assert!(
      (0.0..=1.0).contains(&color.get_z()),
      "Blue component must be between 0.0 and 1.0 inclusive!"
    );
    debug_assert!(
      (0.0..=1.0).contains(&color.get_w()),
      "Alpha component must be between 0.0 and 1.0 inclusive!"
    );
    if color.get_w() != 0.0 {
      if !self.has_reset {
        self.reset();
        self.has_reset = true;
      }
      self.bounding_rects.push(Rect {
        position: FVec2::new(()),
        size,
        color,
      });
    }
  }

  fn get_filled_rects(&self) -> &[Rect] {
    &self.bounding_rects
  }
}
