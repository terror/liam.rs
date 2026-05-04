use super::*;

pub(crate) struct Loader {
  posts: PathBuf,
  projects: PathBuf,
}

impl Loader {
  fn markdown(input: &str) -> Result<String> {
    Self::pandoc(
      [
        "--mathjax",
        "--quiet",
        "-t",
        "html",
        "--syntax-highlighting",
        "monochrome",
      ],
      input,
    )
  }

  pub(crate) fn new() -> Self {
    Self {
      posts: PathBuf::from("posts"),
      projects: PathBuf::from("projects"),
    }
  }

  fn pandoc<I, S>(args: I, input: &str) -> Result<String>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut child = Command::new("pandoc")
      .args(args)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .context("failed to spawn pandoc")?;

    child
      .stdin
      .take()
      .context("failed to open pandoc stdin")?
      .write_all(input.as_bytes())?;

    let output = child.wait_with_output()?;

    ensure!(
      output.status.success(),
      "pandoc failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    Ok(String::from_utf8(output.stdout)?)
  }

  fn post(path: &Path) -> Result<Post> {
    let input = fs::read_to_string(path)
      .with_context(|| format!("failed to read post `{}`", path.display()))?;

    let metadata = fs::metadata(path)?;

    let modified = metadata.modified()?;

    let filename = path
      .file_name()
      .context("post path has no filename")?
      .to_string_lossy()
      .to_string();

    Post::new(
      filename,
      Self::markdown(&input)?,
      &input,
      modified,
      path.to_path_buf(),
      html_escape::encode_text(&Self::pandoc(
        ["--quiet", "-t", "html"],
        &input,
      )?)
      .trim_end()
      .to_string(),
    )
  }

  pub(crate) fn posts(&self) -> Result<Vec<Post>> {
    let mut posts = fs::read_dir(&self.posts)?
      .map(|entry| -> Result<Option<Post>> {
        let path = entry?.path();

        if path.extension().is_some_and(|extension| extension == "md") {
          Ok(Some(Self::post(path.as_path())?))
        } else {
          Ok(None)
        }
      })
      .collect::<Result<Vec<Option<Post>>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<Post>>();

    posts.sort_by(|a, b| {
      b.modified
        .cmp(&a.modified)
        .then_with(|| a.path.cmp(&b.path))
    });

    Ok(posts)
  }

  fn project(path: &Path) -> Result<Project> {
    let input = fs::read_to_string(path).with_context(|| {
      format!("failed to read project `{}`", path.display())
    })?;

    let front_matter = FrontMatter::<ProjectFrontMatter>::parse(&input)?;

    Project::new(front_matter.metadata, Self::markdown(front_matter.content)?)
  }

  pub(crate) fn projects(&self) -> Result<Vec<Project>> {
    let mut projects = fs::read_dir(&self.projects)?
      .map(|entry| -> Result<Option<Project>> {
        let path = entry?.path();

        if path.extension().is_some_and(|extension| extension == "md") {
          Ok(Some(Self::project(path.as_path())?))
        } else {
          Ok(None)
        }
      })
      .collect::<Result<Vec<Option<Project>>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<Project>>();

    projects
      .sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));

    Ok(projects)
  }
}
