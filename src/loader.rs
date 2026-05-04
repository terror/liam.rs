use super::*;

pub(crate) struct Loader {
  converter: Converter,
  posts: PathBuf,
  projects: PathBuf,
}

impl Loader {
  pub(crate) fn new() -> Self {
    Self {
      converter: Converter::new(),
      posts: PathBuf::from("posts"),
      projects: PathBuf::from("projects"),
    }
  }

  pub(crate) fn posts(&self) -> Result<Vec<Post>> {
    let paths = fs::read_dir(&self.posts)?
      .map(|entry| Ok(entry?.path()))
      .collect::<Result<Vec<PathBuf>>>()?;

    let mut posts = paths
      .into_par_iter()
      .map(|path| Post::load(&self.converter, &path))
      .collect::<Result<Vec<Post>>>()?;

    posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.path.cmp(&b.path)));

    Ok(posts)
  }

  pub(crate) fn projects(&self) -> Result<Vec<Project>> {
    let paths = fs::read_dir(&self.projects)?
      .map(|entry| Ok(entry?.path()))
      .collect::<Result<Vec<PathBuf>>>()?;

    let mut projects = paths
      .into_par_iter()
      .map(|path| Project::load(&self.converter, &path))
      .collect::<Result<Vec<Project>>>()?;

    projects
      .sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));

    Ok(projects)
  }
}
