use super::*;

pub(crate) struct Loader {
  posts: PathBuf,
  projects: PathBuf,
}

impl Loader {
  pub(crate) fn markdown(input: &str) -> Result<String> {
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

  pub(crate) fn pandoc<I, S>(args: I, input: &str) -> Result<String>
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

  pub(crate) fn posts(&self) -> Result<Vec<Post>> {
    let mut posts = fs::read_dir(&self.posts)?
      .map(|entry| -> Result<Option<Post>> {
        let path = entry?.path();

        if path.extension().is_some_and(|extension| extension == "md") {
          Ok(Some(Post::load(path.as_path())?))
        } else {
          Ok(None)
        }
      })
      .collect::<Result<Vec<Option<Post>>>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<Post>>();

    posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.path.cmp(&b.path)));

    Ok(posts)
  }

  pub(crate) fn projects(&self) -> Result<Vec<Project>> {
    let mut projects = fs::read_dir(&self.projects)?
      .map(|entry| -> Result<Option<Project>> {
        let path = entry?.path();

        if path.extension().is_some_and(|extension| extension == "md") {
          Ok(Some(Project::load(path.as_path())?))
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
