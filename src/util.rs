use iron_ingot::UVec2;
use png::{BitDepth, ColorType, Compression, FilterType, ScaledFloat, SourceChromaticities};

use std::{
  fs::{create_dir, remove_dir_all, File},
  io::{self, BufWriter},
};

pub(super) fn ensure_empty_dir(dir_path: &str) {
  if let Err(err) = remove_dir_all(dir_path) {
    if err.kind() != io::ErrorKind::NotFound {
      panic!("{err:?}");
    }
  }
  create_dir(dir_path).unwrap();
}

pub(super) fn save_png_file(file_path: &str, size: UVec2, image: &[u8]) {
  let mut encoder = png::Encoder::new(
    BufWriter::new(File::create(file_path).unwrap()),
    size.get_x(),
    size.get_y(),
  );
  encoder.set_color(ColorType::Rgba);
  encoder.set_depth(BitDepth::Eight);
  encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
  encoder.set_source_gamma(ScaledFloat::from_scaled(45455));
  encoder.set_source_chromaticities(SourceChromaticities::new(
    (0.31270, 0.32900),
    (0.64000, 0.33000),
    (0.30000, 0.60000),
    (0.15000, 0.06000),
  ));
  encoder.set_compression(Compression::Rle);
  encoder.set_filter(FilterType::Paeth);
  encoder
    .write_header()
    .unwrap()
    .write_image_data(image)
    .unwrap();
}
