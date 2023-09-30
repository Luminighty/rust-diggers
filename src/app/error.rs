#[derive(Debug)]
pub enum AppError {
  PixelsError(pixels::Error)
}

impl From<pixels::Error> for AppError {
  fn from(value: pixels::Error) -> Self {
    AppError::PixelsError(value)
  }
}
