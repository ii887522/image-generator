mod atlas_instance_flow;
mod consts;
mod gl_app;
mod gl_atlas_batch;
mod gl_atlas_scene;
mod gl_batch;
mod gl_ext;
mod gl_scene;
mod rect;
mod util;

use crate::gl_app::GLApp;
use iron_ingot::{FVec4, UVec2};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rect::Rect;
use util::{ensure_empty_dir, save_png_file};

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;

fn main() {
  let mut is_first_render = true;
  GLApp::new(
    "Image Generator",
    UVec2::new((WINDOW_WIDTH, WINDOW_HEIGHT)),
    |scene| {
      let batch = scene.get_batch();
      if is_first_render {
        draw_random_atlas(batch);
        is_first_render = false;
      } else {
        ensure_empty_dir("out");
        save_images(batch);
        scene.exit();
      }
    },
  )
  .start();
}

fn draw_random_atlas(batch: &mut dyn gl_batch::Specific) {
  let mut rng = thread_rng();
  const IMAGE_COUNT: usize = 48;
  const MIN_IMAGE_SIZE: u32 = 8;
  const MAX_IMAGE_SIZE: u32 = 256;
  const MIN_COLOR: f32 = 0.25;
  const MAX_COLOR: f32 = 1.0;
  for _ in 0..IMAGE_COUNT {
    batch.fill_rect(Rect {
      size: UVec2::new((
        rng.gen_range(MIN_IMAGE_SIZE..=MAX_IMAGE_SIZE),
        rng.gen_range(MIN_IMAGE_SIZE..=MAX_IMAGE_SIZE),
      )),
      color: FVec4::new((
        rng.gen_range(MIN_COLOR..=MAX_COLOR),
        rng.gen_range(MIN_COLOR..=MAX_COLOR),
        rng.gen_range(MIN_COLOR..=MAX_COLOR),
        rng.gen_range(MIN_COLOR..=MAX_COLOR),
      )),
      ..Default::default()
    });
  }
}

fn save_images(batch: &dyn gl_batch::Specific) {
  for (id, rect) in batch.get_filled_rects().iter().enumerate() {
    save_image(id, rect);
  }
}

fn save_image(id: usize, rect: &Rect) {
  const CHANNEL_COUNT: u32 = 4;
  let mut image = Vec::<u8>::new();
  image.resize(
    (rect.size.get_x() * rect.size.get_y() * CHANNEL_COUNT) as _,
    0,
  );
  unsafe {
    gl::ReadnPixels(
      rect.position.get_x() as _,
      (WINDOW_HEIGHT - rect.position.get_y() as u32 - rect.size.get_y()) as _,
      rect.size.get_x() as _,
      rect.size.get_y() as _,
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      image.len() as _,
      image.as_mut_ptr() as *mut _,
    );
  }
  save_png_file(
    &format!("out/{id}.png"),
    rect.size,
    &image
      .par_rchunks_exact((rect.size.get_x() * CHANNEL_COUNT) as _)
      .flatten()
      .map(|&data| data)
      .collect::<Vec<_>>(),
  );
}
