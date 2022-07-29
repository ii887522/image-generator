use crate::{
  consts::{INSTANCE_BUFFER_COUNT, INSTANCE_CAP},
  rect::Rect,
};

use async_std::task;
use futures::Future;
use gl::types::*;

use std::{
  cell::Cell,
  fmt::{self, Debug, Formatter},
  ptr,
};

pub(super) struct Arg<F: Fn(), G: Fn()> {
  pub instance_size: usize,
  pub on_dirty: F,
  pub on_cleaned: G,
}

impl<F: Fn(), G: Fn()> Debug for Arg<F, G> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("Arg")
      .field("instance_size", &self.instance_size)
      .finish_non_exhaustive()
  }
}

pub(super) struct GLBatch {
  ibo: GLuint,
  dst: *mut u8,
  current_dst_id: Cell<usize>,
  is_dirty: Cell<bool>,
  instance_size: usize,
  on_dirty: Box<dyn Fn()>,
  on_cleaned: Box<dyn Fn()>,
}

impl Debug for GLBatch {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("GLBatch")
      .field("ibo", &self.ibo)
      .field("dst", &self.dst)
      .field("current_dst_id", &self.current_dst_id)
      .field("is_dirty", &self.is_dirty)
      .field("instance_size", &self.instance_size)
      .finish_non_exhaustive()
  }
}

unsafe impl Send for GLBatch {}

impl GLBatch {
  pub(super) fn new(arg: Arg<impl Fn() + 'static, impl Fn() + 'static>) -> Self {
    let Arg {
      instance_size,
      on_dirty,
      on_cleaned,
    } = arg;
    Self {
      ibo: 0,
      dst: ptr::null_mut(),
      current_dst_id: Cell::new(0),
      is_dirty: Cell::new(false),
      instance_size,
      on_dirty: Box::new(on_dirty),
      on_cleaned: Box::new(on_cleaned),
    }
  }

  pub(super) const fn get_dst(&self) -> *mut u8 {
    self.dst
  }

  pub(super) fn get_current_dst_id(&self) -> usize {
    self.current_dst_id.get()
  }

  pub(super) fn init(&mut self) {
    unsafe {
      gl::CreateBuffers(1, &mut self.ibo);
      gl::NamedBufferStorage(
        self.ibo,
        (INSTANCE_CAP * self.instance_size * INSTANCE_BUFFER_COUNT) as _,
        ptr::null(),
        gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT,
      );
      self.dst = gl::MapNamedBufferRange(
        self.ibo,
        0,
        (INSTANCE_CAP * self.instance_size * INSTANCE_BUFFER_COUNT) as _,
        gl::MAP_WRITE_BIT
          | gl::MAP_PERSISTENT_BIT
          | gl::MAP_COHERENT_BIT
          | gl::MAP_INVALIDATE_RANGE_BIT
          | gl::MAP_UNSYNCHRONIZED_BIT,
      ) as *mut _;
    }
  }

  pub(super) fn reset(&mut self) {
    self.is_dirty.set(true);
    (*self.on_dirty)();
  }

  pub(crate) fn show(
    &self,
    vao: GLuint,
    instance_count: usize,
    flush_fut: impl Future<Output = ()>,
  ) {
    if self.is_dirty.get() {
      self.current_dst_id.set(
        (self.current_dst_id.get() + INSTANCE_CAP * self.instance_size)
          % (INSTANCE_CAP * self.instance_size * INSTANCE_BUFFER_COUNT),
      );
      task::block_on(flush_fut);
      self.is_dirty.set(false);
      (*self.on_cleaned)();
    }
    unsafe {
      gl::BindVertexArray(vao);
      gl::VertexArrayVertexBuffer(
        vao,
        1,
        self.ibo,
        self.current_dst_id.get() as _,
        self.instance_size as _,
      );
      gl::DrawArraysInstanced(gl::TRIANGLE_FAN, 0, 4, instance_count as _);
    }
  }
}

impl Drop for GLBatch {
  fn drop(&mut self) {
    unsafe {
      gl::UnmapNamedBuffer(self.ibo);
      gl::DeleteBuffers(1, &self.ibo);
    }
  }
}

pub(super) trait Specific: Debug {
  fn fill_rect(&mut self, rect: Rect);
  fn get_filled_rects(&self) -> &[Rect];
}
