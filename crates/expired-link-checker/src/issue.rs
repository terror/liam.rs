use super::*;

#[derive(Clone, Debug)]
pub(crate) struct Issue {
  pub(crate) destination: String,
  pub(crate) file: PathBuf,
  pub(crate) reason: String,
}

impl Display for Issue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}: {} ({})",
      self.file.display(),
      self.destination,
      self.reason
    )
  }
}
