#[cfg(windows)]
use winres::WindowsResource;

#[cfg(windows)]
fn main() {
  let mut res = WindowsResource::new();
  res.set_icon("res/icon.ico");
  res.compile().unwrap();
}

#[cfg(unix)]
fn main() {}
