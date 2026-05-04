use super::*;

pub(crate) struct Generate;

impl Generate {
  pub(crate) fn run() -> Result {
    Generator::new().build()
  }
}
