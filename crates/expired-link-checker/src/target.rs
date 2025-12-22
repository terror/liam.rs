use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Target {
  Fragment,
  Invalid(String),
  Local(PathBuf),
  Remote(Url),
  Skipped,
}
