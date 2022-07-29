use crate::{gl_atlas_scene::GLAtlasScene, gl_ext::cfg_gl_attr, gl_scene::GLScene};
use iron_ingot::UVec2;
use std::fmt::{self, Debug, Formatter};

use sdl2::{
  event::Event,
  video::{GLContext, Window},
  Sdl, VideoSubsystem,
};

#[cfg(windows)]
use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

#[cfg(debug_assertions)]
use std::ptr;

#[cfg(debug_assertions)]
use crate::gl_ext::on_debug_message;

pub(super) struct GLApp<F: FnMut(&mut dyn GLScene)> {
  sdl: Sdl,
  video_subsys: VideoSubsystem,
  window: Window,
  _gl_ctx: GLContext,
  scene: Box<dyn GLScene>,
  on_render: F,
}

impl<F: FnMut(&mut dyn GLScene)> Debug for GLApp<F> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("GLApp")
      .field("video_subsys", &self.video_subsys)
      .field("scene", &self.scene)
      .finish_non_exhaustive()
  }
}

impl<F: FnMut(&mut dyn GLScene)> GLApp<F> {
  pub(super) fn new(title: &str, size: UVec2, on_render: F) -> Self {
    #[cfg(windows)]
    unsafe {
      SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    }

    let sdl = sdl2::init().unwrap();
    let video_subsys = sdl.video().unwrap();
    cfg_gl_attr(&video_subsys);
    let window = video_subsys
      .window(title, size.get_x(), size.get_y())
      .allow_highdpi()
      .opengl()
      .position_centered()
      .hidden()
      .build()
      .unwrap();
    let gl_ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

    #[cfg(debug_assertions)]
    unsafe {
      gl::DebugMessageCallback(Some(on_debug_message), ptr::null());
      gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    }

    video_subsys.gl_set_swap_interval(0).unwrap();
    let scene: Box<dyn GLScene> = Box::new(GLAtlasScene::new(size));
    unsafe {
      gl::ReleaseShaderCompiler();
    }
    Self {
      sdl,
      video_subsys,
      window,
      _gl_ctx: gl_ctx,
      scene,
      on_render,
    }
  }

  pub(super) fn start(&mut self) {
    loop {
      for event in self.sdl.event_pump().unwrap().poll_iter() {
        if let Event::Quit { .. } = event {
          return;
        }
      }
      (self.on_render)(&mut *self.scene);
      if !self.scene.is_running() {
        return;
      }
      self.scene.show(&self.window);
    }
  }
}
