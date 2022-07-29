use gl::types::*;
use sdl2::{video::GLProfile, VideoSubsystem};
use std::mem::size_of_val;

#[cfg(debug_assertions)]
use std::ffi::{c_void, CStr};

pub(super) fn cfg_gl_attr(video_subsys: &VideoSubsystem) {
  let gl_attr = video_subsys.gl_attr();
  gl_attr.set_red_size(8);
  gl_attr.set_green_size(8);
  gl_attr.set_blue_size(8);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_version(4, 6);

  #[cfg(debug_assertions)]
  gl_attr.set_context_flags().debug().set();

  #[cfg(not(debug_assertions))]
  gl_attr.set_context_no_error(true);
}

#[cfg(debug_assertions)]
pub(super) extern "system" fn on_debug_message(
  source: GLenum,
  type_: GLenum,
  id: GLuint,
  severity: GLenum,
  length: GLsizei,
  message: *const GLchar,
  _user_param: *mut c_void,
) {
  let message = &unsafe { CStr::from_ptr(message).to_str().unwrap() }[0..length as _];
  match severity {
    gl::DEBUG_SEVERITY_HIGH => {
      eprintln!(
        "[ERROR] [ID={id}] {} from {}: {message}",
        match type_ {
          gl::DEBUG_TYPE_ERROR => "Error",
          gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated behavior",
          gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined behavior",
          gl::DEBUG_TYPE_PORTABILITY => "Portability",
          gl::DEBUG_TYPE_PERFORMANCE => "Performance",
          gl::DEBUG_TYPE_MARKER => "Marker",
          gl::DEBUG_TYPE_PUSH_GROUP => "Push group",
          gl::DEBUG_TYPE_POP_GROUP => "Pop group",
          gl::DEBUG_TYPE_OTHER => "Unknown",
          _ => unreachable!(),
        },
        match source {
          gl::DEBUG_SOURCE_API => "API",
          gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
          gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
          gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
          gl::DEBUG_SOURCE_APPLICATION => "application",
          gl::DEBUG_SOURCE_OTHER => "unknown",
          _ => unreachable!(),
        },
      );
    }
    gl::DEBUG_SEVERITY_MEDIUM | gl::DEBUG_SEVERITY_LOW => {
      println!(
        "[ WARN] [ID={id}] {} from {}: {message}",
        match type_ {
          gl::DEBUG_TYPE_ERROR => "Error",
          gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated behavior",
          gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined behavior",
          gl::DEBUG_TYPE_PORTABILITY => "Portability",
          gl::DEBUG_TYPE_PERFORMANCE => "Performance",
          gl::DEBUG_TYPE_MARKER => "Marker",
          gl::DEBUG_TYPE_PUSH_GROUP => "Push group",
          gl::DEBUG_TYPE_POP_GROUP => "Pop group",
          gl::DEBUG_TYPE_OTHER => "Unknown",
          _ => unreachable!(),
        },
        match source {
          gl::DEBUG_SOURCE_API => "API",
          gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
          gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
          gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
          gl::DEBUG_SOURCE_APPLICATION => "application",
          gl::DEBUG_SOURCE_OTHER => "unknown",
          _ => unreachable!(),
        },
      );
    }
    gl::DEBUG_SEVERITY_NOTIFICATION => {
      println!(
        "[ INFO] [ID={id}] {} from {}: {message}",
        match type_ {
          gl::DEBUG_TYPE_ERROR => "Error",
          gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated behavior",
          gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined behavior",
          gl::DEBUG_TYPE_PORTABILITY => "Portability",
          gl::DEBUG_TYPE_PERFORMANCE => "Performance",
          gl::DEBUG_TYPE_MARKER => "Marker",
          gl::DEBUG_TYPE_PUSH_GROUP => "Push group",
          gl::DEBUG_TYPE_POP_GROUP => "Pop group",
          gl::DEBUG_TYPE_OTHER => "Unknown",
          _ => unreachable!(),
        },
        match source {
          gl::DEBUG_SOURCE_API => "API",
          gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
          gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
          gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
          gl::DEBUG_SOURCE_APPLICATION => "application",
          gl::DEBUG_SOURCE_OTHER => "unknown",
          _ => unreachable!(),
        },
      );
    }
    _ => unreachable!(),
  }
}

pub(super) fn new_gl_program(vert_shader_code: &[u8], frag_shader_code: &[u8]) -> GLuint {
  unsafe {
    let program = gl::CreateProgram();
    let vert_shader = new_gl_shader(gl::VERTEX_SHADER, vert_shader_code);
    gl::AttachShader(program, vert_shader);
    let frag_shader = new_gl_shader(gl::FRAGMENT_SHADER, frag_shader_code);
    gl::AttachShader(program, frag_shader);
    gl::LinkProgram(program);
    gl::DeleteShader(vert_shader);
    gl::DeleteShader(frag_shader);
    program
  }
}

fn new_gl_shader(shader_type: GLenum, shader_code: &[u8]) -> GLuint {
  unsafe {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
      shader,
      1,
      &shader_code as *const _ as *const *const _,
      &size_of_val(shader_code) as *const _ as *const _,
    );
    gl::CompileShader(shader);
    shader
  }
}
