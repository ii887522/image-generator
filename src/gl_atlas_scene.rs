use crate::{
  gl_atlas_batch::{self, GLAtlasBatch},
  gl_batch,
  gl_ext::new_gl_program,
  gl_scene::GLScene,
};

use gl::types::*;
use iron_ingot::UVec2;
use sdl2::video::Window;

use std::{
  cell::Cell,
  mem::{size_of, size_of_val},
  rc::Rc,
};

#[derive(Debug)]
pub(super) struct GLAtlasScene {
  vao: GLuint,
  vbo: GLuint,
  batch: GLAtlasBatch,
  program: GLuint,
  is_dirty: Rc<Cell<bool>>,
  is_running: bool,
}

impl GLAtlasScene {
  pub(super) fn new(size: UVec2) -> Self {
    let is_dirty = Rc::new(Cell::new(false));
    let mut this = Self {
      vao: 0,
      vbo: 0,
      batch: GLAtlasBatch::new(gl_atlas_batch::Arg {
        size,
        on_dirty: {
          let is_dirty = Rc::clone(&is_dirty);
          move || is_dirty.set(true)
        },
        on_cleaned: {
          let is_dirty = Rc::clone(&is_dirty);
          move || is_dirty.set(false)
        },
      }),
      program: 0,
      is_dirty,
      is_running: true,
    };
    this.init(size);
    this
  }

  fn init(&mut self, size: UVec2) {
    self.init_vao();
    self.init_vbo();
    self.batch.init();
    self.program = new_gl_program(
      include_bytes!("../res/atlas.vert"),
      include_bytes!("../res/atlas.frag"),
    );
    unsafe {
      gl::UseProgram(self.program);
      gl::ProgramUniform2f(self.program, 0, size.get_x() as _, size.get_y() as _);
    }
  }

  fn init_vao(&mut self) {
    unsafe {
      gl::CreateVertexArrays(1, &mut self.vao);
      gl::BindVertexArray(self.vao);

      // vPosition
      gl::VertexArrayAttribBinding(self.vao, 0, 0);
      gl::VertexArrayAttribFormat(self.vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
      gl::EnableVertexArrayAttrib(self.vao, 0);

      // vTranslation
      gl::VertexArrayAttribBinding(self.vao, 1, 1);
      gl::VertexArrayAttribFormat(self.vao, 1, 2, gl::FLOAT, gl::FALSE, 0);
      gl::EnableVertexArrayAttrib(self.vao, 1);

      // vScale
      gl::VertexArrayAttribBinding(self.vao, 2, 1);
      gl::VertexArrayAttribFormat(
        self.vao,
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        (2 * size_of::<f32>()) as _,
      );
      gl::EnableVertexArrayAttrib(self.vao, 2);

      // vColor
      gl::VertexArrayAttribBinding(self.vao, 3, 1);
      gl::VertexArrayAttribFormat(
        self.vao,
        3,
        4,
        gl::FLOAT,
        gl::FALSE,
        (4 * size_of::<f32>()) as _,
      );
      gl::EnableVertexArrayAttrib(self.vao, 3);

      gl::VertexArrayBindingDivisor(self.vao, 0, 0);
      gl::VertexArrayBindingDivisor(self.vao, 1, 1);
    }
  }

  fn init_vbo(&mut self) {
    unsafe {
      gl::CreateBuffers(1, &mut self.vbo);
      const VERTICES: &[f32] = &[
        // vPosition
        0.0f32, 0.0f32, // top left corner vertex
        0.0f32, 1.0f32, // bottom left corner vertex
        1.0f32, 1.0f32, // bottom right corner vertex
        1.0f32, 0.0f32, // top right corner vertex
      ];
      gl::NamedBufferData(
        self.vbo,
        size_of_val(VERTICES) as _,
        VERTICES as *const _ as *const _,
        gl::STATIC_DRAW,
      );
      gl::VertexArrayVertexBuffer(self.vao, 0, self.vbo, 0, (2 * size_of::<f32>()) as _);
    }
  }
}

impl GLScene for GLAtlasScene {
  fn is_running(&self) -> bool {
    self.is_running
  }

  fn get_batch(&mut self) -> &mut dyn gl_batch::Specific {
    &mut self.batch
  }

  fn show(&mut self, _window: &Window) {
    if self.is_dirty.get() {
      unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }
      self.batch.show(self.vao);

      // View the atlas (output) on the screen but reading pixels from the default framebuffer and output them as image
      // files won't work :(
      //
      // window.gl_swap_window();
    }
  }

  fn exit(&mut self) {
    self.is_running = false;
  }
}

impl Drop for GLAtlasScene {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.program);
      gl::DeleteBuffers(1, &self.vbo);
      gl::DeleteVertexArrays(1, &self.vao);
    }
  }
}
