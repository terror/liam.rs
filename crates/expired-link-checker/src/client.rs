use super::*;

#[derive(Debug)]
pub(crate) struct Client {
  client: reqwest::blocking::Client,
  remote_cache: HashMap<String, Status>,
}

impl Client {
  fn check_get(&self, url: &Url) -> Status {
    match self.client.get(url.clone()).send() {
      Ok(response) => Status::from(response.status()),
      Err(error) => Status::Expired(error.to_string()),
    }
  }

  pub(crate) fn check_remote(&mut self, url: &Url) -> Status {
    let mut normalized = url.clone();

    normalized.set_fragment(None);

    let key = normalized.as_str().to_string();

    if let Some(status) = self.remote_cache.get(&key) {
      return status.clone();
    }

    let status = self.check_remote_once(&normalized);

    self.remote_cache.insert(key, status.clone());

    status
  }

  fn check_remote_once(&self, url: &Url) -> Status {
    match self.client.head(url.clone()).send() {
      Ok(response) => {
        let status = response.status();

        if status == StatusCode::METHOD_NOT_ALLOWED
          || status == StatusCode::FORBIDDEN
        {
          return self.check_get(url);
        }

        Status::from(status)
      }
      Err(error) => Status::Expired(error.to_string()),
    }
  }

  pub(crate) fn new(timeout_seconds: u64) -> Result<Self> {
    Ok(Self {
      client: reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(timeout_seconds))
        .redirect(Policy::limited(10))
        .user_agent(format!("{}/0.0.0", env!("CARGO_PKG_NAME")))
        .build()
        .context("building http client")?,
      remote_cache: HashMap::new(),
    })
  }
}
