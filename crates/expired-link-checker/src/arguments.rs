use super::*;

#[derive(Debug)]
enum CheckResult {
  Fragment,
  Issue(Issue),
  Ok,
  Skipped,
}

#[derive(Debug, Parser)]
#[command(
  name = env!("CARGO_PKG_NAME"),
  about = "Check markdown files for expired links.",
)]
pub(crate) struct Arguments {
  /// One or more markdown files to check.
  #[arg(value_name = "FILE", num_args = 0..)]
  files: Vec<PathBuf>,
  /// Process all .md files recursively from the current directory.
  #[arg(short = 'r', long = "recursive", conflicts_with = "files")]
  recursive: bool,
  /// Base directory for absolute links (default: current directory).
  #[arg(long = "root", value_name = "DIR", default_value = ".")]
  root: PathBuf,
  /// Timeout for HTTP requests in seconds.
  #[arg(long = "timeout-seconds", default_value_t = DEFAULT_TIMEOUT_SECONDS)]
  timeout_seconds: u64,
}

impl Arguments {
  fn check_destination(
    &self,
    destination: &str,
    file_path: &Path,
    client: &mut Client,
  ) -> CheckResult {
    let normalized = normalize_destination(destination);

    match self.classify_destination(&normalized, file_path) {
      Target::Fragment => {
        eprintln!("{COLOR_CYAN}FRAG{COLOR_RESET} {normalized}");
        CheckResult::Fragment
      }
      Target::Skipped => {
        eprintln!(
          "{COLOR_YELLOW}SKIP{COLOR_RESET} {normalized} (skipped scheme)"
        );

        CheckResult::Skipped
      }
      Target::Invalid(reason) => {
        eprintln!(
          "{COLOR_RED}BAD{COLOR_RESET} {normalized} (invalid: {reason})"
        );

        CheckResult::Issue(Issue {
          destination: normalized,
          file: file_path.to_path_buf(),
          reason,
        })
      }
      Target::Local(path) => {
        if path.exists() {
          eprintln!("{COLOR_GREEN}OK{COLOR_RESET} {normalized}");
          CheckResult::Ok
        } else {
          let reason = format!("missing file {}", path.display());

          eprintln!("{COLOR_RED}EXPIRED{COLOR_RESET} {normalized} ({reason})");

          CheckResult::Issue(Issue {
            destination: normalized,
            file: file_path.to_path_buf(),
            reason,
          })
        }
      }
      Target::Remote(url) => match client.check_remote(&url) {
        Status::Ok => {
          eprintln!("{COLOR_GREEN}OK{COLOR_RESET} {url}");
          CheckResult::Ok
        }
        Status::Expired(reason) => {
          eprintln!(
            "{COLOR_RED}EXPIRED{COLOR_RESET} {url} (expired: {reason})"
          );

          CheckResult::Issue(Issue {
            destination: url.to_string(),
            file: file_path.to_path_buf(),
            reason,
          })
        }
      },
    }
  }

  fn check_file(&self, path: &Path, client: &mut Client) -> Result<Vec<Issue>> {
    let input = fs::read_to_string(path)
      .with_context(|| format!("reading {}", path.display()))?;

    let destinations = MarkdownParser::new(&input)
      .filter_map(|event| match event {
        Event::Start(
          Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. },
        ) => Some(normalize_destination(&dest_url)),
        _ => None,
      })
      .collect::<HashSet<String>>();

    Ok(
      destinations
        .into_iter()
        .filter_map(|dest| match self.check_destination(&dest, path, client) {
          CheckResult::Issue(issue) => Some(issue),
          _ => None,
        })
        .collect(),
    )
  }

  fn classify_destination(
    &self,
    destination: &str,
    file_path: &Path,
  ) -> Target {
    if destination.is_empty() {
      return Target::Invalid("empty destination".to_string());
    }

    if destination.starts_with('#') {
      return Target::Fragment;
    }

    if has_skipped_scheme(destination) {
      return Target::Skipped;
    }

    let candidate = if destination.starts_with("//") {
      format!("https:{destination}")
    } else {
      destination.to_string()
    };

    match Url::parse(&candidate) {
      Ok(url) => match url.scheme() {
        "http" | "https" => Target::Remote(url),
        "file" => match url.to_file_path() {
          Ok(path) => Target::Local(path),
          Err(()) => {
            Target::Invalid(format!("invalid file url: {destination}"))
          }
        },
        _ => Target::Skipped,
      },
      Err(url::ParseError::RelativeUrlWithoutBase) => {
        self.resolve_local_path(destination, file_path)
      }
      Err(error) => {
        Target::Invalid(format!("invalid url ({error}): {destination}"))
      }
    }
  }

  fn collect_files(&self) -> Result<Vec<PathBuf>> {
    let is_markdown = |path: &Path| path.extension() == Some(OsStr::new("md"));

    let files = if self.recursive {
      WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
          entry.file_type().is_file() && is_markdown(entry.path())
        })
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<_>>()
    } else {
      if self.files.is_empty() {
        bail!("no markdown files provided");
      }

      self
        .files
        .iter()
        .map(|path| {
          ensure!(path.exists(), "file not found: {}", path.display());
          Ok(path.clone())
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|path| path.is_file() && is_markdown(path))
        .collect()
    };

    if files.is_empty() {
      bail!("no markdown files to check");
    }

    Ok(files)
  }

  fn resolve_local_path(&self, destination: &str, file_path: &Path) -> Target {
    let trimmed = strip_fragment_and_query(destination).trim();

    if trimmed.is_empty() {
      return Target::Fragment;
    }

    let base = if trimmed.starts_with('/') {
      self.root.clone()
    } else {
      file_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
    };

    let trimmed = trimmed.trim_start_matches('/');

    Target::Local(base.join(trimmed))
  }

  pub(crate) fn run(self) -> Result<usize> {
    ensure!(
      self.root.is_dir(),
      "root directory does not exist: {}",
      self.root.display()
    );

    let mut client = Client::new(self.timeout_seconds)?;

    let issues = self
      .collect_files()?
      .into_iter()
      .map(|path| self.check_file(&path, &mut client))
      .collect::<Result<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<Issue>>();

    for issue in &issues {
      println!("{issue}");
    }

    Ok(issues.len())
  }
}
