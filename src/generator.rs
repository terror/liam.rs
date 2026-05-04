use super::*;

pub(crate) struct Generator {
  loader: Loader,
}

impl Generator {
  pub(crate) fn new() -> Self {
    Self {
      loader: Loader::new(),
    }
  }

  pub(crate) fn run(&self) -> Result {
    let environment = Environment::new()?;

    let (posts, projects) = (self.loader.posts()?, self.loader.projects()?);

    fs::remove_dir_all("docs/posts").or_else(|error| {
      if error.kind() == std::io::ErrorKind::NotFound {
        Ok(())
      } else {
        Err(error)
      }
    })?;

    fs::create_dir_all("docs/posts")?;

    for post in &posts {
      environment.write_post(post)?;
    }

    environment.write(
      "docs/posts/index.html",
      "posts.html",
      context! { posts => posts.clone() },
    )?;

    environment.write(
      "docs/index.html",
      "index.html",
      context! {
        posts => posts.iter().take(3).cloned().collect::<Vec<Post>>(),
        projects => projects.iter().take(3).cloned().collect::<Vec<_>>(),
      },
    )?;

    environment.write(
      "docs/projects/index.html",
      "projects.html",
      context! { projects => projects },
    )?;

    environment.write(
      "docs/index.xml",
      "rss.xml",
      context! { posts => posts },
    )?;

    Ok(())
  }
}
