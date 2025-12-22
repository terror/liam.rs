use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Status {
  Dead(String),
  Ok,
}

impl From<StatusCode> for Status {
  fn from(status: StatusCode) -> Self {
    if status.is_success() || status.is_redirection() {
      Self::Ok
    } else {
      Self::Dead(format!("status {status}"))
    }
  }
}
