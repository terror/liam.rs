use super::*;

#[derive(Clone, Debug)]
pub(crate) enum Status {
  Expired(String),
  Ok,
}

impl From<StatusCode> for Status {
  fn from(status: StatusCode) -> Self {
    if status.is_success() || status.is_redirection() {
      Self::Ok
    } else {
      Self::Expired(format!("status {status}"))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn status_from_http_status_code() {
    let cases = [
      (StatusCode::OK, Status::Ok),
      (StatusCode::MOVED_PERMANENTLY, Status::Ok),
      (
        StatusCode::BAD_REQUEST,
        Status::Expired("status 400".into()),
      ),
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Status::Expired("status 500".into()),
      ),
    ];

    for (code, expected) in cases {
      match (Status::from(code), expected) {
        (Status::Ok, Status::Ok) => {}
        (Status::Expired(actual), Status::Expired(expected)) => {
          assert!(
            actual.contains(&expected),
            "expected `{actual}` to contain `{expected}`"
          );
        }
        (actual, expected) => {
          panic!("expected {expected:?}, got {actual:?}");
        }
      }
    }
  }
}
