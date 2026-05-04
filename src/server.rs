use super::*;

pub(crate) struct Server {
  port: u16,
}

impl Server {
  fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
      Some("css") => "text/css; charset=utf-8",
      Some("html") => "text/html; charset=utf-8",
      Some("ico") => "image/x-icon",
      Some("js") => "application/javascript; charset=utf-8",
      Some("png") => "image/png",
      Some("svg") => "image/svg+xml",
      Some("webmanifest") => "application/manifest+json",
      Some("xml") => "application/rss+xml; charset=utf-8",
      _ => "application/octet-stream",
    }
  }

  pub(crate) fn new(port: u16) -> Self {
    Self { port }
  }

  pub(crate) fn serve(self) -> Result {
    Generator::new().run()?;

    let live_reload = LiveReloadLayer::new();

    let reloader = live_reload.reloader();

    thread::spawn(move || {
      if let Err(error) = Watcher::new(reloader).watch() {
        eprintln!("error: {error}");
        process::exit(1);
      }
    });

    Runtime::new()?.block_on(self.serve_http(live_reload))
  }

  async fn serve_http(self, live_reload: LiveReloadLayer) -> Result {
    let app = Router::new()
      .fallback(get(Self::static_file))
      .layer(live_reload);

    let listener = TcpListener::bind(("127.0.0.1", self.port)).await?;

    println!("Serving http://127.0.0.1:{}", self.port);

    axum::serve(listener, app).await?;

    Ok(())
  }

  async fn static_file(OriginalUri(uri): OriginalUri) -> Response {
    let file = match Self::static_path(uri.path()) {
      Ok(file) => file,
      Err(error) => {
        return (StatusCode::BAD_REQUEST, error.to_string()).into_response();
      }
    };

    let body = match tokio::fs::read(&file).await {
      Ok(body) => body,
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
        return (StatusCode::NOT_FOUND, "not found").into_response();
      }
      Err(error) => {
        return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
          .into_response();
      }
    };

    ([(header::CONTENT_TYPE, Self::content_type(&file))], body).into_response()
  }

  fn static_path(path: &str) -> Result<PathBuf> {
    let path = path
      .split_once('?')
      .map_or(path, |(path, _query)| path)
      .trim_start_matches('/');

    ensure!(
      !path.split('/').any(|component| component == ".."),
      "invalid path"
    );

    let path = if path.is_empty() {
      PathBuf::from("docs/index.html")
    } else {
      Path::new("docs").join(path)
    };

    if path.is_dir() {
      Ok(path.join("index.html"))
    } else {
      Ok(path)
    }
  }
}
